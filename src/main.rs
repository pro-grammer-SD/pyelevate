mod app;
mod models;
mod parser;
mod pypi;
mod styles;
mod ui;
mod upgrade;
mod security;
mod changelog;
mod popularity;
mod resolver;
mod simulator;
mod panels;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::path::Path;
use tracing::info;

use app::App;
use parser::parse_requirements;
use ui::draw;
use upgrade::UpgradeManager;

#[derive(Parser)]
#[command(name = "PyElevate")]
#[command(about = "Professional-grade Python dependency manager with AI-powered insights", long_about = None)]
#[command(version = "0.2.0")]
#[command(author = "Soumalya Das")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    requirements: Option<String>,

    #[arg(short, long)]
    dry_run: bool,

    #[arg(short, long)]
    lock: bool,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(short, long)]
        requirements: Option<String>,
    },
    Upgrade {
        #[arg(short, long)]
        requirements: Option<String>,

        #[arg(short, long)]
        dry_run: bool,

        #[arg(short, long)]
        lock: bool,
    },
    Simulate {
        #[arg(short, long)]
        requirements: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let requirements_path = determine_requirements_path(cli.requirements.as_deref())?;
    
    info!("🚀 PyElevate v0.2.0 - Starting with {}", requirements_path);

    match cli.command {
        Some(Commands::Check { requirements }) => {
            let path = requirements.as_deref().unwrap_or(&requirements_path);
            check_command(path).await?;
        }
        Some(Commands::Upgrade { requirements, dry_run, lock }) => {
            let path = requirements.as_deref().unwrap_or(&requirements_path);
            upgrade_command(path, dry_run, lock).await?;
        }
        Some(Commands::Simulate { requirements }) => {
            let path = requirements.as_deref().unwrap_or(&requirements_path);
            simulate_command(path).await?;
        }
        None => {
            run_interactive_tui(&requirements_path, cli.dry_run).await?;
        }
    }

    Ok(())
}

fn determine_requirements_path(provided: Option<&str>) -> Result<String> {
    if let Some(path) = provided {
        return Ok(path.to_string());
    }

    let default_path = "requirements.txt";
    if Path::new(default_path).exists() {
        return Ok(default_path.to_string());
    }

    Err(anyhow::anyhow!(
        "Could not find requirements.txt. Please specify with --requirements <path>"
    ))
}

async fn check_command(requirements_path: &str) -> Result<()> {
    let req_file = parse_requirements(requirements_path)?;
    let mut app = App::new(requirements_path.to_string());
    app.set_packages(req_file.packages);

    info!("Fetching latest versions from PyPI...");
    app.pypi_client.update_packages(&mut app.packages).await;

    println!("╔══════════════════════════════════════════════╗");
    println!("║  PyElevate v0.2.0 - Dependency Check Report │");
    println!("╚══════════════════════════════════════════════╝\n");
    println!("📦 Total packages:          {}", app.stats.total);
    println!("🟢 Patch updates:          {}", app.stats.patch_available);
    println!("🟡 Minor updates:          {}", app.stats.minor_available);
    println!("🔴 Major updates:          {}", app.stats.major_available);
    println!("✅ Up to date:             {}", app.stats.up_to_date);
    println!("⚠️  Vulnerable:            {}", app.stats.vulnerable);
    println!("❌ Errors:                 {}\n", app.stats.errors);

    println!("{:<30} {:<15} {:<15} {:<15}", "Package", "Current", "Latest", "Status");
    println!("{}", "─".repeat(75));

    for pkg in &app.packages {
        let latest = pkg.latest_version.as_ref().map(|v| v.as_str()).unwrap_or("N/A");
        let status = pkg.status.as_str();
        println!(
            "{:<30} {:<15} {:<15} {:<15}",
            &pkg.name[..pkg.name.len().min(30)],
            pkg.current_version,
            latest,
            status
        );
    }

    Ok(())
}

async fn upgrade_command(requirements_path: &str, dry_run: bool, lock: bool) -> Result<()> {
    let req_file = parse_requirements(requirements_path)?;
    let mut packages = req_file.packages;
    let pypi_client = pypi::PyPIClient::new();

    println!("╔════════════════════════════════════════════╗");
    println!("║  PyElevate v0.2.0 - Dependency Upgrade    │");
    println!("╚════════════════════════════════════════════╝\n");
    println!("Fetching latest versions from PyPI...");
    
    pypi_client.update_packages(&mut packages).await;

    let upgradable: Vec<_> = packages
        .iter()
        .filter(|p| p.latest_version.is_some())
        .collect();

    println!("\n📋 Available upgrades: {}\n", upgradable.len());
    for pkg in &upgradable {
        let latest = pkg.latest_version.as_ref().unwrap();
        println!(
            "  {} {} → {} ({})",
            pkg.name,
            pkg.current_version,
            latest,
            pkg.status.as_str()
        );
    }

    if dry_run {
        println!("\n🔍 Dry-run mode: No files will be modified");
    } else if !upgradable.is_empty() {
        let backup_path = UpgradeManager::create_backup(requirements_path)?;
        println!("\n💾 Backup created: {}", backup_path);

        let new_content = UpgradeManager::generate_upgraded_content(
            &packages,
            &std::fs::read_to_string(requirements_path)?,
            false,
        )?;

        UpgradeManager::write_requirements(requirements_path, &new_content)?;
        println!("✅ Updated: {}", requirements_path);

        if lock {
            let lock_path = UpgradeManager::write_lock_file(requirements_path, &packages)?;
            println!("🔒 Lock file: {}", lock_path);
        }
    }

    Ok(())
}

async fn simulate_command(requirements_path: &str) -> Result<()> {
    let req_file = parse_requirements(requirements_path)?;
    let mut packages = req_file.packages;
    let pypi_client = pypi::PyPIClient::new();

    pypi_client.update_packages(&mut packages).await;

    let simulator = simulator::UpgradeSimulator::new();
    println!("{}", simulator.generate_report(&packages));

    Ok(())
}

async fn run_interactive_tui(requirements_path: &str, dry_run: bool) -> Result<()> {
    let req_file = parse_requirements(requirements_path)?;
    let mut app = App::new(requirements_path.to_string());
    app.dry_run = dry_run;
    app.set_packages(req_file.packages);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_rate = std::time::Duration::from_millis(250);
    let mut last_tick = std::time::Instant::now();

    app.loading_message = "Fetching package intelligence from PyPI...".to_string();
    terminal.draw(|f| draw(f, app))?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    app.pypi_client.update_packages(&mut app.packages).await;
    
    for pkg in &mut app.packages {
        let _ = app.security_checker.check_package(pkg).await;
    }

    app.apply_sort();
    app.mode = app::AppMode::Display;

    loop {
        terminal.draw(|f| draw(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                handle_input(app, key).await?;

                if app.mode == app::AppMode::Done {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }

    Ok(())
}

async fn handle_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.mode {
        app::AppMode::Display => handle_display_mode(app, key).await?,
        app::AppMode::Search => handle_search_mode(app, key).await?,
        app::AppMode::Confirm => handle_confirm_mode(app, key).await?,
        app::AppMode::GraphView => handle_graph_mode(app, key).await?,
        app::AppMode::ChangelogView => handle_changelog_mode(app, key).await?,
        app::AppMode::Upgrading => {}
        app::AppMode::Loading => {}
        app::AppMode::Done => {
            app.mode = app::AppMode::Done;
            std::process::exit(0);
        }
    }
    Ok(())
}

async fn handle_display_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
            std::process::exit(0);
        }
        (KeyCode::Up, _) => {
            app.move_up();
        }
        (KeyCode::Down, _) => {
            app.move_down();
        }
        (KeyCode::PageUp, _) => {
            app.page_up();
        }
        (KeyCode::PageDown, _) => {
            app.page_down();
        }
        (KeyCode::Home, _) => {
            app.home();
        }
        (KeyCode::End, _) => {
            app.end();
        }
        (KeyCode::Char(' '), _) => {
            app.toggle_selected();
        }
        (KeyCode::Char('a') | KeyCode::Char('A'), _) => {
            app.select_all();
        }
        (KeyCode::Char('d') | KeyCode::Char('D'), _) => {
            app.deselect_all();
        }
        (KeyCode::Char('m') | KeyCode::Char('M'), _) => {
            app.select_all_major();
        }
        (KeyCode::Char('i') | KeyCode::Char('I'), _) => {
            app.select_all_minor();
        }
        (KeyCode::Char('p') | KeyCode::Char('P'), _) => {
            app.select_all_patch();
        }
        (KeyCode::Char('/'), _) => {
            app.mode = app::AppMode::Search;
            app.search_query.clear();
        }
        (KeyCode::Char('g') | KeyCode::Char('G'), _) => {
            app.mode = app::AppMode::GraphView;
        }
        (KeyCode::Char('c') | KeyCode::Char('C'), _) => {
            app.mode = app::AppMode::ChangelogView;
        }
        (KeyCode::Char('s') | KeyCode::Char('S'), _) => {
            app.sort_by = match app.sort_by {
                app::SortBy::Name => app::SortBy::Status,
                app::SortBy::Status => app::SortBy::Current,
                app::SortBy::Current => app::SortBy::Latest,
                app::SortBy::Latest => app::SortBy::Popularity,
                app::SortBy::Popularity => app::SortBy::Name,
            };
            app.apply_sort();
        }
        (KeyCode::Char('u') | KeyCode::Char('U'), _) => {
            if app.count_selected() > 0 {
                app.mode = app::AppMode::Confirm;
            } else if app.has_upgradable_packages() {
                app.set_error("Select packages first (Space to select)".to_string());
            }
        }
        (KeyCode::Char(c), _) => {
            if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '-' || c == '_' {
                app.search_query.clear();
                app.search_query.push(c);
                app.mode = app::AppMode::Search;
                app.refresh_filtered_packages();
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_search_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.search_query.clear();
            app.refresh_filtered_packages();
            app.mode = app::AppMode::Display;
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.refresh_filtered_packages();
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.refresh_filtered_packages();
        }
        KeyCode::Up => {
            app.move_up();
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Char(' ') => {
            app.toggle_selected();
        }
        _ => {}
    }
    Ok(())
}

async fn handle_confirm_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            app.mode = app::AppMode::Upgrading;

            let content = std::fs::read_to_string(&app.requirements_path)?;
            let backup_path = if !app.dry_run {
                Some(UpgradeManager::create_backup(&app.requirements_path)?)
            } else {
                None
            };

            let new_content = UpgradeManager::generate_upgraded_content(
                &app.packages,
                &content,
                true,
            )?;

            if !app.dry_run && !new_content.is_empty() {
                UpgradeManager::write_requirements(&app.requirements_path, &new_content)?;
            }

            let upgrade_count = app.count_selected();
            app.success_message = Some(format!(
                "✅ Successfully upgraded {} package(s){}",
                upgrade_count,
                if let Some(backup) = &backup_path {
                    format!("\n📦 Backup: {}", backup)
                } else {
                    String::new()
                }
            ));
            app.backup_path = backup_path;

            app.mode = app::AppMode::Done;
        }
        KeyCode::Esc => {
            app.mode = app::AppMode::Display;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_graph_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('g') | KeyCode::Char('G') | KeyCode::Esc => {
            app.mode = app::AppMode::Display;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_changelog_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Esc => {
            app.mode = app::AppMode::Display;
        }
        _ => {}
    }
    Ok(())
}

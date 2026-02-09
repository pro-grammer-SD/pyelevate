use crate::app::{App, AppMode};
use crate::panels;
use crate::styles::{Styles, Theme};
use crate::simulator::UpgradeSimulator;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let theme = Theme::default_theme();
    let styles = Styles::new(&theme);

    match app.mode {
        AppMode::Loading => draw_loading(f, app, &styles),
        AppMode::Display => draw_main_multi_panel(f, app, &styles, &theme),
        AppMode::Search => draw_search_mode(f, app, &styles),
        AppMode::Confirm => draw_confirm(f, app, &styles),
        AppMode::Upgrading => draw_upgrading(f, &styles),
        AppMode::Done => draw_done(f, app, &styles),
        AppMode::GraphView => draw_graph_view(f, app, &styles),
        AppMode::ChangelogView => draw_changelog_detail(f, app, &styles),
    }
}

fn draw_loading(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
        .split(size);

    let title = Paragraph::new("🚀 PyElevate v0.2.0 - God Tier Dev Tool")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let loading_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            format!("⟳ {}", app.loading_message),
            styles.header,
        )]),
        Line::from(""),
    ];

    let message = Paragraph::new(loading_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(styles.border));
    f.render_widget(message, chunks[1]);
}

fn draw_main_multi_panel(f: &mut Frame, app: &App, styles: &Styles, _theme: &Theme) {
    let size = f.size();
    
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(15),
            Constraint::Length(3),
        ])
        .split(size);

    draw_header(f, outer_chunks[0], styles);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(outer_chunks[1]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(30), Constraint::Percentage(30)])
        .split(main_chunks[1]);

    panels::render_dependency_list(
        f,
        main_chunks[0],
        &app.packages,
        app.selected_index,
    );

    let selected = app.get_selected_package_ref();
    panels::render_info_panel(f, right_chunks[0], selected);
    panels::render_popularity_panel(f, right_chunks[1], selected.and_then(|p| p.popularity.as_ref()));
    panels::render_changelog_panel(f, right_chunks[2], selected.and_then(|p| p.changelog.as_ref()));

    draw_help_bar(f, outer_chunks[2], styles);
}

fn draw_header(f: &mut Frame, area: Rect, styles: &Styles) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(30), Constraint::Percentage(30)])
        .split(area);

    let title = Paragraph::new("🚀 PyElevate v0.2.0")
        .style(styles.title)
        .alignment(Alignment::Left);
    f.render_widget(title, chunks[0]);

    let stats = Paragraph::new(format!(
        "📦 {} | 🔴 {} | 🔶 {} | 🟢 {} | ⚠️  {}",
        "total",
        "major",
        "minor",
        "patch",
        "vulnerable"
    ))
    .style(styles.help)
    .alignment(Alignment::Center);
    f.render_widget(stats, chunks[1]);

    let version = Paragraph::new("Interactive Python Dependency Manager")
        .style(styles.help)
        .alignment(Alignment::Right);
    f.render_widget(version, chunks[2]);
}

fn draw_help_bar(f: &mut Frame, area: Rect, styles: &Styles) {
    let help_text = "↑↓: Navigate | Tab: Switch Panel | Space: Select | U: Upgrade | G: Graph | C: Changelog | F: Filter | Ctrl+C: Quit";

    let help = Paragraph::new(help_text)
        .style(styles.help)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).style(styles.border));

    f.render_widget(help, area);
}

fn draw_search_mode(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    draw_header(f, chunks[0], styles);

    let search_box = Paragraph::new(format!("🔍 Search: {}_", app.search_query))
        .style(styles.header)
        .block(Block::default().borders(Borders::ALL).style(styles.border));
    f.render_widget(search_box, chunks[1]);

    let filtered: Vec<_> = app.packages
        .iter()
        .filter(|p| p.name.contains(&app.search_query))
        .collect();

    let mut lines = Vec::new();
    for (idx, pkg) in filtered.iter().enumerate() {
        let style = if idx == app.selected_index {
            ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray)
        } else {
            ratatui::style::Style::default()
        };

        let latest = pkg.latest_version.as_ref().map(|v| v.as_str()).unwrap_or("N/A");
        lines.push(Line::from(vec![
            Span::styled(format!("{:<25} ", &pkg.name[..pkg.name.len().min(25)]), style),
            Span::raw(format!("{:<8} → {:<8} ", pkg.current_version, latest)),
            Span::styled(pkg.status.as_str(), style),
        ]));
    }

    let results = Paragraph::new(lines)
        .block(Block::default().title(" Results ").borders(Borders::ALL));
    f.render_widget(results, chunks[2]);

    let help = Paragraph::new("Type to search | ↑↓: Navigate | Space: Select | Esc: Back | Enter: Upgrade")
        .style(styles.help)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).style(styles.border));
    f.render_widget(help, chunks[3]);
}

fn draw_confirm(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let simulator = UpgradeSimulator::new();
    let simulation = simulator.simulate_upgrade(&app.packages);

    let dialog_width = size.width.saturating_sub(4).min(80);
    let dialog_height = 20usize;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((size.height.saturating_sub(dialog_height as u16)) / 2),
            Constraint::Length(dialog_height as u16),
            Constraint::Min(1),
        ])
        .split(size);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((size.width.saturating_sub(dialog_width as u16)) / 2),
            Constraint::Length(dialog_width as u16),
            Constraint::Min(1),
        ])
        .split(popup_layout[1])[1];

    let content = vec![
        Line::from(""),
        Line::from("📋 UPGRADE SIMULATION REPORT"),
        Line::from(""),
        Line::from(format!("📦 Packages to upgrade:  {}", simulation.packages_to_upgrade)),
        Line::from(format!("🔴 Major changes:       {}", simulation.major_changes)),
        Line::from(format!("⚠️  Conflicts:          {}", simulation.conflicts_detected)),
        Line::from(format!("🔒 Security fixes:      {}", simulation.security_fixes)),
        Line::from(format!("📊 Risk level:          {}", simulation.risk_level.as_str())),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", styles.header),
            Span::raw(": Confirm  |  "),
            Span::styled("Esc", styles.header),
            Span::raw(": Cancel"),
        ]),
        Line::from(""),
    ];

    let dialog = Paragraph::new(content)
        .block(
            Block::default()
                .title(" Confirm Upgrade ")
                .borders(Borders::ALL)
                .style(styles.header),
        )
        .alignment(Alignment::Left);

    f.render_widget(
        Block::default()
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black)),
        size,
    );
    f.render_widget(dialog, popup_area);
}

fn draw_upgrading(f: &mut Frame, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(size);

    let title = Paragraph::new("🚀 PyElevate v0.2.0")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let message = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "⟳ Processing upgrades...",
            styles.header,
        )]),
        Line::from(""),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).style(styles.border));
    f.render_widget(message, chunks[1]);
}

fn draw_done(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("🎉 PyElevate v0.2.0")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let message_lines = if let Some(success) = &app.success_message {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✅ Upgrade completed successfully!",
                styles.patch,
            )]),
            Line::from(""),
            Line::from(success.as_str()),
            Line::from(""),
            if let Some(backup) = &app.backup_path {
                Line::from(format!("📦 Backup: {}", backup))
            } else {
                Line::from("")
            },
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✓ No upgrades selected",
                styles.help,
            )]),
            Line::from(""),
        ]
    };

    let message = Paragraph::new(message_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(styles.border));
    f.render_widget(message, chunks[1]);

    let help = Paragraph::new("Press any key to exit")
        .style(styles.help)
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_graph_view(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("📊 Dependency Graph")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let mut lines = vec![Line::from("")];
    for pkg in app.packages.iter().take(20) {
        lines.push(Line::from(format!("📦 {}", pkg.name)));
    }

    let graph = Paragraph::new(lines)
        .block(Block::default().title(" Dependencies ").borders(Borders::ALL));
    f.render_widget(graph, chunks[1]);

    let help = Paragraph::new("G: Back to main | Esc: Quit")
        .style(styles.help)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).style(styles.border));
    f.render_widget(help, chunks[2]);
}

fn draw_changelog_detail(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("📝 Changelog")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let mut lines = vec![Line::from("")];
    if let Some(pkg) = app.get_selected_package_ref() {
        if let Some(changelog) = &pkg.changelog {
            lines.push(Line::from(vec![
                Span::styled("Version: ", ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw(&changelog.version),
            ]));
            lines.push(Line::from(""));
            for change in &changelog.changes {
                lines.push(Line::from(format!("• {}", change)));
            }
        }
    }

    let changelog = Paragraph::new(lines)
        .block(Block::default().title(" Changelog ").borders(Borders::ALL));
    f.render_widget(changelog, chunks[1]);

    let help = Paragraph::new("C: Back to main | Esc: Quit")
        .style(styles.help)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).style(styles.border));
    f.render_widget(help, chunks[2]);
}

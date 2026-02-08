use crate::app::{App, AppMode};
use crate::styles::{Styles, Theme, status_symbol};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let theme = Theme::default_theme();
    let styles = Styles::new(&theme);

    match app.mode {
        AppMode::Loading => draw_loading(f, app, &styles),
        AppMode::Display => draw_main(f, app, &styles, &theme),
        AppMode::Search => draw_search_mode(f, app, &styles, &theme),
        AppMode::Confirm => draw_confirm(f, app, &styles, &theme),
        AppMode::Upgrading => draw_upgrading(f, app, &styles),
        AppMode::Done => draw_done(f, app, &styles),
    }
}

fn draw_loading(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("PyElevate")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let loading_text = vec![
        Line::from(vec![Span::raw("  ")]),
        Line::from(vec![Span::styled(
            format!("⟳ {}", app.loading_message),
            styles.header,
        )]),
        Line::from(vec![Span::raw("  ")]),
    ];

    let message = Paragraph::new(loading_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(styles.border));
    f.render_widget(message, chunks[1]);
}

fn draw_main(f: &mut Frame, app: &App, styles: &Styles, theme: &Theme) {
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
    draw_stats_bar(f, chunks[1], app, styles);
    draw_package_table(f, chunks[2], app, styles, theme);
    draw_help_bar(f, chunks[3], app, styles);
}

fn draw_header(f: &mut Frame, area: Rect, styles: &Styles) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let title = Paragraph::new("PyElevate")
        .style(styles.title)
        .alignment(Alignment::Left);
    f.render_widget(title, chunks[0]);

    let version_info = Paragraph::new("v0.1.0 - Interactive Python Dependency Upgrader")
        .style(styles.help)
        .alignment(Alignment::Right);
    f.render_widget(version_info, chunks[1]);
}

fn draw_stats_bar(f: &mut Frame, area: Rect, app: &App, styles: &Styles) {
    let stats_text = vec![
        Span::raw("Total: "),
        Span::styled(
            format!("{}", app.stats.total),
            styles.header,
        ),
        Span::raw("  |  "),
        Span::raw("Updates: "),
        Span::styled(
            format!("{}", app.stats.total_upgradable()),
            styles.header,
        ),
        Span::raw("  |  "),
        Span::styled("◆", styles.patch),
        Span::raw(": "),
        Span::raw(format!("{}", app.stats.patch_available)),
        Span::raw("  "),
        Span::styled("◈", styles.minor),
        Span::raw(": "),
        Span::raw(format!("{}", app.stats.minor_available)),
        Span::raw("  "),
        Span::styled("✕", styles.major),
        Span::raw(": "),
        Span::raw(format!("{}", app.stats.major_available)),
    ];

    let stats = Paragraph::new(Line::from(stats_text))
        .style(styles.normal)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(styles.border),
        );

    f.render_widget(stats, area);
}

fn draw_package_table(f: &mut Frame, area: Rect, app: &App, styles: &Styles, theme: &Theme) {
    let header = Row::new(vec!["Sel", "Package", "Current", "Latest", "Status"])
        .style(styles.header)
        .bottom_margin(1);

    let rows = app.filtered_packages.iter().enumerate().map(|(display_idx, &pkg_idx)| {
        let pkg = &app.packages[pkg_idx];
        let selected_marker = if pkg.selected { "✓" } else { " " };
        let status_str = pkg.status.as_str();
        let symbol = status_symbol(pkg.status);
        let status_display = format!("{} {}", symbol, status_str);

        let latest_str = pkg.latest_version.as_ref()
            .map(|v| v.as_str())
            .unwrap_or("N/A");

        let row_style = if display_idx == app.selected_index {
            styles.selected
        } else if pkg.error.is_some() {
            styles.error
        } else {
            styles.normal
        };

        let status_style = if display_idx == app.selected_index {
            row_style
        } else {
            theme.status_style(pkg.status)
        };

        Row::new(vec![
            selected_marker.to_string(),
            pkg.name.clone(),
            pkg.current_version.clone(),
            latest_str.to_string(),
            status_display,
        ])
        .style(row_style)
        .style(status_style)
    });

    let table = Table::new(rows, [
        Constraint::Length(3),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(30),
    ])
    .header(header)
    .block(
        Block::default()
            .title(" Packages ")
            .borders(Borders::ALL)
            .style(styles.border),
    )
    .highlight_style(styles.selected);

    f.render_widget(table, area);
}

fn draw_help_bar(f: &mut Frame, area: Rect, app: &App, styles: &Styles) {
    let help_text = match app.mode {
        AppMode::Display => {
            "↑↓: Navigate  Space: Select  A: Select All  M: Major  i: Minor  P: Patch  /: Search  Enter: Upgrade  Ctrl+C: Quit"
        }
        AppMode::Search => {
            "Type to search, Esc: Back to table"
        }
        _ => "↑↓: Navigate  Space: Select  Enter: Confirm  Esc: Cancel"
    };

    let help = Paragraph::new(help_text)
        .style(styles.help)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::TOP).style(styles.border));

    f.render_widget(help, area);
}

fn draw_search_mode(f: &mut Frame, app: &App, styles: &Styles, theme: &Theme) {
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

    let search_box = Paragraph::new(format!("Search: {}_", app.search_query))
        .style(styles.header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(styles.border),
        );
    f.render_widget(search_box, chunks[1]);

    draw_package_table(f, chunks[2], app, styles, theme);
    
    let help = Paragraph::new("Type to search | Esc: Back to table")
        .style(styles.help)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).style(styles.border));
    f.render_widget(help, chunks[3]);
}

fn draw_confirm(f: &mut Frame, app: &App, styles: &Styles, _theme: &Theme) {
    let size = f.size();
    let dialog_width = size.width.saturating_sub(4).min(60);
    let dialog_height = 15usize;

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

    let selected_count = app.count_selected();
    let content = if app.dry_run {
        vec![
            Line::from(""),
            Line::from("Dry-run mode: No files will be modified"),
            Line::from(""),
            Line::from(format!("Will upgrade {} package(s)", selected_count)),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", styles.header),
                Span::raw(": Confirm"),
                Span::raw("  "),
                Span::styled("Esc", styles.header),
                Span::raw(": Cancel"),
            ]),
            Line::from(""),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from("Confirm upgrades?"),
            Line::from(""),
            Line::from(format!("Will upgrade {} package(s)", selected_count)),
            Line::from("Original file will be backed up"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", styles.header),
                Span::raw(": Confirm"),
                Span::raw("  "),
                Span::styled("Esc", styles.header),
                Span::raw(": Cancel"),
            ]),
            Line::from(""),
        ]
    };

    let dialog = Paragraph::new(content)
        .block(
            Block::default()
                .title(" Confirm ")
                .borders(Borders::ALL)
                .style(styles.header),
        )
        .alignment(Alignment::Center);

    f.render_widget(
        Block::default()
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black)),
        size,
    );
    f.render_widget(dialog, popup_area);
}

fn draw_upgrading(f: &mut Frame, app: &App, styles: &Styles) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(size);

    let title = Paragraph::new("PyElevate")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let message = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "⟳ Upgrading dependencies...",
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

    let title = Paragraph::new("PyElevate")
        .style(styles.title)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let message_lines = if let Some(success) = &app.success_message {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✓ Upgrade completed successfully!",
                styles.patch,
            )]),
            Line::from(""),
            Line::from(success.as_str()),
            Line::from(""),
            if let Some(backup) = &app.backup_path {
                Line::from(format!("Backup: {}", backup))
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

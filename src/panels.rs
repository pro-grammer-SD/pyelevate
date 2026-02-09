use crate::models::{Package, PopularityData, Changelog};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style, Modifier},
    Frame,
};

pub fn render_dependency_list(
    f: &mut Frame,
    area: Rect,
    packages: &[Package],
    selected_idx: usize,
) {
    let mut lines = vec![
        Line::from(vec![
            Span::styled("NAME", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("CURRENT", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("LATEST", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("STATUS", Style::default().add_modifier(Modifier::BOLD)),
        ])
    ];

    for (idx, pkg) in packages.iter().enumerate() {
        let marker = if idx == selected_idx { "→ " } else { "  " };
        let style = if idx == selected_idx {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        let status_color = match pkg.status {
            crate::models::VersionStatus::Major => Color::Red,
            crate::models::VersionStatus::Minor => Color::Yellow,
            crate::models::VersionStatus::Patch => Color::Green,
            crate::models::VersionStatus::Vulnerable => Color::Magenta,
            _ => Color::Gray,
        };

        let latest = pkg.latest_version.as_ref().map(|v| v.as_str()).unwrap_or("N/A");
        let line = Line::from(vec![
            Span::styled(marker, style),
            Span::raw(format!("{:<20} | ", &pkg.name[..pkg.name.len().min(20)])),
            Span::raw(format!("{:<8} | ", pkg.current_version)),
            Span::styled(format!("{:<8} | ", latest), Style::default().fg(status_color)),
            Span::styled(pkg.status.as_str(), Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        ]);
        lines.push(line);
    }

    let widget = Paragraph::new(lines)
        .block(Block::default().title(" Dependencies ").borders(Borders::ALL))
        .scroll((0, 0));
    f.render_widget(widget, area);
}

pub fn render_info_panel(
    f: &mut Frame,
    area: Rect,
    package: Option<&Package>,
) {
    let content = if let Some(pkg) = package {
        vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&pkg.name),
            ]),
            Line::from(vec![
                Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&pkg.current_version),
            ]),
            Line::from(vec![
                Span::styled("Source: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(pkg.source.source_type()),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    pkg.status.as_str(),
                    Style::default().fg(match pkg.status {
                        crate::models::VersionStatus::Vulnerable => Color::Magenta,
                        crate::models::VersionStatus::Major => Color::Red,
                        crate::models::VersionStatus::Minor => Color::Yellow,
                        _ => Color::Green,
                    }),
                ),
            ]),
            Line::from(""),
            Line::from(pkg.source.description()),
        ]
    } else {
        vec![Line::from("Select a package")]
    };

    let widget = Paragraph::new(content)
        .block(Block::default().title(" Info ").borders(Borders::ALL));
    f.render_widget(widget, area);
}

pub fn render_popularity_panel(
    f: &mut Frame,
    area: Rect,
    popularity: Option<&PopularityData>,
) {
    let content = if let Some(pop) = popularity {
        vec![
            Line::from(vec![
                Span::styled("Weekly: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} downloads", pop.weekly_downloads)),
            ]),
            Line::from(vec![
                Span::styled("Monthly: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} downloads", pop.downloads_last_month)),
            ]),
            Line::from(""),
            Line::from("Recent Trend:"),
        ]
        .into_iter()
        .chain(pop.downloads_trend.iter().take(5).map(|(date, count)| {
            Line::from(format!("  {}: {}", date, count))
        }))
        .collect()
    } else {
        vec![Line::from("No popularity data")]
    };

    let widget = Paragraph::new(content)
        .block(Block::default().title(" Popularity ").borders(Borders::ALL));
    f.render_widget(widget, area);
}

pub fn render_changelog_panel(
    f: &mut Frame,
    area: Rect,
    changelog: Option<&Changelog>,
) {
    let content = if let Some(cl) = changelog {
        vec![
            Line::from(vec![
                Span::styled("Version: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&cl.version),
            ]),
            Line::from(vec![
                Span::styled("Released: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&cl.release_date),
            ]),
            Line::from(vec![
                Span::styled("Risk: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    cl.risk_level(),
                    Style::default().fg(match cl.risk_level() {
                        "HIGH" => Color::Red,
                        "MEDIUM" => Color::Yellow,
                        _ => Color::Green,
                    }),
                ),
            ]),
            Line::from(""),
        ]
        .into_iter()
        .chain(
            if !cl.breaking_changes.is_empty() {
                vec![
                    Line::from(Span::styled(
                        "⚠️  Breaking Changes:",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )),
                ]
                .into_iter()
                .chain(cl.breaking_changes.iter().take(3).map(|c| Line::from(format!("  • {}", c))))
                .collect::<Vec<_>>()
            } else {
                vec![]
            },
        )
        .collect()
    } else {
        vec![Line::from("No changelog available")]
    };

    let widget = Paragraph::new(content)
        .block(Block::default().title(" Changelog ").borders(Borders::ALL));
    f.render_widget(widget, area);
}

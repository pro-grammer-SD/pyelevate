use ratatui::style::{Color, Modifier, Style};
use crate::models::VersionStatus;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub text_muted: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Magenta,
            accent: Color::Yellow,
            background: Color::Black,
            surface: Color::Rgb(20, 20, 30),
            border: Color::Rgb(50, 50, 80),
            text: Color::White,
            text_muted: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
        }
    }

    pub fn status_color(&self, status: VersionStatus) -> Color {
        match status {
            VersionStatus::UpToDate => self.success,
            VersionStatus::Patch => self.warning,
            VersionStatus::Minor => self.info,
            VersionStatus::Major => self.error,
            VersionStatus::Prerelease => self.secondary,
            VersionStatus::Unknown => self.text_muted,
            VersionStatus::Error => Color::Red,
        }
    }

    pub fn status_style(&self, status: VersionStatus) -> Style {
        Style::default()
            .fg(self.status_color(status))
            .add_modifier(Modifier::BOLD)
    }
}

pub struct Styles {
    pub normal: Style,
    pub selected: Style,
    pub header: Style,
    pub title: Style,
    pub border: Style,
    pub help: Style,
    pub patch: Style,
    pub minor: Style,
    pub major: Style,
    pub up_to_date: Style,
    pub error: Style,
}

impl Styles {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Style::default().fg(theme.text),
            selected: Style::default()
                .fg(theme.primary)
                .bg(theme.surface)
                .add_modifier(Modifier::BOLD),
            header: Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            title: Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(theme.border),
            help: Style::default().fg(theme.text_muted),
            patch: Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::BOLD),
            minor: Style::default()
                .fg(theme.info)
                .add_modifier(Modifier::BOLD),
            major: Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
            up_to_date: Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::DIM),
            error: Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        }
    }
}

pub fn status_symbol(status: VersionStatus) -> &'static str {
    match status {
        VersionStatus::UpToDate => "✓",
        VersionStatus::Patch => "◆",
        VersionStatus::Minor => "◈",
        VersionStatus::Major => "✕",
        VersionStatus::Prerelease => "⬆",
        VersionStatus::Unknown => "?",
        VersionStatus::Error => "⚠",
    }
}

pub fn pad_right(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - s.len()))
    }
}

pub fn truncate_string(s: &str, width: usize) -> String {
    if s.len() > width {
        format!("{}...", &s[..width.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

pub fn format_version_cell(version: &str, width: usize) -> String {
    let truncated = truncate_string(version, width.saturating_sub(2));
    pad_right(&truncated, width)
}

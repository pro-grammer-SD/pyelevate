use crate::models::VersionStatus;
use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Magenta,
            background: Color::Black,
            surface: Color::DarkGray,
            border: Color::Cyan,
            text: Color::White,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
        }
    }

    pub fn status_style(&self, status: VersionStatus) -> Style {
        match status {
            VersionStatus::UpToDate => Style::default().fg(self.success),
            VersionStatus::Patch => Style::default().fg(Color::Green),
            VersionStatus::Minor => Style::default().fg(Color::Yellow),
            VersionStatus::Major => Style::default().fg(Color::Red),
            VersionStatus::Vulnerable => Style::default().fg(Color::Magenta),
            VersionStatus::Error => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::Gray),
        }
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
    pub vulnerable: Style,
}

impl Styles {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Style::default().fg(theme.text),
            selected: Style::default()
                .bg(theme.surface)
                .fg(theme.text)
                .add_modifier(Modifier::BOLD),
            header: Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
            title: Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            border: Style::default().fg(theme.border),
            help: Style::default().fg(Color::Gray),
            patch: Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
            minor: Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::BOLD),
            major: Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
            up_to_date: Style::default().fg(theme.success),
            error: Style::default().fg(theme.error),
            vulnerable: Style::default()
                .fg(Color::Magenta)
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
        VersionStatus::Vulnerable => "⚠",
        VersionStatus::Error => "✗",
        VersionStatus::Prerelease => "β",
        VersionStatus::Unknown => "?",
    }
}

pub fn status_color(status: VersionStatus) -> Color {
    match status {
        VersionStatus::UpToDate => Color::Green,
        VersionStatus::Patch => Color::Green,
        VersionStatus::Minor => Color::Yellow,
        VersionStatus::Major => Color::Red,
        VersionStatus::Vulnerable => Color::Magenta,
        VersionStatus::Error => Color::Red,
        VersionStatus::Prerelease => Color::Cyan,
        VersionStatus::Unknown => Color::Gray,
    }
}

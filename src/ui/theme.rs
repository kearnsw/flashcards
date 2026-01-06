//! Theme and styling for the TUI.

use ratatui::style::{Color, Modifier, Style};

/// Color theme for the application.
pub struct Theme;

impl Theme {
    // ══════════════════════════════════════════════════════════════════════
    // Brand Colors
    // ══════════════════════════════════════════════════════════════════════

    pub const PRIMARY: Color = Color::Rgb(99, 102, 241);      // Indigo
    pub const SECONDARY: Color = Color::Rgb(139, 92, 246);    // Violet
    pub const ACCENT: Color = Color::Rgb(236, 72, 153);       // Pink

    // ══════════════════════════════════════════════════════════════════════
    // Semantic Colors
    // ══════════════════════════════════════════════════════════════════════

    pub const SUCCESS: Color = Color::Rgb(34, 197, 94);       // Green
    pub const WARNING: Color = Color::Rgb(250, 204, 21);      // Yellow
    pub const ERROR: Color = Color::Rgb(239, 68, 68);         // Red
    pub const INFO: Color = Color::Rgb(59, 130, 246);         // Blue

    // ══════════════════════════════════════════════════════════════════════
    // Background Colors
    // ══════════════════════════════════════════════════════════════════════

    pub const BG_DARK: Color = Color::Rgb(15, 23, 42);        // Slate 900
    pub const BG_CARD: Color = Color::Rgb(30, 41, 59);        // Slate 800
    pub const BG_ELEVATED: Color = Color::Rgb(51, 65, 85);    // Slate 700
    pub const BG_HIGHLIGHT: Color = Color::Rgb(71, 85, 105);  // Slate 600

    // ══════════════════════════════════════════════════════════════════════
    // Text Colors
    // ══════════════════════════════════════════════════════════════════════

    pub const TEXT: Color = Color::Rgb(248, 250, 252);        // Slate 50
    pub const TEXT_MUTED: Color = Color::Rgb(148, 163, 184);  // Slate 400
    pub const TEXT_DIM: Color = Color::Rgb(100, 116, 139);    // Slate 500

    // ══════════════════════════════════════════════════════════════════════
    // Rating Colors
    // ══════════════════════════════════════════════════════════════════════

    pub const RATING_AGAIN: Color = Color::Rgb(239, 68, 68);  // Red
    pub const RATING_HARD: Color = Color::Rgb(251, 191, 36);  // Amber
    pub const RATING_GOOD: Color = Color::Rgb(59, 130, 246);  // Blue
    pub const RATING_EASY: Color = Color::Rgb(34, 197, 94);   // Green

    // ══════════════════════════════════════════════════════════════════════
    // Styles
    // ══════════════════════════════════════════════════════════════════════

    pub fn title() -> Style {
        Style::default()
            .fg(Self::TEXT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn highlight() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected() -> Style {
        Style::default()
            .bg(Self::BG_HIGHLIGHT)
            .fg(Self::TEXT)
    }

    pub fn card_border() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
    }

    pub fn card_front() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn card_back() -> Style {
        Style::default()
            .fg(Self::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_new() -> Style {
        Style::default()
            .fg(Self::INFO)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_learning() -> Style {
        Style::default()
            .fg(Self::WARNING)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_due() -> Style {
        Style::default()
            .fg(Self::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_hint() -> Style {
        Style::default()
            .fg(Self::TEXT_DIM)
    }

    pub fn key_highlight() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Box Drawing Characters
// ══════════════════════════════════════════════════════════════════════════

pub mod borders {
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const DOUBLE_VERTICAL: &str = "║";

    pub const ROUNDED_TOP_LEFT: &str = "╭";
    pub const ROUNDED_TOP_RIGHT: &str = "╮";
    pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
    pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
}

// ══════════════════════════════════════════════════════════════════════════
// Icons
// ══════════════════════════════════════════════════════════════════════════

pub mod icons {
    pub const CARD: &str = "🃏";
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const STAR: &str = "★";
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_LEFT: &str = "←";
    pub const BRAIN: &str = "🧠";
    pub const FIRE: &str = "🔥";
    pub const SPARKLE: &str = "✨";
    pub const CLOCK: &str = "⏱";
    pub const BOOK: &str = "📚";
}

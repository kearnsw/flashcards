//! Theme and styling for the TUI.

use ratatui::style::{Color, Modifier, Style};

/// Color palette for a theme.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Brand Colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    // Semantic Colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Background Colors
    pub bg_dark: Color,
    pub bg_card: Color,
    pub bg_elevated: Color,
    pub bg_highlight: Color,

    // Text Colors
    pub text: Color,
    pub text_muted: Color,
    pub text_dim: Color,

    // Rating Colors
    pub rating_again: Color,
    pub rating_hard: Color,
    pub rating_good: Color,
    pub rating_easy: Color,
}

/// Available theme names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Default,
    KanagawaWave,
    KanagawaDragon,
    Dracula,
    CatppuccinMocha,
    GruvboxDark,
    Nord,
    TokyoNight,
    RosePine,
    SolarizedDark,
}

impl ThemeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeName::Default => "default",
            ThemeName::KanagawaWave => "kanagawa-wave",
            ThemeName::KanagawaDragon => "kanagawa-dragon",
            ThemeName::Dracula => "dracula",
            ThemeName::CatppuccinMocha => "catppuccin-mocha",
            ThemeName::GruvboxDark => "gruvbox-dark",
            ThemeName::Nord => "nord",
            ThemeName::TokyoNight => "tokyo-night",
            ThemeName::RosePine => "rose-pine",
            ThemeName::SolarizedDark => "solarized-dark",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeName::Default => "Default",
            ThemeName::KanagawaWave => "Kanagawa Wave",
            ThemeName::KanagawaDragon => "Kanagawa Dragon",
            ThemeName::Dracula => "Dracula",
            ThemeName::CatppuccinMocha => "Catppuccin",
            ThemeName::GruvboxDark => "Gruvbox",
            ThemeName::Nord => "Nord",
            ThemeName::TokyoNight => "Tokyo Night",
            ThemeName::RosePine => "Rosé Pine",
            ThemeName::SolarizedDark => "Solarized",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().replace('_', "-").as_str() {
            "kanagawa-wave" | "kanagawa" => ThemeName::KanagawaWave,
            "kanagawa-dragon" => ThemeName::KanagawaDragon,
            "dracula" => ThemeName::Dracula,
            "catppuccin-mocha" | "catppuccin" => ThemeName::CatppuccinMocha,
            "gruvbox-dark" | "gruvbox" => ThemeName::GruvboxDark,
            "nord" => ThemeName::Nord,
            "tokyo-night" | "tokyonight" => ThemeName::TokyoNight,
            "rose-pine" | "rosepine" => ThemeName::RosePine,
            "solarized-dark" | "solarized" => ThemeName::SolarizedDark,
            _ => ThemeName::Default,
        }
    }

    pub fn all() -> &'static [ThemeName] {
        &[
            ThemeName::Default,
            ThemeName::KanagawaWave,
            ThemeName::KanagawaDragon,
            ThemeName::Dracula,
            ThemeName::CatppuccinMocha,
            ThemeName::GruvboxDark,
            ThemeName::Nord,
            ThemeName::TokyoNight,
            ThemeName::RosePine,
            ThemeName::SolarizedDark,
        ]
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeName::Default => ThemeName::KanagawaWave,
            ThemeName::KanagawaWave => ThemeName::KanagawaDragon,
            ThemeName::KanagawaDragon => ThemeName::Dracula,
            ThemeName::Dracula => ThemeName::CatppuccinMocha,
            ThemeName::CatppuccinMocha => ThemeName::GruvboxDark,
            ThemeName::GruvboxDark => ThemeName::Nord,
            ThemeName::Nord => ThemeName::TokyoNight,
            ThemeName::TokyoNight => ThemeName::RosePine,
            ThemeName::RosePine => ThemeName::SolarizedDark,
            ThemeName::SolarizedDark => ThemeName::Default,
        }
    }
}

/// Theme struct that holds colors and provides style methods.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: ThemeName,
    pub colors: ThemeColors,
}

impl Theme {
    pub fn new(name: ThemeName) -> Self {
        let colors = match name {
            ThemeName::Default => Self::default_colors(),
            ThemeName::KanagawaWave => Self::kanagawa_wave_colors(),
            ThemeName::KanagawaDragon => Self::kanagawa_dragon_colors(),
            ThemeName::Dracula => Self::dracula_colors(),
            ThemeName::CatppuccinMocha => Self::catppuccin_mocha_colors(),
            ThemeName::GruvboxDark => Self::gruvbox_dark_colors(),
            ThemeName::Nord => Self::nord_colors(),
            ThemeName::TokyoNight => Self::tokyo_night_colors(),
            ThemeName::RosePine => Self::rose_pine_colors(),
            ThemeName::SolarizedDark => Self::solarized_dark_colors(),
        };
        Self { name, colors }
    }

    pub fn from_name(name: &str) -> Self {
        Self::new(ThemeName::from_str(name))
    }

    // ══════════════════════════════════════════════════════════════════════════
    // Theme Color Palettes
    // ══════════════════════════════════════════════════════════════════════════

    fn default_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(99, 102, 241),      // Indigo
            secondary: Color::Rgb(139, 92, 246),    // Violet
            accent: Color::Rgb(236, 72, 153),       // Pink

            success: Color::Rgb(34, 197, 94),       // Green
            warning: Color::Rgb(250, 204, 21),      // Yellow
            error: Color::Rgb(239, 68, 68),         // Red
            info: Color::Rgb(59, 130, 246),         // Blue

            bg_dark: Color::Rgb(15, 23, 42),        // Slate 900
            bg_card: Color::Rgb(30, 41, 59),        // Slate 800
            bg_elevated: Color::Rgb(51, 65, 85),    // Slate 700
            bg_highlight: Color::Rgb(71, 85, 105),  // Slate 600

            text: Color::Rgb(248, 250, 252),        // Slate 50
            text_muted: Color::Rgb(148, 163, 184),  // Slate 400
            text_dim: Color::Rgb(100, 116, 139),    // Slate 500

            rating_again: Color::Rgb(239, 68, 68),  // Red
            rating_hard: Color::Rgb(251, 191, 36),  // Amber
            rating_good: Color::Rgb(59, 130, 246),  // Blue
            rating_easy: Color::Rgb(34, 197, 94),   // Green
        }
    }

    /// Kanagawa Wave - inspired by the Great Wave painting
    fn kanagawa_wave_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x7E, 0x9C, 0xD8),      // crystalBlue
            secondary: Color::Rgb(0x95, 0x7F, 0xB8),    // oniViolet
            accent: Color::Rgb(0xD2, 0x7E, 0x99),       // sakuraPink

            success: Color::Rgb(0x98, 0xBB, 0x6C),      // springGreen
            warning: Color::Rgb(0xFF, 0x9E, 0x3B),      // roninYellow
            error: Color::Rgb(0xE8, 0x24, 0x24),        // samuraiRed
            info: Color::Rgb(0x7F, 0xB4, 0xCA),         // springBlue

            bg_dark: Color::Rgb(0x1F, 0x1F, 0x28),      // sumiInk3
            bg_card: Color::Rgb(0x2A, 0x2A, 0x37),      // sumiInk4
            bg_elevated: Color::Rgb(0x36, 0x36, 0x46),  // sumiInk5
            bg_highlight: Color::Rgb(0x2D, 0x4F, 0x67), // waveBlue2

            text: Color::Rgb(0xDC, 0xD7, 0xBA),         // fujiWhite
            text_muted: Color::Rgb(0xC8, 0xC0, 0x93),   // oldWhite
            text_dim: Color::Rgb(0x72, 0x71, 0x69),     // fujiGray

            rating_again: Color::Rgb(0xE8, 0x24, 0x24), // samuraiRed
            rating_hard: Color::Rgb(0xFF, 0x9E, 0x3B),  // roninYellow
            rating_good: Color::Rgb(0x7E, 0x9C, 0xD8),  // crystalBlue
            rating_easy: Color::Rgb(0x98, 0xBB, 0x6C),  // springGreen
        }
    }

    /// Kanagawa Dragon - warmer variant
    fn kanagawa_dragon_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x8B, 0xA4, 0xB0),      // dragonBlue2
            secondary: Color::Rgb(0x89, 0x92, 0xA7),    // dragonViolet
            accent: Color::Rgb(0xA2, 0x92, 0xA3),       // dragonPink

            success: Color::Rgb(0x87, 0xA9, 0x87),      // dragonGreen
            warning: Color::Rgb(0xC4, 0xB2, 0x8A),      // dragonYellow
            error: Color::Rgb(0xC4, 0x74, 0x6E),        // dragonRed
            info: Color::Rgb(0x8E, 0xA4, 0xA2),         // dragonAqua

            bg_dark: Color::Rgb(0x18, 0x16, 0x16),      // dragonBlack3
            bg_card: Color::Rgb(0x1D, 0x1C, 0x19),      // dragonBlack4
            bg_elevated: Color::Rgb(0x28, 0x27, 0x27),  // dragonBlack5
            bg_highlight: Color::Rgb(0x39, 0x36, 0x36), // dragonBlack6

            text: Color::Rgb(0xC5, 0xC9, 0xC5),         // dragonWhite
            text_muted: Color::Rgb(0xA6, 0xA6, 0x9C),   // dragonGray
            text_dim: Color::Rgb(0x62, 0x5E, 0x5A),     // dragonBlack6

            rating_again: Color::Rgb(0xC4, 0x74, 0x6E), // dragonRed
            rating_hard: Color::Rgb(0xC4, 0xB2, 0x8A),  // dragonYellow
            rating_good: Color::Rgb(0x8B, 0xA4, 0xB0),  // dragonBlue2
            rating_easy: Color::Rgb(0x87, 0xA9, 0x87),  // dragonGreen
        }
    }

    /// Dracula - popular dark theme
    fn dracula_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0xBD, 0x93, 0xF9),      // Purple
            secondary: Color::Rgb(0xFF, 0x79, 0xC6),    // Pink
            accent: Color::Rgb(0x8B, 0xE9, 0xFD),       // Cyan

            success: Color::Rgb(0x50, 0xFA, 0x7B),      // Green
            warning: Color::Rgb(0xF1, 0xFA, 0x8C),      // Yellow
            error: Color::Rgb(0xFF, 0x55, 0x55),        // Red
            info: Color::Rgb(0x8B, 0xE9, 0xFD),         // Cyan

            bg_dark: Color::Rgb(0x28, 0x2A, 0x36),      // Background
            bg_card: Color::Rgb(0x44, 0x47, 0x5A),      // Current Line
            bg_elevated: Color::Rgb(0x44, 0x47, 0x5A),  // Selection
            bg_highlight: Color::Rgb(0x62, 0x72, 0xA4), // Comment

            text: Color::Rgb(0xF8, 0xF8, 0xF2),         // Foreground
            text_muted: Color::Rgb(0xBD, 0xBD, 0xBD),   // Muted
            text_dim: Color::Rgb(0x62, 0x72, 0xA4),     // Comment

            rating_again: Color::Rgb(0xFF, 0x55, 0x55), // Red
            rating_hard: Color::Rgb(0xFF, 0xB8, 0x6C),  // Orange
            rating_good: Color::Rgb(0x8B, 0xE9, 0xFD),  // Cyan
            rating_easy: Color::Rgb(0x50, 0xFA, 0x7B),  // Green
        }
    }

    /// Catppuccin Mocha - soothing pastel theme
    fn catppuccin_mocha_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0xCB, 0xA6, 0xF7),      // Mauve
            secondary: Color::Rgb(0xF5, 0xC2, 0xE7),    // Pink
            accent: Color::Rgb(0x89, 0xDC, 0xEB),       // Sky

            success: Color::Rgb(0xA6, 0xE3, 0xA1),      // Green
            warning: Color::Rgb(0xF9, 0xE2, 0xAF),      // Yellow
            error: Color::Rgb(0xF3, 0x8B, 0xA8),        // Red
            info: Color::Rgb(0x89, 0xB4, 0xFA),         // Blue

            bg_dark: Color::Rgb(0x1E, 0x1E, 0x2E),      // Base
            bg_card: Color::Rgb(0x31, 0x32, 0x44),      // Surface0
            bg_elevated: Color::Rgb(0x45, 0x47, 0x5A),  // Surface1
            bg_highlight: Color::Rgb(0x58, 0x5B, 0x70), // Surface2

            text: Color::Rgb(0xCD, 0xD6, 0xF4),         // Text
            text_muted: Color::Rgb(0xA6, 0xAD, 0xC8),   // Subtext0
            text_dim: Color::Rgb(0x6C, 0x70, 0x86),     // Overlay0

            rating_again: Color::Rgb(0xF3, 0x8B, 0xA8), // Red
            rating_hard: Color::Rgb(0xFA, 0xB3, 0x87),  // Peach
            rating_good: Color::Rgb(0x89, 0xB4, 0xFA),  // Blue
            rating_easy: Color::Rgb(0xA6, 0xE3, 0xA1),  // Green
        }
    }

    /// Gruvbox Dark - retro groove
    fn gruvbox_dark_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x8E, 0xC0, 0x7C),      // Aqua
            secondary: Color::Rgb(0xD3, 0x86, 0x9B),    // Purple
            accent: Color::Rgb(0xFE, 0x80, 0x19),       // Orange

            success: Color::Rgb(0xB8, 0xBB, 0x26),      // Green
            warning: Color::Rgb(0xFA, 0xBD, 0x2F),      // Yellow
            error: Color::Rgb(0xFB, 0x49, 0x34),        // Red
            info: Color::Rgb(0x83, 0xA5, 0x98),         // Blue

            bg_dark: Color::Rgb(0x28, 0x28, 0x28),      // bg0
            bg_card: Color::Rgb(0x3C, 0x38, 0x36),      // bg1
            bg_elevated: Color::Rgb(0x50, 0x49, 0x45),  // bg2
            bg_highlight: Color::Rgb(0x66, 0x5C, 0x54), // bg3

            text: Color::Rgb(0xEB, 0xDB, 0xB2),         // fg
            text_muted: Color::Rgb(0xBD, 0xAE, 0x93),   // fg3
            text_dim: Color::Rgb(0x92, 0x83, 0x74),     // fg4

            rating_again: Color::Rgb(0xFB, 0x49, 0x34), // Red
            rating_hard: Color::Rgb(0xFA, 0xBD, 0x2F),  // Yellow
            rating_good: Color::Rgb(0x83, 0xA5, 0x98),  // Blue
            rating_easy: Color::Rgb(0xB8, 0xBB, 0x26),  // Green
        }
    }

    /// Nord - arctic, north-bluish
    fn nord_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x88, 0xC0, 0xD0),      // Frost - nord8
            secondary: Color::Rgb(0x81, 0xA1, 0xC1),    // Frost - nord9
            accent: Color::Rgb(0x5E, 0x81, 0xAC),       // Frost - nord10

            success: Color::Rgb(0xA3, 0xBE, 0x8C),      // Aurora green
            warning: Color::Rgb(0xEB, 0xCB, 0x8B),      // Aurora yellow
            error: Color::Rgb(0xBF, 0x61, 0x6A),        // Aurora red
            info: Color::Rgb(0x88, 0xC0, 0xD0),         // Frost

            bg_dark: Color::Rgb(0x2E, 0x34, 0x40),      // Polar Night - nord0
            bg_card: Color::Rgb(0x3B, 0x42, 0x52),      // Polar Night - nord1
            bg_elevated: Color::Rgb(0x43, 0x4C, 0x5E),  // Polar Night - nord2
            bg_highlight: Color::Rgb(0x4C, 0x56, 0x6A), // Polar Night - nord3

            text: Color::Rgb(0xEC, 0xEF, 0xF4),         // Snow Storm - nord6
            text_muted: Color::Rgb(0xD8, 0xDE, 0xE9),   // Snow Storm - nord4
            text_dim: Color::Rgb(0x4C, 0x56, 0x6A),     // Polar Night - nord3

            rating_again: Color::Rgb(0xBF, 0x61, 0x6A), // Red
            rating_hard: Color::Rgb(0xEB, 0xCB, 0x8B),  // Yellow
            rating_good: Color::Rgb(0x88, 0xC0, 0xD0),  // Frost
            rating_easy: Color::Rgb(0xA3, 0xBE, 0x8C),  // Green
        }
    }

    /// Tokyo Night - city lights
    fn tokyo_night_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x7A, 0xA2, 0xF7),      // Blue
            secondary: Color::Rgb(0xBB, 0x9A, 0xF7),    // Purple
            accent: Color::Rgb(0xFF, 0x9E, 0x64),       // Orange

            success: Color::Rgb(0x9E, 0xCE, 0x6A),      // Green
            warning: Color::Rgb(0xE0, 0xAF, 0x68),      // Yellow
            error: Color::Rgb(0xF7, 0x76, 0x8E),        // Red
            info: Color::Rgb(0x7D, 0xCF, 0xFF),         // Cyan

            bg_dark: Color::Rgb(0x1A, 0x1B, 0x26),      // bg
            bg_card: Color::Rgb(0x24, 0x28, 0x3B),      // bg_highlight
            bg_elevated: Color::Rgb(0x29, 0x2E, 0x42),  // terminal_black
            bg_highlight: Color::Rgb(0x3B, 0x40, 0x48), // bg_visual

            text: Color::Rgb(0xC0, 0xCA, 0xF5),         // fg
            text_muted: Color::Rgb(0xA9, 0xB1, 0xD6),   // fg_dark
            text_dim: Color::Rgb(0x56, 0x5F, 0x89),     // comment

            rating_again: Color::Rgb(0xF7, 0x76, 0x8E), // Red
            rating_hard: Color::Rgb(0xE0, 0xAF, 0x68),  // Yellow
            rating_good: Color::Rgb(0x7A, 0xA2, 0xF7),  // Blue
            rating_easy: Color::Rgb(0x9E, 0xCE, 0x6A),  // Green
        }
    }

    /// Rosé Pine - all natural pine, faux fur and a bit of soho vibes
    fn rose_pine_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0xEB, 0xBC, 0xBA),      // Rose
            secondary: Color::Rgb(0xC4, 0xA7, 0xE7),    // Iris
            accent: Color::Rgb(0xF6, 0xC1, 0x77),       // Gold

            success: Color::Rgb(0x31, 0x74, 0x8F),      // Pine (used as success)
            warning: Color::Rgb(0xF6, 0xC1, 0x77),      // Gold
            error: Color::Rgb(0xEB, 0x6F, 0x92),        // Love
            info: Color::Rgb(0x9C, 0xCF, 0xD8),         // Foam

            bg_dark: Color::Rgb(0x19, 0x17, 0x24),      // Base
            bg_card: Color::Rgb(0x1F, 0x1D, 0x2E),      // Surface
            bg_elevated: Color::Rgb(0x26, 0x23, 0x3A),  // Overlay
            bg_highlight: Color::Rgb(0x40, 0x3D, 0x52), // Highlight Med

            text: Color::Rgb(0xE0, 0xDE, 0xF4),         // Text
            text_muted: Color::Rgb(0x90, 0x8C, 0xAA),   // Subtle
            text_dim: Color::Rgb(0x6E, 0x6A, 0x86),     // Muted

            rating_again: Color::Rgb(0xEB, 0x6F, 0x92), // Love
            rating_hard: Color::Rgb(0xF6, 0xC1, 0x77),  // Gold
            rating_good: Color::Rgb(0x9C, 0xCF, 0xD8),  // Foam
            rating_easy: Color::Rgb(0x31, 0x74, 0x8F),  // Pine
        }
    }

    /// Solarized Dark - precision colors
    fn solarized_dark_colors() -> ThemeColors {
        ThemeColors {
            primary: Color::Rgb(0x26, 0x8B, 0xD2),      // Blue
            secondary: Color::Rgb(0x6C, 0x71, 0xC4),    // Violet
            accent: Color::Rgb(0x2A, 0xA1, 0x98),       // Cyan

            success: Color::Rgb(0x85, 0x99, 0x00),      // Green
            warning: Color::Rgb(0xB5, 0x89, 0x00),      // Yellow
            error: Color::Rgb(0xDC, 0x32, 0x2F),        // Red
            info: Color::Rgb(0x26, 0x8B, 0xD2),         // Blue

            bg_dark: Color::Rgb(0x00, 0x2B, 0x36),      // Base03
            bg_card: Color::Rgb(0x07, 0x36, 0x42),      // Base02
            bg_elevated: Color::Rgb(0x58, 0x6E, 0x75),  // Base01
            bg_highlight: Color::Rgb(0x65, 0x7B, 0x83), // Base00

            text: Color::Rgb(0xFD, 0xF6, 0xE3),         // Base3
            text_muted: Color::Rgb(0x93, 0xA1, 0xA1),   // Base1
            text_dim: Color::Rgb(0x65, 0x7B, 0x83),     // Base00

            rating_again: Color::Rgb(0xDC, 0x32, 0x2F), // Red
            rating_hard: Color::Rgb(0xB5, 0x89, 0x00),  // Yellow
            rating_good: Color::Rgb(0x26, 0x8B, 0xD2),  // Blue
            rating_easy: Color::Rgb(0x85, 0x99, 0x00),  // Green
        }
    }

    // ══════════════════════════════════════════════════════════════════════════
    // Styles
    // ══════════════════════════════════════════════════════════════════════════

    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.colors.text)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle(&self) -> Style {
        Style::default()
            .fg(self.colors.text_muted)
    }

    pub fn highlight(&self) -> Style {
        Style::default()
            .fg(self.colors.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected(&self) -> Style {
        Style::default()
            .bg(self.colors.bg_highlight)
            .fg(self.colors.text)
    }

    pub fn card_front(&self) -> Style {
        Style::default()
            .fg(self.colors.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn card_back(&self) -> Style {
        Style::default()
            .fg(self.colors.success)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_new(&self) -> Style {
        Style::default()
            .fg(self.colors.info)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_learning(&self) -> Style {
        Style::default()
            .fg(self.colors.warning)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_due(&self) -> Style {
        Style::default()
            .fg(self.colors.success)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_hint(&self) -> Style {
        Style::default()
            .fg(self.colors.text_dim)
    }

    pub fn key_highlight(&self) -> Style {
        Style::default()
            .fg(self.colors.accent)
            .add_modifier(Modifier::BOLD)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(ThemeName::Default)
    }
}

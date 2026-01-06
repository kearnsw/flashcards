//! Custom widgets for the flashcard TUI.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::BorderType, Block, Borders, Paragraph, Widget, Wrap},
};

use super::theme::Theme;
use crate::models::DeckStats;

// ══════════════════════════════════════════════════════════════════════════
// Logo Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct Logo;

impl Logo {
    const ART: &'static str = r#"
    ╭─────────────────────────────────────────╮
    │  _____ _           _                    │
    │ |  ___| | __ _ ___| |__   ___ __ _ _ __ │
    │ | |_  | |/ _` / __| '_ \ / __/ _` | '__││
    │ |  _| | | (_| \__ \ | | | (_| (_| | |   │
    │ |_|   |_|\__,_|___/_| |_|\___\__,_|_|   │
    │                     ┌───────────────┐   │
    │      ╭────╮         │ Spaced        │   │
    │      │ 🧠 │         │ Repetition    │   │
    │      ╰────╯         │ Learning      │   │
    │                     └───────────────┘   │
    ╰─────────────────────────────────────────╯"#;

    pub fn render(area: Rect, buf: &mut Buffer) {
        let lines: Vec<Line> = Self::ART
            .lines()
            .skip(1)
            .map(|line| {
                Line::from(vec![
                    Span::styled(line, Style::default().fg(Theme::PRIMARY))
                ])
            })
            .collect();

        let para = Paragraph::new(lines)
            .alignment(Alignment::Center);

        para.render(area, buf);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Stats Bar Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct StatsBar {
    stats: DeckStats,
}

impl StatsBar {
    pub fn new(stats: DeckStats) -> Self {
        Self { stats }
    }
}

impl Widget for StatsBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

        // New cards
        let new_text = Line::from(vec![
            Span::styled("● ", Theme::stats_new()),
            Span::styled("New: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                self.stats.new_cards.to_string(),
                Theme::stats_new(),
            ),
        ]);
        Paragraph::new(new_text)
            .alignment(Alignment::Center)
            .render(chunks[0], buf);

        // Learning cards
        let learning_text = Line::from(vec![
            Span::styled("● ", Theme::stats_learning()),
            Span::styled("Learning: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                self.stats.learning_cards.to_string(),
                Theme::stats_learning(),
            ),
        ]);
        Paragraph::new(learning_text)
            .alignment(Alignment::Center)
            .render(chunks[1], buf);

        // Due cards
        let due_text = Line::from(vec![
            Span::styled("● ", Theme::stats_due()),
            Span::styled("Due: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                self.stats.due_cards.to_string(),
                Theme::stats_due(),
            ),
        ]);
        Paragraph::new(due_text)
            .alignment(Alignment::Center)
            .render(chunks[2], buf);

        // Total
        let total_text = Line::from(vec![
            Span::styled("Total: ", Style::default().fg(Theme::TEXT_MUTED)),
            Span::styled(
                self.stats.total_cards.to_string(),
                Style::default().fg(Theme::TEXT_DIM),
            ),
        ]);
        Paragraph::new(total_text)
            .alignment(Alignment::Center)
            .render(chunks[3], buf);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Flashcard Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct FlashcardWidget<'a> {
    content: &'a str,
    is_front: bool,
}

impl<'a> FlashcardWidget<'a> {
    pub fn new(content: &'a str, is_front: bool) -> Self {
        Self { content, is_front }
    }
}

impl Widget for FlashcardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (label, label_style, border_style) = if self.is_front {
            ("QUESTION", Theme::card_front(), Style::default().fg(Theme::ACCENT))
        } else {
            ("ANSWER", Theme::card_back(), Style::default().fg(Theme::SUCCESS))
        };

        // Outer block with pretty border
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Line::from(vec![
                Span::raw(" "),
                Span::styled(label, label_style),
                Span::raw(" "),
            ]))
            .title_alignment(Alignment::Center);

        let inner = block.inner(area);
        block.render(area, buf);

        // Content
        let content_para = Paragraph::new(self.content)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Theme::TEXT));

        // Center vertically
        let content_height = self.content.lines().count() as u16;
        let vertical_padding = inner.height.saturating_sub(content_height) / 2;

        let content_area = Rect {
            x: inner.x + 2,
            y: inner.y + vertical_padding,
            width: inner.width.saturating_sub(4),
            height: inner.height.saturating_sub(vertical_padding),
        };

        content_para.render(content_area, buf);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Rating Buttons Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct RatingButtons<'a> {
    intervals: &'a [(crate::models::ReviewRating, String)],
    enabled: bool,
}

impl<'a> RatingButtons<'a> {
    pub fn new(intervals: &'a [(crate::models::ReviewRating, String)], enabled: bool) -> Self {
        Self { intervals, enabled }
    }
}

impl Widget for RatingButtons<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

        for (i, (rating, interval)) in self.intervals.iter().enumerate() {
            let color = if self.enabled {
                rating.color()
            } else {
                Theme::TEXT_DIM
            };

            let key = (i + 1).to_string();
            let name = rating.name();

            let button = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(color));

            let inner = button.inner(chunks[i]);
            button.render(chunks[i], buf);

            // Key number
            let key_line = Line::from(vec![
                Span::styled(&key, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            ]);
            Paragraph::new(key_line)
                .alignment(Alignment::Center)
                .render(
                    Rect {
                        y: inner.y,
                        ..inner
                    },
                    buf,
                );

            // Rating name
            let name_line = Line::from(vec![
                Span::styled(name, Style::default().fg(color)),
            ]);
            Paragraph::new(name_line)
                .alignment(Alignment::Center)
                .render(
                    Rect {
                        y: inner.y + 1,
                        ..inner
                    },
                    buf,
                );

            // Interval
            if self.enabled {
                let interval_line = Line::from(vec![
                    Span::styled(interval, Style::default().fg(Theme::TEXT_MUTED)),
                ]);
                Paragraph::new(interval_line)
                    .alignment(Alignment::Center)
                    .render(
                        Rect {
                            y: inner.y + 2,
                            ..inner
                        },
                        buf,
                    );
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Key Hints Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct KeyHints<'a> {
    hints: &'a [(&'a str, &'a str)],
}

impl<'a> KeyHints<'a> {
    pub fn new(hints: &'a [(&'a str, &'a str)]) -> Self {
        Self { hints }
    }
}

impl Widget for KeyHints<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let spans: Vec<Span> = self
            .hints
            .iter()
            .flat_map(|(key, desc)| {
                vec![
                    Span::styled(*key, Theme::key_highlight()),
                    Span::styled(format!(" {} ", desc), Theme::key_hint()),
                    Span::styled("│ ", Style::default().fg(Theme::TEXT_DIM)),
                ]
            })
            .collect();

        let line = Line::from(spans);
        Paragraph::new(line)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Completion Screen Widget
// ══════════════════════════════════════════════════════════════════════════

pub struct CompletionScreen {
    cards_studied: usize,
    duration_mins: u64,
}

impl CompletionScreen {
    pub fn new(cards_studied: usize, duration_mins: u64) -> Self {
        Self {
            cards_studied,
            duration_mins,
        }
    }
}

impl Widget for CompletionScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Theme::SUCCESS))
            .title(Line::from(vec![
                Span::raw(" "),
                Span::styled("✨ SESSION COMPLETE ✨", Theme::card_back()),
                Span::raw(" "),
            ]))
            .title_alignment(Alignment::Center);

        let inner = block.inner(area);
        block.render(area, buf);

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("🎉 ", Style::default()),
                Span::styled("Great job!", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
                Span::styled(" 🎉", Style::default()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Cards studied: ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::styled(
                    self.cards_studied.to_string(),
                    Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Time: ", Style::default().fg(Theme::TEXT_MUTED)),
                Span::styled(
                    format!("{} minutes", self.duration_mins),
                    Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Theme::TEXT_DIM)),
                Span::styled("ESC", Theme::key_highlight()),
                Span::styled(" to return", Style::default().fg(Theme::TEXT_DIM)),
            ]),
        ];

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .render(inner, buf);
    }
}

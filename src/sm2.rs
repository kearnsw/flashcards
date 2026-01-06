//! SM-2 Spaced Repetition Algorithm
//!
//! Implementation of the SuperMemo SM-2 algorithm for scheduling flashcard reviews.

use chrono::{Duration, Local};

use crate::models::{Card, ReviewRating};

/// Result of reviewing a card.
#[derive(Debug)]
pub struct ReviewResult {
    pub new_interval: u32,
    pub new_ease_factor: f64,
    pub next_due: chrono::DateTime<Local>,
}

/// SM-2 scheduler for flashcard reviews.
pub struct Scheduler {
    min_ease: f64,
    easy_bonus: f64,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            min_ease: 1.3,
            easy_bonus: 1.3,
        }
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate new ease factor based on rating.
    fn calculate_ease_factor(&self, card: &Card, rating: ReviewRating) -> f64 {
        // Map 0-3 rating to SM-2's 0-5 scale
        let q = (rating as u8) as f64 * 5.0 / 3.0;

        // SM-2 formula
        let new_ef = card.ease_factor + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02));

        new_ef.max(self.min_ease)
    }

    /// Calculate the next review interval in days.
    fn calculate_interval(&self, card: &Card, rating: ReviewRating, new_ease: f64) -> u32 {
        if rating == ReviewRating::Again {
            return 1;
        }

        if card.repetitions == 0 {
            // First review
            return match rating {
                ReviewRating::Again => 0,
                ReviewRating::Hard => 1,
                ReviewRating::Good => 1,
                ReviewRating::Easy => 4,
            };
        }

        if card.repetitions == 1 {
            // Second review
            return if rating == ReviewRating::Easy { 4 } else { 1 };
        }

        // Subsequent reviews
        let current = card.interval.max(1) as f64;

        let new_interval = match rating {
            ReviewRating::Again => 1.0,
            ReviewRating::Hard => current * 1.2,
            ReviewRating::Good => current * new_ease,
            ReviewRating::Easy => current * new_ease * self.easy_bonus,
        };

        (new_interval.round() as u32).max(1)
    }

    /// Process a card review and update its state.
    pub fn review_card(&self, card: &mut Card, rating: ReviewRating) -> ReviewResult {
        let now = Local::now();

        // Calculate new values
        let new_ease_factor = self.calculate_ease_factor(card, rating);
        let new_interval = self.calculate_interval(card, rating, new_ease_factor);

        // Update repetitions
        let new_repetitions = if rating == ReviewRating::Again {
            card.lapses += 1;
            0
        } else {
            card.repetitions + 1
        };

        // Calculate next due date
        let next_due = if rating == ReviewRating::Again && card.is_new() {
            now + Duration::minutes(10)
        } else {
            now + Duration::days(new_interval as i64)
        };

        // Update card
        card.ease_factor = new_ease_factor;
        card.interval = new_interval;
        card.repetitions = new_repetitions;
        card.due_date = Some(next_due);
        card.last_reviewed = Some(now);
        card.total_reviews += 1;

        ReviewResult {
            new_interval,
            new_ease_factor,
            next_due,
        }
    }

    /// Get human-readable interval string.
    pub fn interval_string(days: u32) -> String {
        if days == 0 {
            "< 1 min".to_string()
        } else if days == 1 {
            "1 day".to_string()
        } else if days < 7 {
            format!("{} days", days)
        } else if days < 30 {
            let weeks = days / 7;
            format!("{} week{}", weeks, if weeks > 1 { "s" } else { "" })
        } else if days < 365 {
            let months = days / 30;
            format!("{} month{}", months, if months > 1 { "s" } else { "" })
        } else {
            let years = days / 365;
            format!("{} year{}", years, if years > 1 { "s" } else { "" })
        }
    }

    /// Preview intervals for each rating.
    pub fn preview_intervals(&self, card: &Card) -> [(ReviewRating, String); 4] {
        let ratings = [
            ReviewRating::Again,
            ReviewRating::Hard,
            ReviewRating::Good,
            ReviewRating::Easy,
        ];

        ratings.map(|rating| {
            let new_ef = self.calculate_ease_factor(card, rating);
            let interval = self.calculate_interval(card, rating, new_ef);

            let interval_str = if rating == ReviewRating::Again && card.is_new() {
                "10 min".to_string()
            } else {
                Self::interval_string(interval)
            };

            (rating, interval_str)
        })
    }
}

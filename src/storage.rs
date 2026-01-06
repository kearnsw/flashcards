//! Storage module for saving and loading flashcard decks.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Deck;

/// Handles deck persistence.
pub struct DeckStorage {
    decks_dir: PathBuf,
}

impl DeckStorage {
    pub fn new(decks_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&decks_dir)
            .with_context(|| format!("Failed to create decks directory: {:?}", decks_dir))?;

        Ok(Self { decks_dir })
    }

    /// Get default storage location.
    pub fn default_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("flashcards")
            .join("decks")
    }

    fn deck_path(&self, deck_id: &str) -> PathBuf {
        self.decks_dir.join(format!("{}.json", deck_id))
    }

    /// Save a deck to disk.
    pub fn save_deck(&self, deck: &Deck) -> Result<PathBuf> {
        let path = self.deck_path(&deck.id);
        let json = serde_json::to_string_pretty(deck)?;
        fs::write(&path, json)?;
        Ok(path)
    }

    /// Load a deck from disk.
    pub fn load_deck(&self, deck_id: &str) -> Result<Option<Deck>> {
        let path = self.deck_path(deck_id);
        if !path.exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(&path)?;
        let deck: Deck = serde_json::from_str(&json)?;
        Ok(Some(deck))
    }

    /// Delete a deck file.
    pub fn delete_deck(&self, deck_id: &str) -> Result<bool> {
        let path = self.deck_path(deck_id);
        if path.exists() {
            fs::remove_file(&path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// List all available decks.
    pub fn list_decks(&self) -> Result<Vec<DeckInfo>> {
        let mut decks = Vec::new();

        for entry in fs::read_dir(&self.decks_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(deck) = serde_json::from_str::<Deck>(&json) {
                        decks.push(DeckInfo {
                            id: deck.id,
                            name: deck.name,
                            card_count: deck.cards.len(),
                            description: deck.description,
                        });
                    }
                }
            }
        }

        decks.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(decks)
    }

    /// Import cards from a CSV file.
    pub fn import_csv(&self, csv_path: &Path, deck_name: &str) -> Result<Deck> {
        let mut deck = Deck::new(deck_name.to_string());
        let content = fs::read_to_string(csv_path)?;

        for (i, line) in content.lines().enumerate() {
            // Skip header
            if i == 0 && line.to_lowercase().contains("front") {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let front = parts[0].trim().to_string();
                let back = parts[1].trim().to_string();

                if !front.is_empty() && !back.is_empty() {
                    deck.add_card(front, back);
                }
            }
        }

        Ok(deck)
    }

    /// Import all CSV files from a folder.
    /// Names decks based on filename, converting snake_case/kebab-case to Title Case.
    pub fn import_folder(&self, folder_path: &Path) -> Result<Vec<(String, usize)>> {
        let mut results = Vec::new();

        for entry in fs::read_dir(folder_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "csv") {
                let deck_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(filename_to_title_case)
                    .unwrap_or_else(|| "Imported Deck".to_string());

                match self.import_csv(&path, &deck_name) {
                    Ok(deck) => {
                        let card_count = deck.cards.len();
                        if card_count > 0 {
                            self.save_deck(&deck)?;
                            results.push((deck_name, card_count));
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to import {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(results)
    }
}

/// Convert a filename (snake_case or kebab-case) to Title Case.
fn filename_to_title_case(name: &str) -> String {
    name.split(|c| c == '_' || c == '-')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Summary info for a deck.
#[derive(Debug, Clone)]
pub struct DeckInfo {
    pub id: String,
    pub name: String,
    pub card_count: usize,
    pub description: String,
}

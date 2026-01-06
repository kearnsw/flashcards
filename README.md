# Flashcards TUI (Rust)

A beautiful, fast Anki-style spaced repetition flashcard application for the terminal.

Built with [Ratatui](https://ratatui.rs) for a modern TUI experience.

## Features

- **SM-2 Spaced Repetition** - Optimal review scheduling based on recall quality
- **Beautiful TUI** - Modern terminal interface with custom theme
- **Keyboard-Driven** - Fast, efficient studying with keyboard shortcuts
- **JSON Storage** - Human-readable deck format
- **CSV Import** - Easy import from spreadsheets

## Screenshots

```
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
    ╰─────────────────────────────────────────╯
```

### Study Screen

```
╭─────────────────── QUESTION ───────────────────╮
│                                                │
│        What is a list comprehension?           │
│                                                │
╰────────────────────────────────────────────────╯

● New: 8   ● Learning: 0   ● Due: 0   Total: 8

╭──────────╮ ╭──────────╮ ╭──────────╮ ╭──────────╮
│    1     │ │    2     │ │    3     │ │    4     │
│  Again   │ │   Hard   │ │   Good   │ │   Easy   │
│  10 min  │ │  1 day   │ │  1 day   │ │  4 days  │
╰──────────╯ ╰──────────╯ ╰──────────╯ ╰──────────╯
```

## Installation

### From Source

```bash
# Clone the repository
cd flashcards-rs

# Build release version
cargo build --release

# Run
./target/release/flashcards
```

### With Cargo

```bash
cargo install --path .
```

## Usage

### Run the TUI

```bash
flashcards
```

### Specify Decks Directory

```bash
flashcards --decks-dir ./my-decks
```

### Import from CSV

```bash
flashcards --import cards.csv --import-name "My Deck"
```

## Keyboard Shortcuts

### Deck Selection
| Key | Action |
|-----|--------|
| `↑/k` | Move up |
| `↓/j` | Move down |
| `Enter` | Select deck |
| `n` | New deck |
| `q` | Quit |

### Study Mode
| Key | Action |
|-----|--------|
| `Space` | Show answer |
| `1` | Again (forgot) |
| `2` | Hard |
| `3` | Good |
| `4` | Easy |
| `a` | Add card |
| `Esc` | Back to decks |

### Add Card
| Key | Action |
|-----|--------|
| `Tab` | Switch field |
| `Enter` | Add card / Next field |
| `Esc` | Done |

## Spaced Repetition (SM-2)

The algorithm adjusts intervals based on your recall:

- **Again (1)** - Forgot completely → Review in 10 minutes
- **Hard (2)** - Struggled → Interval × 1.2
- **Good (3)** - Normal recall → Interval × ease factor
- **Easy (4)** - Perfect recall → Interval × ease factor × 1.3

Cards you know well appear less frequently. Cards you struggle with appear more often.

## Deck Format

Decks are stored as JSON:

```json
{
  "id": "abc123",
  "name": "Python Basics",
  "cards": [
    {
      "id": "card1",
      "front": "What is a list comprehension?",
      "back": "A concise way to create lists...",
      "ease_factor": 2.5,
      "interval": 0,
      "repetitions": 0
    }
  ]
}
```

## CSV Import Format

```csv
front,back,tags
Question 1,Answer 1,topic1;topic2
Question 2,Answer 2,topic1
```

## Theme Colors

The TUI uses a beautiful dark theme with semantic colors:

- **Primary**: Indigo (#6366F1)
- **Accent**: Pink (#EC4899)
- **Success**: Green (#22C55E)
- **Warning**: Yellow (#FACC15)
- **Error**: Red (#EF4444)

## License

MIT

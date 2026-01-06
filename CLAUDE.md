# SRL (Spaced Repetition Learning)

## Project Overview
A TUI flashcard application with spaced repetition, built in Rust using ratatui.

## Repository Structure
- **Main repo:** https://github.com/kearnsw/flashcards
- **Homebrew tap:** https://github.com/kearnsw/homebrew-tap
- **Binary name:** `srl`
- **Package name:** `flashcards`

## Release Process

To release a new version:

1. **Bump version** in `Cargo.toml`
2. **Commit changes** to main branch
3. **Create and push a git tag:** `git tag -a vX.Y.Z -m "Description" && git push origin vX.Y.Z`
4. **Get sha256 of tarball:** `curl -sL https://github.com/kearnsw/flashcards/archive/refs/tags/vX.Y.Z.tar.gz | shasum -a 256`
5. **Update homebrew-tap:**
   - Clone: `git clone git@github.com:kearnsw/homebrew-tap.git`
   - Edit `Formula/srl.rb`: update `url` (version) and `sha256`
   - Commit and push

Use `/release` slash command to automate this process.

## Installation
```bash
brew tap kearnsw/tap
brew install srl
```

## Development
```bash
cargo build
cargo run
```

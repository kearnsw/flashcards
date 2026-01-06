# Release

Release a new version of SRL to GitHub and Homebrew tap.

## Instructions

When the user runs `/release`, perform these steps:

1. **Check for uncommitted changes** - if any exist, ask user to commit first or offer to commit them

2. **Determine version bump** - Ask user: patch (0.0.X), minor (0.X.0), or major (X.0.0)?

3. **Update Cargo.toml** - Bump the version number

4. **Commit the version bump** if not already committed

5. **Create and push git tag:**
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin main
   git push origin vX.Y.Z
   ```

6. **Get sha256 of the release tarball:**
   ```bash
   curl -sL https://github.com/kearnsw/flashcards/archive/refs/tags/vX.Y.Z.tar.gz | shasum -a 256
   ```

7. **Update homebrew-tap:**
   - Clone to /tmp: `git clone git@github.com:kearnsw/homebrew-tap.git /tmp/homebrew-tap`
   - Edit `/tmp/homebrew-tap/Formula/srl.rb`:
     - Update `url` to new version tag
     - Update `sha256` to new hash
   - Commit and push the homebrew-tap changes

8. **Report success** with:
   - New version number
   - Git tag
   - Homebrew update status
   - Command to upgrade: `brew update && brew upgrade srl`

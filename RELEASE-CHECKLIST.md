# Release checklist

- `git pull`
- Look for outdated deps
- `git push`, make sure CI passes
  - Optionally run tests/clippy locally or do next steps while waiting for CI
- `cargo publish --dry-run -p crate-name`
- Bump version
- Update CHANGELOG.md
- `git tag -a 'crate-name-vX.Y.Z'
  - If specifying multiple versions, put more important first in case the line gets truncated on GitHub
- `cargo publish -p crate-name`
- `git push` the tag
- GitHub release
  - Copy relevant part of CHANGELOG.md to description

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md). Note that ripgrep is a binary, we don't need some steps such as updating Cargo.lock.

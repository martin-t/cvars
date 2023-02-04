# Release checklist

- Make sure CI passes
- `cargo publish --dry-run -p crate-name`
- Bump version
- Update CHANGELOG.md
- `git tag -a 'crate-name-vX.Y.Z'
  - If specifying multiple versions, put more important first in case the line gets truncated on GitHub
- `cargo publish -p crate-name`
- GitHub release
  - Copy relevant part of CHANGELOG.md to description

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md)

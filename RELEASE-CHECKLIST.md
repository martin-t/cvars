# Release checklist

- Make sure CI passes
- `cargo publish --dry-run -p crate-name`
- Bump version
- Update CHANGELOG.md
- `git tag -a 'crate-name-vX.Y.Z'
- `cargo publish -p crate-name`
- GitHub release - copy relevant part of CHANGELOG.md to description

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md)

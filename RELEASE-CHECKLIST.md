# Release checklist

- Make sure CI passes
- `cargo publish --dry-run -p crate-name`
- Bump version
- Update CHANGELOG.md
- Git tag
- `cargo publish`
- GitHub release

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md)

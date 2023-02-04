# Release checklist

- `git pull`
- Look for fixmes/todos
- Look for outdated deps
- `cargo publish --dry-run -p crate-name`
- Bump version
- Update CHANGELOG.md
- `git push`, make sure CI passes
  - This is after bumping versions to make sure all the packages in the repo are still compatible
- `cargo publish -p crate-name`
- `git tag -a 'crate-name-vX.Y.Z`
  - If specifying multiple versions, put more important first in case the line gets truncated on GitHub
- `git push` the tag
- GitHub release
  - Copy relevant part of CHANGELOG.md to description

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md). Note that ripgrep is a binary, we don't need some steps such as updating Cargo.lock.

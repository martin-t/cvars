# Release checklist

- `git pull`
- Look for fixmes/todos
- Look for outdated deps (`cargo outdated` or [deps.rs](https://deps.rs/repo/github/martin-t/cvars))
- Bump version(s)
- For workspace and separately for consoles and benches: `cargo update` (even for libs [^lockfile])
  - This is after bumping versions since the crates depend on each other
- Update CHANGELOG.md
- Commit, `git push`, make sure CI passes
  - This is after bumping versions to make sure all the packages in the repo are still compatible
- For each package to release:
  - Optional: Double check the generated package has the correct versions of `cvars-*` deps - `cargo publish --dry-run -p crate-name` or `cargo tree`.
  - `cargo publish -p crate-name`
- `git tag -a crate-name-vX.Y.Z`
  - If specifying multiple versions, put more important first in case the line gets truncated on GitHub
- `git push` the tag
- GitHub release
  - Copy relevant part of CHANGELOG.md to description

Inspiration: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/RELEASE-CHECKLIST.md). Note that ripgrep is a binary, we don't need some steps such as comitting Cargo.lock.

[^lockfile]: Docs.rs appears to respect Cargo.lock even for libs, this might cause outdated docs when using `pub use`.
  Not sure what exactly happened but it seems cvars v0.3.1 depended on an outdated version of cvars-macros
  which caused removed a todo comment to appear on [docs.rs](https://docs.rs/cvars/0.3.1/cvars/macro.cvars.html).

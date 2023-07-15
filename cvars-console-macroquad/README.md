<div align="center">
    <h1>Macroquad console</h1>
    <br />
    A simple and ergonomic way to store and edit configuration in your game at runtime
</div>
<br />

[![GitHub](https://img.shields.io/badge/github-martin--t/cvars-8da0cb?logo=github)](https://github.com/martin-t/cvars)
[![Docs.rs](https://img.shields.io/badge/docs.rs-cvars--console--macroquad-66c2a5?logo=docs.rs)](https://docs.rs/cvars-console-macroquad)
[![Crates.io](https://img.shields.io/crates/v/cvars-console-macroquad?logo=rust)](https://crates.io/crates/cvars-console-macroquad)
[![License (AGPL3)](https://img.shields.io/github/license/martin-t/cvars)](https://github.com/martin-t/cvars/blob/master/LICENSE)
[![CI](https://github.com/martin-t/cvars/workflows/CI-macroquad/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Audit](https://github.com/martin-t/cvars/workflows/audit-macroquad/badge.svg)](https://rustsec.org/)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg?path=cvars-console-macroquad)](https://deps.rs/repo/github/martin-t/cvars?path=cvars-console-macroquad)
[![Discord](https://img.shields.io/badge/-Discord-7389d8?logo=discord&label=&logoColor=ffffff&labelColor=6A7EC2)](https://discord.gg/aA7hCFvYh9)

In-game console for the [macroquad](https://github.com/not-fl3/macroquad) game engine for changing [cvars](https://github.com/martin-t/cvars) at runtime.

![Macroquad console](screenshot.png)

## Usage

- Add `cvars-console-macroquad` to your `Cargo.toml`:

```shell
cargo add cvars-console-macroquad
```

- Create a `MacroquadConsole` when initializing your game.

- Call its `update` method in your main loop.

## Real-world example

See how [RecWars](https://github.com/martin-t/rec-wars) uses [cvars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs) and the console.

## Compatibility

The version of macroquad used by your game has to match the version used by cvars-console-macroquad, otherwise you'll get a segfault. Unlike with cvars-console-fyrox, there is no error at compile time.

You can use `cargo tree` to debug the issue but in general **every time you update the engine after a breaking change, you have to update the console**.

This means that there has to be a new major[^major] release of cvars-console-macroquad for each new major release of macroquad even though there are no changes to the console. I will try to release a new version soon after macroquad but since i am the only maintainer, it might not always be possible. If you need to use the latest macroquad and cvars-console-macroquad hasn't caught up yet, feel free to submit a PR.

You can also temporarily make a fork of the console with the macroquad version number updated and add a [patch section](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section) to your `Cargo.toml`.

[^major]: Since macroquad's version number is `0.y.z`, changing `y` is considered a major release as per [Cargo's flavor of semantic versioning](https://doc.rust-lang.org/cargo/reference/semver.html#change-categories).

## License

AGPL-v3 or newer

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

## License

AGPL-v3 or newer

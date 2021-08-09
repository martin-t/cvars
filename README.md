<div align="center">
    <h1>Cvars</h1>
    <i>Configuration variables .rs</i>
    <br />
    A simple and ergonomic way to store and edit runtime configuration in your game or other software.
</div>
<br />

[![CI](https://github.com/martin-t/cvars/workflows/CI/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg)](https://deps.rs/repo/github/martin-t/cvars)
[![Discord](https://img.shields.io/discord/770013530593689620?label=discord)](https://discord.gg/aA7hCFvYh9)
![Total lines](https://tokei.rs/b1/github/martin-t/cvars)
![Lines of comments](https://tokei.rs/b1/github/martin-t/cvars?category=comments)

Cvars (*console variables* or *configuration variables*) are a way to store settings which the user might want to change at runtime without restarting. They are inspired by the idSoftware family of game engines but they can be useful outside games.

TL;DR: Set and get struct fields based on the field's name as a string.

# Usage

Your game's config is in a struct like this:

```rust
use cvars::SetGet;

#[derive(SetGet)]
pub struct Cvars {
    g_rocket_launcher_ammo_max: i32,
    g_rocket_launcher_damage: f32,
}
```

The player wants to change a cvar and types `g_rocket_launcher_damage 150` into the game's console or stdin - you get both the cvar name and new value as strings so you can't do `cvars.g_rocket_launcher_damage = 150`. Instead, you call `cvars.set_str("g_rocket_launcher_damage", "150");` which looks up the correct field and parses the value into the proper type. From then on, rockets do 150 damage.

The important thing is that you can still access your cvars as regular struct fields - e.g. `player.health -= cvars.g_rocket_launcher_damage;`. This means you only need to use strings when the user (player or developer when debugging or testing a different balance) is reading or writing the values. The rest of your gamelogic is still statically typed and using a cvar in gamecode is just a field access without any overhead.

For a real-world example, look at [how RecWars uses cvars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs).

### Enums

Cvars can have any type which implements the `FromStr` and `Display` traits. If you want to use enums, it's best to derive these traits automatically via `[strum](https://crates.io/crates/strum)`.

TODO Example

### MSRV

The minimum supported Rust version is currently 1.54 because of `#![doc = include_str!("README.md")]`. It could be lowered to 1.36 or 1.31 if somebody was interested in using this lib but couldn't use latest Rust.

# (Planned) Features

- [x] Derive macro `SetGet` to create settters and getters for cvars based on their name
    - [x] Statically typed (`set`, `get`)
    - [x] As string (`set_str`, `get_string`)
- [x] Function like `cvars!` macro to declare type and initial value on one line
- [ ] Allow setters to validate the new value and reject it (e.g. make sure it's within a sane range).
- [ ] Autocompletion for in-game consoles
- [ ] Console for macroquad
- [ ] Console for rg3d
- [ ] Browser GUI for games without a console

# Alternatives

TODO compare performance and pros/cons (boiletplate, accessible to players, ...)

- [tuna](https://crates.io/crates/tuna)
- [cvar](https://crates.io/crates/cvar)
- [const-tweaker](https://crates.io/crates/const-tweaker)
- [inline_tweak](https://crates.io/crates/inline_tweak)

# License

AGPL-v3 or newer

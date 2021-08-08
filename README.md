<div align="center">
    <h1>Cvars</h1>
    <i>Configuration variables .rs</i>
    <br />
    A simple and ergonomic way to store and edit runtime configuration in your game or other software.
</div>
<br />

[![CI](https://github.com/martin-t/cvars/workflows/CI/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg)](https://deps.rs/repo/github/martin-t/cvars)
[![Discord](https://img.shields.io/discord/770013530593689620?label=discord)](https://discord.gg/9BQVVgV)
![Total lines](https://tokei.rs/b1/github/martin-t/cvars)
![Lines of comments](https://tokei.rs/b1/github/martin-t/cvars?category=comments)

Cvars (*console variables* or *configuration variables*) are a way to store settings which the user might want to change at runtime without restarting. They are inspired by the idSoftware family of game engines but they can be useful outside games.

TL;DR: Set and get struct fields based on the field's name as a string.

# Usage

Your game's config is in a struct like this:

```rust
#[derive(SetGet)]
pub struct Cvars {
    g_rocket_launcher_ammo_max: i32,
    g_rocket_launcher_damage: f32,
}
```

The player can then type `g_rocket_launcher_damage 150`. This calls `cvars.set_str("g_rocket_launcher_damage", "150");` and from then on, rockets do 150 damage.

The important thing is that you can still access your cvars as regular struct fields - e.g. `player.health -= cvars.g_rocket_launcher_damage;`. This means you only need to use strings when the user (player or developer when debugging or testing a different balance) is reading or writing the values. The rest of your gamelogic is still statically typed and using a cvar is just a field access without any overhead.

TODO Simple example, test if cargo docs runs it

For a real-world example, look at [how RecWars uses cvars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs).

- TODO Docs in lib?
- TODO MSRV

### Enums

Cvars can have any type which implements the `FromStr` and `Display` traits. If you want to use enums, it's best to derive these traits automatically via `[strum](https://crates.io/crates/strum)`.

TODO Example

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

# Alternatives:

TODO compare performance and pros/cons (boiletplate, accessible to players, ...)

- [tuna](https://crates.io/crates/tuna)
- [cvar](https://crates.io/crates/cvar)
- [const-tweaker](https://crates.io/crates/const-tweaker)
- [inline_tweak](https://crates.io/crates/inline_tweak)

# License

AGPL-v3 or newer

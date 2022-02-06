<div align="center">
    <h1>Cvars</h1>
    <i>Configuration variables .rs</i>
    <br />
    A simple and ergonomic way to store and edit runtime configuration in your game or other software.
</div>
<br />

[![Crates.io](https://img.shields.io/crates/v/cvars)](https://crates.io/crates/cvars)
[![CI](https://github.com/martin-t/cvars/workflows/CI/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Audit](https://github.com/martin-t/cvars/workflows/audit/badge.svg)](https://rustsec.org/)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg)](https://deps.rs/repo/github/martin-t/cvars)
[![Discord](https://img.shields.io/discord/770013530593689620?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/aA7hCFvYh9)
![Total lines](https://tokei.rs/b1/github/martin-t/cvars)

Cvars (_console variables_ or _configuration variables_) are a way to store settings which the user might want to change at runtime without restarting. They are inspired by the idTech (Doom, Quake) and Source family of game engines but they can be useful outside games. Cvars allow you to iterate faster by letting you test certain gameplay changes without recompiling. They also make your game more moddable.

**TL;DR**: Set and get struct fields based on the field's name as a string. User writes the cvar's name and new value, it sets the appropriate field in your config struct.

<a href="https://user-images.githubusercontent.com/4079823/152082630-a705286d-c630-4507-9213-b8a7b106d47e.mp4">Usage example video</a>

The cvars crate aims to minimize boilerplate - there are no traits to implement manually and no setup code to call per cvar. There is also no extra performance cost for keeping everything configurable even after you're done finding the best values - you can (and are meant to) keep things tweakable for your players to experiment themselves.

# Usage

```rust
use cvars::SetGet;

// This struct contains all your config options.
#[derive(SetGet)]
pub struct Cvars {
    g_rocket_launcher_ammo_max: i32,
    g_rocket_launcher_damage: f32,
}

// Here you set default values.
impl Cvars {
    pub fn new() -> Self {
        Self {
            g_rocket_launcher_ammo_max: 20,
            g_rocket_launcher_damage: 100.0,
        }
    }
}

// Normally, you store this within or next to your game state.
let mut cvars = Cvars::new();

// These normally come from the user
// (from stdin / your game's console / etc.)
let cvar_name = "g_rocket_launcher_damage";
let new_value = "150";

// This looks up the right field and sets it to the new value.
cvars.set_str(cvar_name, new_value).unwrap();
```

The player wants to change a cvar and types `g_rocket_launcher_damage 150` into the game's console or stdin. You get both the cvar name and new value as strings so you can't do `cvars.g_rocket_launcher_damage = 150`. You need to look up the correct field based on the string - this is what `cvars` does - it generates `set_str` (and some other useful methods). You call `cvars.set_str("g_rocket_launcher_damage", "150");` which looks up the right field, parses the value into its type and updates the field with it. From then on, rockets do 150 damage.

The important thing is that in the rest of your application, you can still access your cvars as regular struct fields - e.g. `player.health -= cvars.g_rocket_launcher_damage;`. This means you only need to use strings when the user (player or developer when debugging or testing a different balance) is changing the values. The rest of your gamelogic is still statically typed and using a cvar in gamecode is just a field access without any overhead.

See [examples/stdin.rs](https://github.com/martin-t/cvars/blob/master/examples/stdin.rs) for a small runnable example.

For a real-world example, look at [how RecWars uses cvars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs).

## Enums

Cvar values can have any type which implements the `FromStr` and `Display` traits. If you want to use enums, it's best to derive these traits automatically via `[strum](https://crates.io/crates/strum)`.

```rust
use strum_macros::{Display, EnumString};

use cvars::SetGet;

#[derive(Debug, Clone, SetGet)]
pub struct Cvars {
    pub cl_splitscreen: Splitscreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Splitscreen {
    Vertical,
    Horizontal,
}
```

Tip: use `#[strum(ascii_case_insensitive)]` so players don't need to pay attention to capilatization when changing cvars - both `"Vertical"` and `"vertical"` will parse into `Splitscreen::Vertical`.

## Skipping fields

If a field is not meant to be configurable, mark it with `#[cvars(skip)]`.

## MSRV

The minimum supported Rust version is currently 1.54 because of `#![doc = include_str!("README.md")]`. It could be lowered to 1.36 or 1.31 if somebody was interested in using this lib but couldn't use latest Rust.

# (Planned) Features

- [x] Derive macro `SetGet` to create settters and getters for cvars based on their name
  - [x] Statically typed (`set`, `get`)
  - [x] As string (`set_str`, `get_string`)
- [x] Function like `cvars!` macro to declare type and initial value on one line
- [ ] Save config to and load it from files - useful if your game has multiple balance presets
- [ ] Allow setters to validate the new value and reject it (e.g. make sure it's within a sane range).
- [ ] Console for Fyrox (rg3d) - planned soon™
- [ ] Console for Bevy - not currently planned but will accept a PR
- [ ] Console for Macroquad - implemented [here](https://github.com/martin-t/rec-wars/blob/master/src/console.rs), needs to be extracted into a separate crate
- [ ] Browser GUI for games without a console
- [ ] Autocompletion for in-game consoles / GUI

# Alternatives

- [inline_tweak](https://crates.io/crates/inline_tweak)
  - Uses hashmaps - overhead on every access
  - Can't be used in some contexts (e.g. in a `const`)
  - Veloren switched to it from const-tweaker
- [const-tweaker](https://crates.io/crates/const-tweaker)
  - Web GUI
  - Only supports a few stdlib types, no custom types
  - Has soundness issues [according](https://github.com/tgolsson/tuna#alternatives) to tuna's author
  - Uses hashmaps - overhead on every access
- [tuna](https://crates.io/crates/tuna)
  - Web GUI
  - Unclear if it supports enums
  - Uses hashmaps - overhead on every access
- [cvar](https://crates.io/crates/cvar)
  - Uses a trait instead of a macro. The trait seems to need to be implemented manually so more boilerplate.
  - Has additional features (lists, actions) which `cvars` currently doesn't.

Compared to these, cvars either has no overhead at runtime or requires less setup code. The downside [currently](https://github.com/martin-t/cvars/issues/6) is that it increases compile times - e.g. 300 cvars can add 700 ms to incremental builds even when no cvars were changed. This should be fixed before a 1.0 release.

# License

AGPL-v3 or newer

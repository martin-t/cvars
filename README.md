<div align="center">
    <h1>Cvars</h1>
    <i>Configuration variables .rs</i>
    <br />
    A simple and ergonomic way to store and edit runtime configuration in your game or other software.
</div>
<br />

[![Discord](https://img.shields.io/discord/770013530593689620?label=discord)](https://discord.gg/9BQVVgV)

Cvars (*console variables* or *configuration variables*) are a way to store settings which the user might want to change at runtime without restarting. They are inspired by the idSoftware family of game engines but they can be useful outside games.

TODO Simple example

For a real-world example, look at [how RecWars uses cvars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs).

- TODO Changelog
- TODO Docs in lib?
- TODO Enums must impl FromStr and Display (use strum, case insensitive)

# (Planned) Features

- [x] Get and set cvars by their name
- [x] String-based access
- [x] Statically typed access
- [ ] Function like macro to declare type and initial value on one line
- [ ] Autocompletion for in-game consoles
- [ ] Console for macroquad
- [ ] Console for rg3d
- [ ] Browser GUI for games without a console

# Alternatives:

- [tuna](https://crates.io/crates/tuna)
- [cvar](https://crates.io/crates/cvar)
- [const-tweaker](https://crates.io/crates/const-tweaker)
- [inline_tweak](https://crates.io/crates/inline_tweak)

# License

[AGPL-v3](LICENSE) or newer

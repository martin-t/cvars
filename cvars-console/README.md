# Cvars-console

Engine-independant parts of in-game consoles.

Currently parsing and executing commands, help, history. Eventually cvarlist, search, tab completion, ...

_Internal crate._

If you're writing a game, you don't need to use this crate directly. Instead, use one of the frontends depending on your game engine:

- [cvars-console-fyrox](https://crates.io/crates/cvars-console-fyrox)
- [cvars-console-macroquad](https://crates.io/crates/cvars-console-macroquad)

You only need to depend on `cvars-console` if you're writing a new console frontend.

Also see the main [cvars](https://crates.io/crates/cvars) crate.

## License

AGPL-v3 or newer

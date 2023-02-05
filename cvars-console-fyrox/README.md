# Cvars-console-fyrox

[![Crates.io](https://img.shields.io/crates/v/cvars-console-fyrox)](https://crates.io/crates/cvars-console-fyrox)
[![License (AGPL3)](https://img.shields.io/github/license/martin-t/cvars)](https://github.com/martin-t/cvars/blob/master/LICENSE)
[![CI](https://github.com/martin-t/cvars/workflows/CI-fyrox/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Audit](https://github.com/martin-t/cvars/workflows/audit-fyrox/badge.svg)](https://rustsec.org/)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg?path=cvars-console-fyrox)](https://deps.rs/repo/github/martin-t/cvars?path=cvars-console-fyrox)
[![Discord](https://img.shields.io/discord/770013530593689620?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/aA7hCFvYh9)

In-game console for the [Fyrox](https://github.com/FyroxEngine/Fyrox) game engine for changing [cvars](https://github.com/martin-t/cvars) at runtime.

![Fyrox console](https://raw.githubusercontent.com/martin-t/cvars/master/media/cvars-console-fyrox.png)

## Usage

- Add `cvars-console-fyrox` to your `Cargo.toml`:

```shell
cargo add cvars-console-fyrox
```

- Create a `FyroxConsole` when initializing your game:

```rust,ignore
FyroxConsole::new(&mut engine.user_interface);
```

You're responsible for opening and closing the console according to your game's key bindings.
You also need to call `resized` and `ui_message` on the appropriate engine events.

## Real-world example

See how [RustCycles](https://github.com/rustcycles/rustcycles) uses [cvars](https://github.com/rustcycles/rustcycles/blob/master/src/cvars.rs) and the [console](https://github.com/rustcycles/rustcycles/blob/master/src/client/process.rs).

## License

AGPL-v3 or newer

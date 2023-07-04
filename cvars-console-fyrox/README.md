<div align="center">
    <h1>Fyrox console</h1>
    <br />
    A simple and ergonomic way to store and edit configuration in your game at runtime
</div>
<br />

[![GitHub](https://img.shields.io/badge/github-martin--t/cvars-8da0cb?logo=github)](https://github.com/martin-t/cvars)
[![Docs.rs](https://img.shields.io/badge/docs.rs-cvars--console--fyrox-66c2a5?logo=docs.rs)](https://docs.rs/cvars-console-fyrox)
[![Crates.io](https://img.shields.io/crates/v/cvars-console-fyrox?logo=rust)](https://crates.io/crates/cvars-console-fyrox)
[![License (AGPL3)](https://img.shields.io/github/license/martin-t/cvars)](https://github.com/martin-t/cvars/blob/master/LICENSE)
[![CI](https://github.com/martin-t/cvars/workflows/CI-fyrox/badge.svg)](https://github.com/martin-t/cvars/actions)
[![Audit](https://github.com/martin-t/cvars/workflows/audit-fyrox/badge.svg)](https://rustsec.org/)
[![Dependency status](https://deps.rs/repo/github/martin-t/cvars/status.svg?path=cvars-console-fyrox)](https://deps.rs/repo/github/martin-t/cvars?path=cvars-console-fyrox)
[![Discord](https://img.shields.io/badge/-Discord-7389d8?logo=discord&label=&logoColor=ffffff&labelColor=6A7EC2)](https://discord.gg/aA7hCFvYh9)

In-game console for the [Fyrox](https://github.com/FyroxEngine/Fyrox) game engine for changing [cvars](https://github.com/martin-t/cvars) at runtime.

![Fyrox console](screenshot.png)

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

## Compatibility

The version of fyrox-ui used by your game must match the version used by cvars-console-fyrox, otherwise you'll get confusing errors such as:

```text
expected struct `fyrox_ui::UserInterface`, found struct `UserInterface`
```

You can use `cargo tree` to debug the issue.

This means that there has to be a new version of cvars-console-fyrox for each new version of fyrox-ui even though there are no changes to the console. If you're using the latest Fyrox and cvars-console-fyrox hasn't caught up yet, you might have to temporarily make a fork of the console with the fyrox-ui version number updated and add a [patch section](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section) to your `Cargo.toml`.

## License

AGPL-v3 or newer

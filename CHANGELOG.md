# Changelog

This is a shared changelog for all projects in this repo:

- The main [`cvars`](https://crates.io/crates/cvars) crate (with its internal proc macros in `cvars-macros`)
- The console backend [`cvars-console`](https://crates.io/crates/cvars-console)
- Frontends for game engines
  - [`cvars-console-fyrox`](https://crates.io/crates/cvars-console-fyrox)
  - [`cvars-console-macroquad`](https://crates.io/crates/cvars-console-macroquad)

## Cvars, cvars-macros - unreleased

- Docs

## Cvars-console-fyrox v0.1.0 - 2023-02-06

- First release - in-game console with history

## Cvars-console-macroquad v0.1.0 - 2023-02-06

- First release - in-game console with history

## Cvars-console v0.1.0 - 2023-02-04

- Basic help message
- Command history
- Setting / getting cvars

## Cvars v0.3.0, cvars-macros v0.2.0 - 2023-02-04

- **Breaking change:** The `cvars! {}` macro now generates a `Default` impl that uses the specified values instead of the default for the field's type. It no longer generates a `new` function.

## Cvars v0.2.0, cvars-macros 0.1.1 - 2023-02-03

- `SetGet` trait for dynamic dispatch
- Accept attributes in `cvars! {}`

## Cvars-macros 0.1.0 - 2023-01-16

- `#[cvars(skip)]` for ignoring some struct fields
- First release - split off proc macros from the main `cvars` crate

## Cvars v0.1.0 - 2021-08-11

- Experimental `cvars!` macro to generate the `Cvars` struct
- Derive macro for structs (`#[derive(SetGet)]`) which generates getters and setters

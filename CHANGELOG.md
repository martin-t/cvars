# Changelog

This is a shared changelog for all projects in this repo:

- The main `cvars` crate
- The proc macros in `cvars-macros`
- The console backend `cvars-console`
- Frontends for game engines `cvars-console-fyrox` and `cvars-console-macroquad`

## Cvars-console-fyrox v0.1.0 - TODO

- First release - in-game console with history

## Cvars-console-macroquad v0.1.0 - TODO

- First release - in-game console with history

## Cvars-console v0.1.0 - TODO

- Basic help message
- Command history
- Setting / getting cvars

## Cvars v0.2.0 - 2023-02-03

- `SetGet` trait for dynamic dispatch

## Cvars-macros 0.1.1 - 2023-02-03

- Accept attributes in `cvars! {}`

## Cvars-macros 0.1.0 - 2023-01-16

- `#[cvars(skip)]` for ignoring some struct fields
- First release - split off proc macros from the main `cvars` crate

## Cvars v0.1.0 - 2021-08-11

- Experimental `cvars!` macro to generate the `Cvars` struct
- Derive macro for structs (`#[derive(SetGet)]`) which generates getters and setters

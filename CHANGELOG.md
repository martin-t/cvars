# Changelog

This is a shared changelog for all projects in this repo:

- The main [`cvars`](https://crates.io/crates/cvars) crate (with its internal proc macros in `cvars-macros`)
- The console backend [`cvars-console`](https://crates.io/crates/cvars-console)
- Frontends for game engines
  - [`cvars-console-fyrox`](https://crates.io/crates/cvars-console-fyrox)
  - [`cvars-console-macroquad`](https://crates.io/crates/cvars-console-macroquad)

## Cvars-macros v0.4.1 - 2023-07-08

- Add support for non-`Copy` types such as `String`.

## Cvars-console-fyrox v0.2.0 - 2023-07-01

- **Breaking change:** Update to cvars v0.4.0
- **Breaking change:** Update to fyrox-ui v0.21.0

## Cvars-console and cvars-console-macroquad v0.2.0 - 2023-07-01

- **Breaking change:** Update to cvars v0.4.0

## Cvars and cvars-macros v0.4.0 - 2023-07-01

- Update to syn 2
- Internal improvements: unified macro logic so `cvars!` no longer depends on `#[derive(SetGet)]`.
- **Breaking change:** `cvars!` no longer adds `#[derive(Debug, Clone)]` to the generated struct. This might improve compile times with a large (~10k) number of cvars.
- Reduce the amount of code generated per-cvar (6x fewer lines of LLVM IR):
  - Recompile speedup after editing cvars: 10.5s -> 3.5s for 1k cvars (3x faster).
  - Recompile speedup after editing other code: 700ms -> 450ms for 1k cvars (1.5x faster).
- The `cvars!` proc macro is no longer experimental and is the recommended way to use cvars because it is more convenient than `#[derive(SetGet)]`.
- Add `cvar_count()` and `CVAR_COUNT` to get the number of cvars.
- Accept `0`, `1`, `f` and `t` for bool cvars.
- `cvars!` now accepts inner attributes and comments - see its docs for usage examples.

## Cvars v0.3.2 - 2023-02-08

- Fix docs of reexported items

## Cvars v0.3.1, cvars-macros v0.2.1 - 2023-02-06

- Fix and improve docs

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
- Accept attributes and comments in `cvars! {}`

## Cvars-macros 0.1.0 - 2023-01-16

- `#[cvars(skip)]` for ignoring some struct fields
- First release - split off proc macros from the main `cvars` crate

## Cvars v0.1.0 - 2021-08-11

- Experimental `cvars!` macro to generate the `Cvars` struct
- Derive macro for structs (`#[derive(SetGet)]`) which generates getters and setters

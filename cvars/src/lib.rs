//! <div align="center">
//!     <h1>Cvars</h1>
//!     <i>Configuration variables .rs</i>
//!     <br />
//!     A simple and ergonomic way to store and edit runtime configuration in your program
//! </div>
//! <br />
//!
//! **TL;DR:** Cvars allow you to access struct fields based on their name as a string.
//!
//! This is useful for app configuration, especially in games - gamedevs, modders and players access cvars dynamically using a TUI
//! while your gamecode accesses them statically as struct fields.
//!
//! The `cvars` crate contains proc macros to generate:
//! - setter and getter methods directly on your config struct
//! - an impl of `SetGet` for your config struct so it can use dynamic dispatch
//!
//! In-game consoles are in separate crates - pick one based on your game engine:
//! - [cvars-console-fyrox](https://crates.io/crates/cvars-console-fyrox) - [Fyrox](https://crates.io/crates/fyrox) console
//! - [cvars-console-macroquad](https://crates.io/crates/cvars-console-macroquad) - [Macroquad](https://crates.io/crates/macroquad) console
//!
//! # Example
//!
//! ```rust
//! use cvars::cvars;
//!
//! // This generates a Cvars struct containing all your config options
//! // and a corresponding Default impl.
//! cvars! {
//!     g_rocket_launcher_ammo_max: i32 = 20,
//!     g_rocket_launcher_damage: f32 = 100.0,
//! }
//!
//! // Store this in your game state.
//! let mut cvars = Cvars::default();
//!
//! // These normally come from the user
//! // (from stdin / your game's console / etc.)
//! let cvar_name = "g_rocket_launcher_damage";
//! let new_value = "150";
//!
//! // This looks up the right field and sets it to the new value.
//! cvars.set_str(cvar_name, new_value).unwrap();
//! ```
//!
//! A player/modder/gamedev wants rockets to do more damage.
//! He types `g_rocket_launcher_damage 150` into the game's console or stdin.
//! The code gets both the cvar name and new value as strings
//! so you can't write `cvars.g_rocket_launcher_damage = 150`.
//! You need to look up the correct field based on the string - this is what `cvars` does - it generates `set_str`
//! (and some other useful methods). You call `cvars.set_str("g_rocket_launcher_damage", "150");`
//! which looks up the right field, parses the value into its type and updates the field with it.
//! From then on, rockets do 150 damage.
//!
//! The important thing is that in the rest of your application,
//! you can still access your cvars as regular struct fields - e.g. `player.health -= cvars.g_rocket_launcher_damage;`.
//! This means you only need to use strings when the user
//! (player or developer when debugging or testing a different balance) is changing the values.
//! The rest of your **gamelogic is still statically typed** and using a cvar in gamecode
//! is just a field access without any overhead.
//!
//! A typical game will have hundreds or thousands of tunable parameters.
//! With cvars and a console you can keep them all configurable for advanced players,
//! modders and your-gamedev-self without having a build and elaborate settings menu.
//! You can keep everything configurable using a TUI
//! while also exposing common settings to normal players in your game's GUI.
//!
//! See [cvars/examples/stdin.rs](https://github.com/martin-t/cvars/blob/master/cvars/examples/stdin.rs)
//! for a small runnable example.
//!
//! For real-world examples, look at games using cvars:
//!
//! - [RecWars](https://github.com/martin-t/rec-wars/blob/master/src/cvars.rs) - uses the Macroquad console,
//!   every aspect of the gameplay is configurable,
//!   you can test it [in your browsser](https://martin-t.gitlab.io/gitlab-pages/rec-wars/macroquad.html)
//! - [RustCycles](https://github.com/rustcycles/rustcycles/blob/master/src/cvars.rs) - uses the Fyrox console
//!
//! # What it generates
//!
//! - A struct containing all your cvars
//! - A Default impl with the specified default values
//! - Methods to set and get cvars by name
//!
//! The generated methods have these signatures (same as the `SetGet` trait):
//!
//! ```rust
//! # struct Cvars {}
//! impl Cvars {
//!     /// Finds the cvar whose name matches `cvar_name` and returns its value as a `String`.
//!     ///
//!     /// Returns `Err` if the cvar doesn't exist.
//!     pub fn get_string(&self, cvar_name: &str) -> Result<String, String>
//!     # { unimplemented!() }
//!
//!     /// Finds the cvar whose name matches `cvar_name`, tries to parse `str_value` to its type and sets it to the parsed value.
//!     ///
//!     /// Returns `Err` if the cvar doesn't exist or if `str_value` fails to parse to its type.
//!     pub fn set_str(&mut self, cvar_name: &str, str_value: &str) -> Result<(), String>
//!     # { unimplemented!() }
//! }
//! ```
//!
//! # Enums
//!
//! Cvar values can have any type which implements the `FromStr` and `Display` traits.
//! If you want to use enums, it's best to derive these traits automatically
//! via [strum](https://crates.io/crates/strum).
//!
//! ```rust
//! use strum_macros::{Display, EnumString};
//!
//! use cvars::cvars;
//!
//! cvars! {
//!     cl_splitscreen: Splitscreen = Splitscreen::Vertical,
//! }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
//! #[strum(ascii_case_insensitive)]
//! pub enum Splitscreen {
//!     Vertical,
//!     Horizontal,
//! }
//! ```
//!
//! Tip: use `#[strum(ascii_case_insensitive)]` so players don't need to pay attention to capilatization
//! when changing cvars - both `"Vertical"` and `"vertical"` will parse into `Splitscreen::Vertical`.
//!
//! # Skipping fields
//!
//! If a field is not meant to be configurable, mark it with `#[cvars(skip)]`.
//!
//! # Related crates
//!
//! See the [README](https://github.com/martin-t/cvars) for more information about the cvars family of crates
//! and for comparison with alternatives such as [inline_tweak](https://crates.io/crates/inline_tweak).

#![warn(missing_docs)]

pub use cvars_macros::{cvars, SetGet, SetGetDummy};

/// A trait for writing generic code that can access cvars but doesn't know the concrete Cvars struct.
///
/// This is implemented automatically by both `#[derive(SetGet)]` and `cvars! {}`.
///
/// The methods provided here call those implemented directly on the concrete Cvars struct,
/// there is no difference between them.
///
/// Implementation note: This trait can't include the `get` and `set` methods
/// because it would no longer be object-safe.
pub trait SetGet {
    /// Finds the cvar whose name matches `cvar_name` and returns it's value as a `String`.
    ///
    /// Returns `Err` if the cvar doesn't exist.
    fn get_string(&self, cvar_name: &str) -> Result<String, String>;

    /// Finds the cvar whose name matches `cvar_name`, tries to parse `str_value` to its type and sets it to the parsed value.
    ///
    /// Returns `Err` if the cvar doesn't exist or if `str_value` fails to parse to its type.
    fn set_str(&mut self, cvar_name: &str, str_value: &str) -> Result<(), String>;
}

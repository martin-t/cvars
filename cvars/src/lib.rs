#![doc = include_str!("../../README.md")]

//#![warn(missing_docs)] TODO

pub use cvars_macros::{SetGet, SetGetDummy, cvars};

/// A trait for writing generic code
/// that can access cvars but doesn't know the concrete Cvars struct.
///
/// Implementation note: This trait can't include the `get` and `set` methods
/// because it would no longer be object-safe.
pub trait SetGet {
    fn get_string(&self, cvar_name: &str) -> Result<String, String>;
    fn set_str(&mut self, cvar_name: &str, str_value: &str) -> Result<(), String>;
}

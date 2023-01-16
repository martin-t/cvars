#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

pub use cvars_macros::{cvars, SetGet, SetGetDummy};

/// A trait for writing generic code that can access cvars but doesn't know the concrete Cvars struct.
///
/// The methods provided here are also available on the concrete Cvars struct directly.
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

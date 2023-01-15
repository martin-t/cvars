#![doc = include_str!("../../README.md")]

//#![warn(missing_docs)] TODO

pub use cvars_macros::{SetGet, SetGetDummy, cvars};

/// A mostly internal trait for writing generic code
/// that can access cvars but doesn't know the concrete Cvars struct.
/// TODO Rename to SetGet?
pub trait CvarAccess {
    fn get<T>(&self, cvar_name: &str) -> Result<T, String>;
    fn get_string(&self, cvar_name: &str) -> Result<String, String>;
    fn set<T>(&mut self, cvar_name: &str, value: T) -> Result<(), String>;
    fn set_str(&mut self, cvar_name: &str, str_value: &str) -> Result<(), String>;
}

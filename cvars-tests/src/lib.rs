//! The `cvars` crate reexports macros from `cvars-macros`
//! so users only need to add one crate as a dependency.
//! This means everything (tests, examples, other macros) uses `cvars::` as a prefix.
//! That in turn means tests for cvars-macros must be in a separate crate to avoid a circular dependency.

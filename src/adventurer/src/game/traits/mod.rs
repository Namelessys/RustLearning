// src/traits.rs
pub mod has_name;
pub mod greeter;

// flat re-exports so callers don't need deep paths:
pub(crate) use has_name::{Name, HasName, impl_has_name_via};
pub(crate) use greeter::Greeter;
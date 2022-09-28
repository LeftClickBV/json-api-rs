//! Collection types with consistent ordering.

pub mod map;
pub mod set;

pub use indexmap::Equivalent;

pub use self::{map::Map, set::Set};

//! Public facade of Metafix core.

pub mod api; // make `metafix_core::api` visible

mod engine; // internal

// re-export the most-used symbols for ergonomic `use metafix_core::prelude::*`
pub use api::prelude::*;

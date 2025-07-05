#![cfg_attr(not(debug_assertions), deny(clippy::todo, clippy::dbg_macro))]
#![cfg_attr(debug_assertions, warn(clippy::todo, clippy::dbg_macro))]

//! Public facade of Metafix core.

pub mod api; // make `metafix_core::api` visible

mod engine;
mod parser;

// re-export the most-used symbols for ergonomic `use metafix_core::prelude::*`
pub use api::prelude::*;

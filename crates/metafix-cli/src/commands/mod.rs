#![cfg_attr(not(debug_assertions), deny(clippy::todo, clippy::dbg_macro))]
#![cfg_attr(debug_assertions, warn(clippy::todo, clippy::dbg_macro))]

mod scan;
pub use scan::*;



#![allow(dead_code)]

mod config;
mod requests;
mod parse;
mod fetch;
mod vars;
mod types;

// Re-exports
pub use fetch::{
    fetch,
    say_hi,
    Request,
    Schema,
};
pub use types::Map;
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod context;
mod ffi;
pub mod gen;

mod support;
pub use support::*;
pub mod options;
pub mod types;

pub mod combinatory;
pub mod integers;

#![warn(clippy::pedantic)]
//! This crate implements a high level abstraction of the sqlparser library: <https://github.com/sqlparser-rs/sqlparser-rs>.
//!
//! For each dialect there's a corresponding implementation for all the AST,
//! including all structures and interfaces. Although this implementation is
//! quite limited and a little excessive, it's a good way to get started with
//! dialect-specific implementations, instead of using the way too genetic
//! implementation from the parser library.

extern crate core;

pub mod ansi;
pub mod common;

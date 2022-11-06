#![warn(clippy::all, clippy::pedantic, clippy::missing_docs_in_private_items)]
//! This crate implements a high level abstraction of the sqlparser library: <https://github.com/sqlparser-rs/sqlparser-rs>.
//!
//! For each dialect there's a corresponding implementation for all the AST,
//! including all structures and interfaces. Although this implementation is
//! quite limited and a little excessive, it's a good way to get started with
//! dialect-specific implementations, instead of using the way too genetic
//! implementation from the parser library.

/// AST structures and functions for `ANSI` data type.
///
/// Based on documentation from ANSI standard 2016 [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html
pub mod ansi;

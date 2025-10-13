//! [![crates.io](https://img.shields.io/crates/v/commented?style=flat-square&logo=rust)](https://crates.io/crates/commented)
//! [![docs.rs](https://img.shields.io/badge/docs.rs-commented-blue?style=flat-square&logo=docs.rs)](https://docs.rs/commented)
//! ![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
//! ![msrv](https://img.shields.io/badge/msrv-1.85-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/commented)](https://github.com/nik-rev/commented)
//!
//! This crate contains the [`comment`](comment) function, which turns the passed input
//! into a comment for the given file type
//!
//! ```toml
//! commented = "0.2.0"
//! ```
//!
//! # Usage
//!
//! ```rust
//! use commented::comment;
//!
//! assert_eq!(comment("hello, world!", "index.html"), "<!-- hello, world! -->");
//! assert_eq!(comment("hello, world!", "main.rs"), "/* hello, world! */");
//! assert_eq!(comment("hello, world!", "script.lua"), "--[[ hello, world! --]]");
//! // fallback to `#`, as that's the most common comment tokens for
//! // files without an extension
//! assert_eq!(comment("hello, world!", "123456789"), "# hello, world!");//!
//! ```
//!
//! # Features
//!
//! Enabling the `glob` feature will *slightly* improve accuracy of the comment heuristics.

use std::path::Path;

mod generated;

/// Turn the given `content` into a comment for the given `path`
///
/// # Examples
///
/// ```
/// use commented::comment;
///
/// assert_eq!(comment("hello, world!", "index.html"), "<!-- hello, world! -->");
/// assert_eq!(comment("hello, world!", "main.rs"), "/* hello, world! */");
/// assert_eq!(comment("hello, world!", "script.lua"), "--[[ hello, world! --]]");
/// // fallback to `#`, as that's the most common comment tokens for
/// // files without an extension
/// assert_eq!(comment("hello, world!", "123456789"), "# hello, world!");
/// ```
pub fn comment<S: ToString, P: AsRef<Path>>(content: S, path: P) -> String {
    generated::comment(content, path)
}

//! This crate contains the [`comment`] function, which turns the passed input
//! into a comment for the given file type
//!
//! ```toml
//! commented = "0.1.0"
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

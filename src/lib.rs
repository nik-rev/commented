//! This crate contains the [`comment`] function, which turns the passed input
//! into a comment for the given file type

use std::path::Path;

mod generated;

/// Turn the given `content` into a comment for the given `path`
///
/// # Examples
///
/// ```
/// use comment_line::comment;
///
/// assert_eq!(comment("hello, world!", "index.html"), "<!-- hello, world! -->");
/// assert_eq!(comment("hello, world!", "main.rs"), "/* hello, world! */");
/// assert_eq!(comment("hello, world!", "script.lua"), "--[[ hello, world! --]]");
/// assert_eq!(comment("hello, world!", "123456789"), "# hello, world!");
/// ```
pub fn comment<S: ToString, P: AsRef<Path>>(content: S, path: P) -> String {
    generated::comment(content, path)
}

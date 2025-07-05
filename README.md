This crate contains the `comment` function, which turns the passed input
into a comment for the given file type

```rs
use commented::comment;

assert_eq!(comment("hello, world!", "index.html"), "<!-- hello, world! -->");
assert_eq!(comment("hello, world!", "main.rs"), "/* hello, world! */");
assert_eq!(comment("hello, world!", "script.lua"), "--[[ hello, world! --]]");
// fallback to `#`, as that's the most common comment tokens for
// files without an extension
assert_eq!(comment("hello, world!", "123456789"), "# hello, world!");
```

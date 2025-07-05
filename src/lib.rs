use std::path::Path;

mod generated;

pub fn comment_line<S: ToString, P: AsRef<Path>>(content: S, path: P) -> String {
    generated::comment_line(content, path)
}

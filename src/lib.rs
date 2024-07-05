//! A tiny crate providing a function to convert a input string to a valid safe
//! filename.
//!
//! ## Examples
//!
//! Convert a input string to a valid safe filename.
//!
//! ```
//! use filenamify::filenamify;
//! let safe_filename = filenamify("//foo/bar/file");
//! assert_eq!(safe_filename, "_foo_bar_file");
//! ```

use std::sync::OnceLock;
use regex::Regex;

static RESERVED: OnceLock<Regex> = OnceLock::new();

/// Convert a input string to a valid safe filename.
/// 
/// See [`crate` level documentation] for an example
///
/// [`crate` level documentation]: crate
///
pub fn filenamify<S: AsRef<str>>(input: S) -> String {
    let replacemant = "_";
    let reserved = RESERVED.get_or_init( || Regex::new("[<>:\"/\\\\|?*\u{0000}-\u{001F}\u{007F}\u{0080}-\u{009F}]+").unwrap());
    let windows_reserved = Regex::new("^(con|prn|aux|nul|com\\d|lpt\\d)$").unwrap();
    let outer_periods = Regex::new("^\\.+|\\.+$").unwrap();

    let input = reserved.replace_all(input.as_ref(), replacemant);
    let input = outer_periods.replace_all(input.as_ref(), replacemant);

    let mut result = input.into_owned();
    if windows_reserved.is_match(result.as_str()) {
        result.push_str(replacemant);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::filenamify;

    #[test]
    fn test_filenamify() {
        assert_eq!(filenamify("foo/bar"), "foo_bar");
        assert_eq!(filenamify("foo//bar"), "foo_bar");
        assert_eq!(filenamify("//foo//bar//"), "_foo_bar_");
        assert_eq!(filenamify("foo\\bar"), "foo_bar");
        assert_eq!(filenamify("foo\\\\\\bar"), "foo_bar");
        assert_eq!(filenamify(r"foo\\bar"), "foo_bar");
        assert_eq!(filenamify(r"foo\\\\\\bar"), "foo_bar");
        assert_eq!(filenamify("////foo////bar////"), "_foo_bar_");
        assert_eq!(filenamify("foo\u{0000}bar"), "foo_bar");
        assert_eq!(filenamify("\"foo<>bar*"), "_foo_bar_");
        assert_eq!(filenamify("."), "_");
        assert_eq!(filenamify(".."), "_");
        assert_eq!(filenamify("./"), "__");
        assert_eq!(filenamify("../"), "__");
        assert_eq!(filenamify("../../foo/bar"), "__.._foo_bar");
        assert_eq!(filenamify("foo.bar."), "foo.bar_");
        assert_eq!(filenamify("foo.bar.."), "foo.bar_");
        assert_eq!(filenamify("foo.bar..."), "foo.bar_");
        assert_eq!(filenamify("con"), "con_");
        assert_eq!(filenamify("com1"), "com1_");
        assert_eq!(filenamify(":nul|"), "_nul_");
        assert_eq!(filenamify("foo/bar/nul"), "foo_bar_nul");
        assert_eq!(filenamify("file:///file.tar.gz"), "file_file.tar.gz");
        assert_eq!(filenamify("http://www.google.com"), "http_www.google.com");
        assert_eq!(filenamify("https://www.youtube.com/watch?v=dQw4w9WgXcQ"), "https_www.youtube.com_watch_v=dQw4w9WgXcQ");
    }
}

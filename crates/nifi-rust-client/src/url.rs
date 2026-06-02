//! URL helpers for emitter-generated code.
//!
//! Path-param values are arbitrary strings (NiFi IDs are UUIDs in
//! practice but the spec doesn't guarantee it, and downstream callers
//! may forward untrusted values). All path-param substitutions go
//! through [`encode_path_segment`] so a `/` or `?` in the value cannot
//! reshape the URL.

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

/// RFC 3986 path-segment "reserved" + unsafe characters. We encode
/// every byte that would be interpreted by either `url::Url::set_path`
/// or the server's URL parser as a structural delimiter, plus
/// historical hostile bytes (`%`, space, `"`, `` ` ``, `<`, `>`, `\`).
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'/')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'\\')
    .add(b'`')
    .add(b'{')
    .add(b'}');

/// Percent-encode an arbitrary string for safe use as a single URL
/// path segment. Used by emitter-generated code to substitute
/// path-param values into endpoint paths.
pub fn encode_path_segment(s: &str) -> String {
    utf8_percent_encode(s, PATH_SEGMENT).to_string()
}

/// Percent-encode an arbitrary string for use as a *multi-segment* URL
/// path value — i.e. a value that legitimately contains `/` separators
/// (e.g. NiFi policy resources like `process-groups/root`).
///
/// Slashes are preserved as structural delimiters, but every other
/// reserved/unsafe byte is still encoded *within* each segment, so a `?`
/// or `#` in the value cannot reshape the URL into a query/fragment.
///
/// Used by emitter-generated code for path params whose OpenAPI schema
/// declares a slash-permitting `pattern` (see `parser.rs`).
pub fn encode_path_multi_segment(s: &str) -> String {
    s.split('/')
        .map(encode_path_segment)
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_slash_question_hash() {
        assert_eq!(encode_path_segment("a/b"), "a%2Fb");
        assert_eq!(encode_path_segment("a?b"), "a%3Fb");
        assert_eq!(encode_path_segment("a#b"), "a%23b");
    }

    #[test]
    fn leaves_uuid_untouched() {
        let uuid = "8f3b2d1c-4a5e-4f6a-9b8c-1d2e3f4a5b6c";
        assert_eq!(encode_path_segment(uuid), uuid);
    }

    #[test]
    fn encodes_space_and_unicode() {
        assert_eq!(encode_path_segment("hello world"), "hello%20world");
        assert_eq!(encode_path_segment("café"), "caf%C3%A9");
    }

    #[test]
    fn encodes_path_traversal() {
        assert_eq!(encode_path_segment(".."), ".."); // dots themselves are safe;
        // the encoding rejects path separators which are what makes traversal possible.
        assert_eq!(encode_path_segment("../etc/passwd"), "..%2Fetc%2Fpasswd");
    }

    #[test]
    fn multi_segment_preserves_slashes() {
        // NiFi policy resources are genuinely multi-segment paths.
        assert_eq!(
            encode_path_multi_segment("process-groups/root"),
            "process-groups/root"
        );
        assert_eq!(
            encode_path_multi_segment("processors/8f3b2d1c-4a5e-4f6a-9b8c-1d2e3f4a5b6c"),
            "processors/8f3b2d1c-4a5e-4f6a-9b8c-1d2e3f4a5b6c"
        );
    }

    #[test]
    fn multi_segment_still_encodes_within_segments() {
        // Slashes stay structural, but query/fragment injection inside a
        // segment is still neutralized.
        assert_eq!(encode_path_multi_segment("a?b/c#d"), "a%3Fb/c%23d");
        assert_eq!(encode_path_multi_segment("a b/c d"), "a%20b/c%20d");
    }
}

//! Server-quirk compatibility shims.
//!
//! The types in this module exist to absorb cases where a real NiFi server's
//! wire format does not match its own published OpenAPI spec. Keeping the
//! workarounds isolated here lets the generated code stay a faithful
//! translation of the spec while the client still deserializes responses
//! from production servers.
//!
//! The currently shipped shim is [`FlexibleString`].

use serde::{Deserialize, Deserializer, Serialize};

/// String wrapper that deserializes from either a JSON string or a JSON
/// number (rendered as the number's decimal form).
///
/// NiFi servers do not always honour their own OpenAPI spec for `date-time`
/// fields. NiFi 2.6.0 returns a formatted timestamp string for
/// [`StatusSnapshotDTO::timestamp`][1] (matching the spec's
/// `string + format: date-time`), but later versions emit the same field as
/// a JSON integer holding Unix milliseconds. The generator therefore renders
/// every `string + format: date-time` field as `Option<FlexibleString>` so
/// the same DTO compiles and deserializes against every supported NiFi
/// version.
///
/// `FlexibleString` derefs to `&str` and serializes back as a JSON string,
/// so most consumer code (`as_deref()`, `Display`, equality with `&str`,
/// `Into<String>`) keeps working unchanged.
///
/// [1]: https://nifi.apache.org/nifi-docs/rest-api.html
///
/// # Examples
///
/// ```
/// use nifi_rust_client::FlexibleString;
///
/// // JSON string (NiFi 2.6.0 wire shape)
/// let from_str: FlexibleString =
///     serde_json::from_str(r#""04/27/2026 10:00:00 UTC""#).unwrap();
/// assert_eq!(&*from_str, "04/27/2026 10:00:00 UTC");
///
/// // JSON integer (NiFi 2.9.0 wire shape — Unix milliseconds)
/// let from_int: FlexibleString = serde_json::from_str("1777382685086").unwrap();
/// assert_eq!(&*from_int, "1777382685086");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct FlexibleString(pub String);

impl FlexibleString {
    /// Borrow the inner string as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the owned `String`.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::ops::Deref for FlexibleString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FlexibleString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FlexibleString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for FlexibleString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for FlexibleString {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<FlexibleString> for String {
    fn from(s: FlexibleString) -> Self {
        s.0
    }
}

impl PartialEq<str> for FlexibleString {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for FlexibleString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for FlexibleString {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl<'de> Deserialize<'de> for FlexibleString {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = FlexibleString;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a string or a number")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v.to_owned()))
            }

            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v))
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v.to_string()))
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v.to_string()))
            }

            fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v.to_string()))
            }

            fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<FlexibleString, E> {
                Ok(FlexibleString(v.to_string()))
            }

            // NiFi never emits floating-point timestamps, but be defensive: a
            // fractional value gets truncated to its integer component, which
            // matches what `as i64` would produce for sane inputs.
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<FlexibleString, E> {
                Ok(FlexibleString((v as i64).to_string()))
            }
        }

        de.deserialize_any(V)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_from_string() {
        let v: FlexibleString = serde_json::from_str(r#""04/27/2026 10:00:00 UTC""#).unwrap();
        assert_eq!(&*v, "04/27/2026 10:00:00 UTC");
    }

    #[test]
    fn deserialize_from_unix_millis_integer() {
        let v: FlexibleString = serde_json::from_str("1777382685086").unwrap();
        assert_eq!(&*v, "1777382685086");
    }

    #[test]
    fn deserialize_from_negative_integer() {
        let v: FlexibleString = serde_json::from_str("-1").unwrap();
        assert_eq!(&*v, "-1");
    }

    #[test]
    fn deserialize_from_zero() {
        let v: FlexibleString = serde_json::from_str("0").unwrap();
        assert_eq!(&*v, "0");
    }

    #[test]
    fn deserialize_from_float_truncates() {
        let v: FlexibleString = serde_json::from_str("123.456").unwrap();
        assert_eq!(&*v, "123");
    }

    #[test]
    fn deserialize_inside_struct() {
        #[derive(Deserialize)]
        struct Snap {
            timestamp: Option<FlexibleString>,
        }
        let s: Snap = serde_json::from_str(r#"{"timestamp": 1777382685086}"#).unwrap();
        assert_eq!(s.timestamp.as_deref(), Some("1777382685086"));

        let s: Snap = serde_json::from_str(r#"{"timestamp": "anything"}"#).unwrap();
        assert_eq!(s.timestamp.as_deref(), Some("anything"));

        let s: Snap = serde_json::from_str(r#"{"timestamp": null}"#).unwrap();
        assert!(s.timestamp.is_none());
    }

    #[test]
    fn serialize_round_trips_as_string() {
        let v = FlexibleString::from("1777382685086");
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#""1777382685086""#);
    }

    #[test]
    fn equality_with_str_and_string() {
        let v = FlexibleString::from("hello");
        assert_eq!(v, *"hello");
        assert_eq!(v, "hello");
        assert_eq!(v, String::from("hello"));
    }

    #[test]
    fn deref_and_display() {
        let v = FlexibleString::from("abc");
        let lower: &str = &v;
        assert_eq!(lower, "abc");
        assert_eq!(format!("{v}"), "abc");
    }

    #[test]
    fn into_string_and_back() {
        let s: String = FlexibleString::from("xyz").into();
        assert_eq!(s, "xyz");
        let v: FlexibleString = s.into();
        assert_eq!(&*v, "xyz");
    }
}

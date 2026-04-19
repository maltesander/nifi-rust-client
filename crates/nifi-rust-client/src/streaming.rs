#![deny(missing_docs)]

//! Streaming byte responses for endpoints returning unbounded binary
//! payloads (provenance content, flowfile content, NAR/asset downloads).
//!
//! Every generated endpoint whose OpenAPI response body is
//! `application/octet-stream` or `*/*` ships in two flavors:
//!
//! - Buffered: `fn ...() -> Result<Vec<u8>, NifiError>`.
//! - Streaming: `fn ..._stream() -> Result<BytesStream, NifiError>`.
//!
//! Use the streaming variant when the response may be large enough
//! that buffering into a `Vec<u8>` is undesirable.
//!
//! Retry semantics: the initial request (status-line exchange) is
//! retried via the configured `AuthProvider` and `RetryPolicy` just
//! like any other request. Once the stream has started producing
//! chunks, transport errors terminate the stream — they are not
//! retried.
//!
//! To adapt to `tokio::io::AsyncRead`, wrap the stream with
//! `tokio_util::io::StreamReader`.

use std::pin::Pin;

/// A stream of [`bytes::Bytes`] chunks delivered by a NiFi binary-download
/// endpoint. Each item is a chunk of the HTTP response body.
pub type BytesStream =
    Pin<Box<dyn futures_core::Stream<Item = Result<bytes::Bytes, crate::NifiError>> + Send>>;

#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::{Bytes, BytesStream, NifiError};
use std::pin::Pin;

#[test]
fn streaming_types_are_public() {
    fn _takes_stream(_s: BytesStream) {}
    fn _takes_bytes(_b: Bytes) {}
    // Confirm the alias resolves to the expected shape.
    fn _equiv() -> BytesStream {
        let _: Pin<Box<dyn futures_core::Stream<Item = Result<Bytes, NifiError>> + Send>>;
        unreachable!()
    }
}

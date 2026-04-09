// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::DataTransferApi;
#[allow(unused_imports)]
use crate::dynamic::traits::DataTransferTransactionsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0DataTransferApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl DataTransferApi for V2_8_0DataTransferApi<'_> {
    fn transactions<'b>(&'b self, port_id: &'b str) -> impl DataTransferTransactionsApi + 'b {
        crate::dynamic::dispatch::DataTransferTransactionsApiDispatch {
            client: self.client,
            port_id: port_id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}

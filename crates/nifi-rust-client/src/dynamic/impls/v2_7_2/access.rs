// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::AccessApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2AccessApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl AccessApi for V2_7_2AccessApi<'_> {
    async fn log_out(&self) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::access::AccessApi {
            client: self.client,
        };
        api.log_out().await
    }
    async fn log_out_complete(&self) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::access::AccessApi {
            client: self.client,
        };
        api.log_out_complete().await
    }
}

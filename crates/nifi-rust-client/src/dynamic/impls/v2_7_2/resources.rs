// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ResourcesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ResourcesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ResourcesApi for V2_7_2ResourcesApi<'_> {
    async fn get_resources(&self) -> Result<types::ResourcesEntity, NifiError> {
        let api = crate::v2_7_2::api::resources::ResourcesApi {
            client: self.client,
        };
        Ok(api.get_resources().await?.into())
    }
}

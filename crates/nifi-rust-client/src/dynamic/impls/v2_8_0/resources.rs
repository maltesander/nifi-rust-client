// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ResourcesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ResourcesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ResourcesApi for V2_8_0ResourcesApi<'_> {
    async fn get_resources(&self) -> Result<types::ResourcesEntity, NifiError> {
        let api = crate::v2_8_0::api::resources::ResourcesApi {
            client: self.client,
        };
        Ok(api.get_resources().await?.into())
    }
}

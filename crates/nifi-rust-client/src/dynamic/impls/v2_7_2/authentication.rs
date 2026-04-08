// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::AuthenticationApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2AuthenticationApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl AuthenticationApi for V2_7_2AuthenticationApi<'_> {
    async fn get_authentication_configuration(
        &self,
    ) -> Result<types::AuthenticationConfigurationDto, NifiError> {
        let api = crate::v2_7_2::api::authentication::AuthenticationApi {
            client: self.client,
        };
        Ok(api.get_authentication_configuration().await?.into())
    }
}

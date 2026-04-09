// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct AuthenticationApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> AuthenticationApi<'a> {
    /// Retrieves the authentication configuration endpoint and status information
    ///
    /// Calls `GET /nifi-api/authentication/configuration`.
    ///
    /// # Permissions
    /// No authentication required.
    pub async fn get_authentication_configuration(
        &self,
    ) -> Result<crate::v2_7_2::types::AuthenticationConfigurationDto, NifiError> {
        let e: crate::v2_7_2::types::AuthenticationConfigurationEntity =
            self.client.get("/authentication/configuration").await?;
        Ok(e.authentication_configuration.unwrap_or_default())
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_7_2::traits::AuthenticationApi for AuthenticationApi<'_> {
    async fn get_authentication_configuration(
        &self,
    ) -> Result<crate::v2_7_2::types::AuthenticationConfigurationDto, NifiError> {
        self.get_authentication_configuration().await
    }
}

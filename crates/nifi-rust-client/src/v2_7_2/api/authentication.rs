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
    pub async fn get_authentication_configuration(
        &self,
    ) -> Result<crate::types::AuthenticationConfigurationDto, NifiError> {
        let e: crate::types::AuthenticationConfigurationEntity =
            self.client.get("/authentication/configuration").await?;
        Ok(e.authentication_configuration)
    }
}

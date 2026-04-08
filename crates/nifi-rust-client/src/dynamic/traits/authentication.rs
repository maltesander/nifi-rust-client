// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Authentication API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait AuthenticationApi {
    /// Retrieves the authentication configuration endpoint and status information
    ///
    /// Calls `GET /nifi-api/authentication/configuration`.
    async fn get_authentication_configuration(
        &self,
    ) -> Result<types::AuthenticationConfigurationDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_authentication_configuration".to_string(),
            version: "unknown".to_string(),
        })
    }
}

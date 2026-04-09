// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Authentication API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait AuthenticationApi {
    /// Retrieves the authentication configuration endpoint and status information
    async fn get_authentication_configuration(
        &self,
    ) -> Result<crate::v2_7_2::types::AuthenticationConfigurationDto, NifiError>;
}

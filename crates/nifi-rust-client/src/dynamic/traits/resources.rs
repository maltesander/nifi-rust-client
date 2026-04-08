// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Resources API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ResourcesApi {
    /// Gets the available resources that support access/authorization policies
    ///
    /// Calls `GET /nifi-api/resources`.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    ///
    /// # Permissions
    /// Requires `Read - /resources`.
    async fn get_resources(&self) -> Result<types::ResourcesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_resources".to_string(),
            version: "unknown".to_string(),
        })
    }
}

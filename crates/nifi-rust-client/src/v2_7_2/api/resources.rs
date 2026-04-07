// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ResourcesApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ResourcesApi<'a> {
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
    pub async fn get_resources(&self) -> Result<crate::v2_7_2::types::ResourcesEntity, NifiError> {
        self.client.get("/resources").await
    }
}

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
    pub async fn get_resources(&self) -> Result<crate::v2_6_0::types::ResourcesEntity, NifiError> {
        self.client.get("/resources").await
    }
}

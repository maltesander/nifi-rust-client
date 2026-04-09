// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Connections API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ConnectionsApi {
    /// Deletes a connection
    async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ConnectionEntity, NifiError>;
    /// Gets a connection
    async fn get_connection(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ConnectionEntity, NifiError>;
    /// Updates a connection
    async fn update_connection(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ConnectionEntity,
    ) -> Result<crate::v2_6_0::types::ConnectionEntity, NifiError>;
}

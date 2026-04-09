// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Funnels API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FunnelsApi {
    /// Deletes a funnel
    async fn remove_funnel(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::FunnelEntity, NifiError>;
    /// Gets a funnel
    async fn get_funnel(&self, id: &str) -> Result<crate::v2_6_0::types::FunnelEntity, NifiError>;
    /// Updates a funnel
    async fn update_funnel(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::FunnelEntity,
    ) -> Result<crate::v2_6_0::types::FunnelEntity, NifiError>;
}

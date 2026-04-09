// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Labels API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait LabelsApi {
    /// Deletes a label
    async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::LabelEntity, NifiError>;
    /// Gets a label
    async fn get_label(&self, id: &str) -> Result<crate::v2_8_0::types::LabelEntity, NifiError>;
    /// Updates a label
    async fn update_label(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::LabelEntity,
    ) -> Result<crate::v2_8_0::types::LabelEntity, NifiError>;
}

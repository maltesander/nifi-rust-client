// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `download` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait VersionsDownloadApi {
    /// Gets the latest version of a Process Group for download
    async fn export_flow_version(&self) -> Result<(), NifiError>;
}
/// The Versions API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait VersionsApi {
    fn download<'b>(&'b self, id: &'b str) -> impl VersionsDownloadApi + 'b;
    /// Create a version control request
    async fn create_version_control_request(
        &self,
        body: &crate::v2_6_0::types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError>;
    /// Deletes the version control request with the given ID
    async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError>;
    /// Updates the request with the given ID
    async fn update_version_control_request(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::VersionControlComponentMappingEntity,
    ) -> Result<crate::v2_6_0::types::VersionControlInformationEntity, NifiError>;
    /// Stops version controlling the Process Group with the given ID
    async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::VersionControlInformationEntity, NifiError>;
    /// Gets the Version Control information for a process group
    async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::VersionControlInformationEntity, NifiError>;
    /// Save the Process Group with the given ID
    async fn save_to_flow_registry(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::StartVersionControlRequestEntity,
    ) -> Result<crate::v2_6_0::types::VersionControlInformationEntity, NifiError>;
    /// Update the version of a Process Group with the given ID
    async fn update_flow_version(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::VersionedFlowSnapshotEntity,
    ) -> Result<crate::v2_6_0::types::VersionControlInformationEntity, NifiError>;
    /// Initiate the Revert Request of a Process Group with the given ID
    async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
    /// Deletes the Revert Request with the given ID
    async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
    /// Returns the Revert Request with the given ID
    async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
    /// Initiate the Update Request of a Process Group with the given ID
    async fn initiate_version_control_update(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
    /// Deletes the Update Request with the given ID
    async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
    /// Returns the Update Request with the given ID
    async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::VersionedFlowUpdateRequestEntity, NifiError>;
}

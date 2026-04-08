// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::VersionsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Versions API. Use via the [`VersionsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum VersionsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0VersionsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2VersionsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0VersionsApi<'a>),
}
impl VersionsApi for VersionsApiDispatch<'_> {
    async fn create_version_control_request(
        &self,
        body: types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_version_control_request(body).await,
            Self::V2_7_2(api) => api.create_version_control_request(body).await,
            Self::V2_8_0(api) => api.create_version_control_request(body).await,
        }
    }
    async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_revert_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_revert_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_revert_request(id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_update_request_1(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_update_request_1(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_update_request_1(id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn export_flow_version(&self, id: &str) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.export_flow_version(id).await,
            Self::V2_7_2(api) => api.export_flow_version(id).await,
            Self::V2_8_0(api) => api.export_flow_version(id).await,
        }
    }
    async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_revert_request(id).await,
            Self::V2_7_2(api) => api.get_revert_request(id).await,
            Self::V2_8_0(api) => api.get_revert_request(id).await,
        }
    }
    async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_update_request(id).await,
            Self::V2_7_2(api) => api.get_update_request(id).await,
            Self::V2_8_0(api) => api.get_update_request(id).await,
        }
    }
    async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_version_information(id).await,
            Self::V2_7_2(api) => api.get_version_information(id).await,
            Self::V2_8_0(api) => api.get_version_information(id).await,
        }
    }
    async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.initiate_revert_flow_version(id, body).await,
            Self::V2_7_2(api) => api.initiate_revert_flow_version(id, body).await,
            Self::V2_8_0(api) => api.initiate_revert_flow_version(id, body).await,
        }
    }
    async fn initiate_version_control_update(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.initiate_version_control_update(id, body).await,
            Self::V2_7_2(api) => api.initiate_version_control_update(id, body).await,
            Self::V2_8_0(api) => api.initiate_version_control_update(id, body).await,
        }
    }
    async fn save_to_flow_registry(
        &self,
        id: &str,
        body: types::StartVersionControlRequestEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.save_to_flow_registry(id, body).await,
            Self::V2_7_2(api) => api.save_to_flow_registry(id, body).await,
            Self::V2_8_0(api) => api.save_to_flow_registry(id, body).await,
        }
    }
    async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_flow_version(
        &self,
        id: &str,
        body: types::VersionedFlowSnapshotEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_flow_version(id, body).await,
            Self::V2_7_2(api) => api.update_flow_version(id, body).await,
            Self::V2_8_0(api) => api.update_flow_version(id, body).await,
        }
    }
    async fn update_version_control_request(
        &self,
        id: &str,
        body: types::VersionControlComponentMappingEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_version_control_request(id, body).await,
            Self::V2_7_2(api) => api.update_version_control_request(id, body).await,
            Self::V2_8_0(api) => api.update_version_control_request(id, body).await,
        }
    }
}

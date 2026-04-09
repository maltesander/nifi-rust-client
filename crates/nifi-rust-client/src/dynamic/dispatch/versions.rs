// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::VersionsApi;
use crate::dynamic::traits::VersionsDownloadApi;
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
impl<'a> VersionsApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl VersionsApi for VersionsApiDispatch<'_> {
    type VersionsDownloadApi<'b>
        = VersionsDownloadApiDispatch<'b>
    where
        Self: 'b;
    fn download<'b>(&'b self, id: &'b str) -> Self::VersionsDownloadApi<'b> {
        VersionsDownloadApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn create_version_control_request(
        &self,
        body: &types::CreateActiveRequestEntity,
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
        body: &types::VersionControlInformationEntity,
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
        body: &types::VersionControlInformationEntity,
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
        body: &types::StartVersionControlRequestEntity,
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
        body: &types::VersionedFlowSnapshotEntity,
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
        body: &types::VersionControlComponentMappingEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_version_control_request(id, body).await,
            Self::V2_7_2(api) => api.update_version_control_request(id, body).await,
            Self::V2_8_0(api) => api.update_version_control_request(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [VersionsDownloadApi].
pub struct VersionsDownloadApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl VersionsDownloadApi for VersionsDownloadApiDispatch<'_> {
    async fn export_flow_version(&self) -> Result<(), NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_flow_version().await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_flow_version().await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_flow_version().await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "export_flow_version".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

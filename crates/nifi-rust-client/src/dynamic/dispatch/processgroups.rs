// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessGroupsApi;
use crate::dynamic::traits::ProcessGroupsConnectionsApi;
use crate::dynamic::traits::ProcessGroupsControllerServicesApi;
use crate::dynamic::traits::ProcessGroupsCopyApi;
use crate::dynamic::traits::ProcessGroupsDownloadApi;
use crate::dynamic::traits::ProcessGroupsEmptyAllConnectionsRequestsApi;
use crate::dynamic::traits::ProcessGroupsFlowContentsApi;
use crate::dynamic::traits::ProcessGroupsFunnelsApi;
use crate::dynamic::traits::ProcessGroupsInputPortsApi;
use crate::dynamic::traits::ProcessGroupsLabelsApi;
use crate::dynamic::traits::ProcessGroupsLocalModificationsApi;
use crate::dynamic::traits::ProcessGroupsOutputPortsApi;
use crate::dynamic::traits::ProcessGroupsPasteApi;
use crate::dynamic::traits::ProcessGroupsProcessGroupsApi;
use crate::dynamic::traits::ProcessGroupsProcessorsApi;
use crate::dynamic::traits::ProcessGroupsRemoteProcessGroupsApi;
use crate::dynamic::traits::ProcessGroupsReplaceRequestsApi;
use crate::dynamic::traits::ProcessGroupsSnippetInstanceApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ProcessGroups API. Use via the [`ProcessGroupsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ProcessGroupsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ProcessGroupsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ProcessGroupsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ProcessGroupsApi<'a>),
}
impl<'a> ProcessGroupsApiDispatch<'a> {
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
impl ProcessGroupsApi for ProcessGroupsApiDispatch<'_> {
    fn connections<'b>(&'b self, id: &'b str) -> impl ProcessGroupsConnectionsApi + 'b {
        ProcessGroupsConnectionsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsControllerServicesApi + 'b {
        ProcessGroupsControllerServicesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn copy<'b>(&'b self, id: &'b str) -> impl ProcessGroupsCopyApi + 'b {
        ProcessGroupsCopyApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn download<'b>(&'b self, id: &'b str) -> impl ProcessGroupsDownloadApi + 'b {
        ProcessGroupsDownloadApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsEmptyAllConnectionsRequestsApi + 'b {
        ProcessGroupsEmptyAllConnectionsRequestsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn flow_contents<'b>(&'b self, id: &'b str) -> impl ProcessGroupsFlowContentsApi + 'b {
        ProcessGroupsFlowContentsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn funnels<'b>(&'b self, id: &'b str) -> impl ProcessGroupsFunnelsApi + 'b {
        ProcessGroupsFunnelsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn input_ports<'b>(&'b self, id: &'b str) -> impl ProcessGroupsInputPortsApi + 'b {
        ProcessGroupsInputPortsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn labels<'b>(&'b self, id: &'b str) -> impl ProcessGroupsLabelsApi + 'b {
        ProcessGroupsLabelsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsLocalModificationsApi + 'b {
        ProcessGroupsLocalModificationsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn output_ports<'b>(&'b self, id: &'b str) -> impl ProcessGroupsOutputPortsApi + 'b {
        ProcessGroupsOutputPortsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn paste<'b>(&'b self, id: &'b str) -> impl ProcessGroupsPasteApi + 'b {
        ProcessGroupsPasteApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn process_groups<'b>(&'b self, id: &'b str) -> impl ProcessGroupsProcessGroupsApi + 'b {
        ProcessGroupsProcessGroupsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn processors<'b>(&'b self, id: &'b str) -> impl ProcessGroupsProcessorsApi + 'b {
        ProcessGroupsProcessorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsRemoteProcessGroupsApi + 'b {
        ProcessGroupsRemoteProcessGroupsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn replace_requests<'b>(&'b self, id: &'b str) -> impl ProcessGroupsReplaceRequestsApi + 'b {
        ProcessGroupsReplaceRequestsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn snippet_instance<'b>(&'b self, id: &'b str) -> impl ProcessGroupsSnippetInstanceApi + 'b {
        ProcessGroupsSnippetInstanceApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_process_group(id).await,
            Self::V2_7_2(api) => api.get_process_group(id).await,
            Self::V2_8_0(api) => api.get_process_group(id).await,
        }
    }
    async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_replace_process_group_request(id).await,
            Self::V2_7_2(api) => api.get_replace_process_group_request(id).await,
            Self::V2_8_0(api) => api.get_replace_process_group_request(id).await,
        }
    }
    async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_process_group(
        &self,
        id: &str,
        body: &types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_process_group(id, body).await,
            Self::V2_7_2(api) => api.update_process_group(id, body).await,
            Self::V2_8_0(api) => api.update_process_group(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsConnectionsApi].
pub struct ProcessGroupsConnectionsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsConnectionsApi for ProcessGroupsConnectionsApiDispatch<'_> {
    async fn create_connection(
        &self,
        body: &types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_connection(&crate::v2_6_0::types::ConnectionEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_connection(&crate::v2_7_2::types::ConnectionEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_connection(&crate::v2_8_0::types::ConnectionEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_connection".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_connections(&self) -> Result<types::ConnectionsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connections().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connections().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connections().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_connections".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsControllerServicesApi].
pub struct ProcessGroupsControllerServicesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsControllerServicesApi for ProcessGroupsControllerServicesApiDispatch<'_> {
    async fn create_controller_service_1(
        &self,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_6_0::types::ControllerServiceEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_7_2::types::ControllerServiceEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_8_0::types::ControllerServiceEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_controller_service_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsCopyApi].
pub struct ProcessGroupsCopyApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsCopyApi for ProcessGroupsCopyApiDispatch<'_> {
    async fn copy(
        &self,
        body: &types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy(&crate::v2_6_0::types::CopyRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy(&crate::v2_7_2::types::CopyRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy(&crate::v2_8_0::types::CopyRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "copy".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsDownloadApi].
pub struct ProcessGroupsDownloadApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsDownloadApi for ProcessGroupsDownloadApiDispatch<'_> {
    async fn export_process_group(
        &self,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_process_group(include_referenced_services).await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_process_group(include_referenced_services).await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id: &self.id,
                };
                api.export_process_group(include_referenced_services).await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "export_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsEmptyAllConnectionsRequestsApi].
pub struct ProcessGroupsEmptyAllConnectionsRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsEmptyAllConnectionsRequestsApi
    for ProcessGroupsEmptyAllConnectionsRequestsApiDispatch<'_>
{
    async fn create_empty_all_connections_request(
        &self,
    ) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_empty_all_connections_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_drop_all_flowfiles_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_drop_all_flowfiles_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn remove_drop_request_1(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "remove_drop_request_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsFlowContentsApi].
pub struct ProcessGroupsFlowContentsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsFlowContentsApi for ProcessGroupsFlowContentsApiDispatch<'_> {
    async fn replace_process_group(
        &self,
        body: &types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_6_0::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_7_2::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_8_0::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "replace_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsFunnelsApi].
pub struct ProcessGroupsFunnelsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsFunnelsApi for ProcessGroupsFunnelsApiDispatch<'_> {
    async fn create_funnel(
        &self,
        body: &types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_funnel(&crate::v2_6_0::types::FunnelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_funnel(&crate::v2_7_2::types::FunnelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_funnel(&crate::v2_8_0::types::FunnelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_funnel".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_funnels(&self) -> Result<types::FunnelsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_funnels().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_funnels().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_funnels().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_funnels".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsInputPortsApi].
pub struct ProcessGroupsInputPortsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsInputPortsApi for ProcessGroupsInputPortsApiDispatch<'_> {
    async fn create_input_port(
        &self,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_input_port(&crate::v2_6_0::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_input_port(&crate::v2_7_2::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_input_port(&crate::v2_8_0::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_input_port".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_input_ports(&self) -> Result<types::InputPortsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_input_ports().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_input_ports().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_input_ports().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_input_ports".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsLabelsApi].
pub struct ProcessGroupsLabelsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsLabelsApi for ProcessGroupsLabelsApiDispatch<'_> {
    async fn create_label(
        &self,
        body: &types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_label(&crate::v2_6_0::types::LabelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_label(&crate::v2_7_2::types::LabelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_label(&crate::v2_8_0::types::LabelEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_label".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_labels(&self) -> Result<types::LabelsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_labels().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_labels().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_labels().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_labels".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsLocalModificationsApi].
pub struct ProcessGroupsLocalModificationsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsLocalModificationsApi for ProcessGroupsLocalModificationsApiDispatch<'_> {
    async fn get_local_modifications(&self) -> Result<types::FlowComparisonEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_local_modifications".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsOutputPortsApi].
pub struct ProcessGroupsOutputPortsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsOutputPortsApi for ProcessGroupsOutputPortsApiDispatch<'_> {
    async fn create_output_port(
        &self,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_output_port(&crate::v2_6_0::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_output_port(&crate::v2_7_2::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_output_port(&crate::v2_8_0::types::PortEntity::try_from(body.clone())?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_output_port".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_output_ports(&self) -> Result<types::OutputPortsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_output_ports().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_output_ports().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_output_ports().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_output_ports".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsPasteApi].
pub struct ProcessGroupsPasteApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsPasteApi for ProcessGroupsPasteApiDispatch<'_> {
    async fn paste(
        &self,
        body: &types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .paste(&crate::v2_6_0::types::PasteRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .paste(&crate::v2_7_2::types::PasteRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .paste(&crate::v2_8_0::types::PasteRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "paste".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsProcessGroupsApi].
pub struct ProcessGroupsProcessGroupsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsProcessGroupsApi for ProcessGroupsProcessGroupsApiDispatch<'_> {
    async fn create_process_group(
        &self,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: &types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_6_0::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_6_0::types::ProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_7_2::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_7_2::types::ProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_8_0::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_8_0::types::ProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_process_groups(&self) -> Result<types::ProcessGroupsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_groups().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_groups().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_groups().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_process_groups".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn import_process_group(
        &self,
        body: &types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_6_0::types::ProcessGroupUploadEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_7_2::types::ProcessGroupUploadEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_8_0::types::ProcessGroupUploadEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "import_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn upload_process_group(&self) -> Result<types::ProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.upload_process_group().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.upload_process_group().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.upload_process_group().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "upload_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsProcessorsApi].
pub struct ProcessGroupsProcessorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsProcessorsApi for ProcessGroupsProcessorsApiDispatch<'_> {
    async fn create_processor(
        &self,
        body: &types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_processor(&crate::v2_6_0::types::ProcessorEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_processor(&crate::v2_7_2::types::ProcessorEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_processor(&crate::v2_8_0::types::ProcessorEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_processor".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_processors(
        &self,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_processors".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsRemoteProcessGroupsApi].
pub struct ProcessGroupsRemoteProcessGroupsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsRemoteProcessGroupsApi for ProcessGroupsRemoteProcessGroupsApiDispatch<'_> {
    async fn create_remote_process_group(
        &self,
        body: &types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_6_0::types::RemoteProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_7_2::types::RemoteProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_8_0::types::RemoteProcessGroupEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_remote_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_remote_process_groups(
        &self,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_remote_process_groups".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsReplaceRequestsApi].
pub struct ProcessGroupsReplaceRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsReplaceRequestsApi for ProcessGroupsReplaceRequestsApiDispatch<'_> {
    async fn initiate_replace_process_group(
        &self,
        body: &types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_6_0::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_7_2::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_8_0::types::ProcessGroupImportEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "initiate_replace_process_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessGroupsSnippetInstanceApi].
pub struct ProcessGroupsSnippetInstanceApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessGroupsSnippetInstanceApi for ProcessGroupsSnippetInstanceApiDispatch<'_> {
    async fn copy_snippet(
        &self,
        body: &types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_6_0::types::CopySnippetRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_7_2::types::CopySnippetRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_8_0::types::CopySnippetRequestEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "copy_snippet".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

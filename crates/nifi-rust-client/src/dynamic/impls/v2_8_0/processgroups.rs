// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsConnectionsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsControllerServicesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsCopyApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsDownloadApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsEmptyAllConnectionsRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsFlowContentsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsFunnelsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsInputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsLabelsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsLocalModificationsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsOutputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsPasteApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsProcessorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsRemoteProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsReplaceRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessGroupsSnippetInstanceApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ProcessGroupsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProcessGroupsApi for V2_8_0ProcessGroupsApi<'_> {
    type ProcessGroupsConnectionsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsConnectionsApiDispatch<'b>
    where
        Self: 'b;
    fn connections<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsConnectionsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsConnectionsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsControllerServicesApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsControllerServicesApiDispatch<'b>
    where
        Self: 'b;
    fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsControllerServicesApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsControllerServicesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsCopyApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsCopyApiDispatch<'b>
    where
        Self: 'b;
    fn copy<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsCopyApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsCopyApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsDownloadApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsDownloadApiDispatch<'b>
    where
        Self: 'b;
    fn download<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsDownloadApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsDownloadApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsEmptyAllConnectionsRequestsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsEmptyAllConnectionsRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsEmptyAllConnectionsRequestsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsEmptyAllConnectionsRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsFlowContentsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsFlowContentsApiDispatch<'b>
    where
        Self: 'b;
    fn flow_contents<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFlowContentsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsFlowContentsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsFunnelsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsFunnelsApiDispatch<'b>
    where
        Self: 'b;
    fn funnels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFunnelsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsFunnelsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsInputPortsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsInputPortsApiDispatch<'b>
    where
        Self: 'b;
    fn input_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsInputPortsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsInputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsLabelsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsLabelsApiDispatch<'b>
    where
        Self: 'b;
    fn labels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsLabelsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsLabelsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsLocalModificationsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsLocalModificationsApiDispatch<'b>
    where
        Self: 'b;
    fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsLocalModificationsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsLocalModificationsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsOutputPortsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsOutputPortsApiDispatch<'b>
    where
        Self: 'b;
    fn output_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsOutputPortsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsOutputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsPasteApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsPasteApiDispatch<'b>
    where
        Self: 'b;
    fn paste<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsPasteApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsPasteApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsProcessGroupsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsProcessGroupsApiDispatch<'b>
    where
        Self: 'b;
    fn process_groups<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessGroupsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsProcessGroupsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsProcessorsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsProcessorsApiDispatch<'b>
    where
        Self: 'b;
    fn processors<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessorsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsProcessorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsRemoteProcessGroupsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsRemoteProcessGroupsApiDispatch<'b>
    where
        Self: 'b;
    fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsRemoteProcessGroupsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsRemoteProcessGroupsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsReplaceRequestsApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsReplaceRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn replace_requests<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsReplaceRequestsApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsReplaceRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ProcessGroupsSnippetInstanceApi<'b>
        = crate::dynamic::dispatch::ProcessGroupsSnippetInstanceApiDispatch<'b>
    where
        Self: 'b;
    fn snippet_instance<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsSnippetInstanceApi<'b> {
        crate::dynamic::dispatch::ProcessGroupsSnippetInstanceApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .delete_replace_process_group_request(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_process_group(id).await?.into())
    }
    async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_replace_process_group_request(id).await?.into())
    }
    async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .remove_process_group(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_process_group(
        &self,
        id: &str,
        body: &types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .update_process_group(
                id,
                &crate::v2_8_0::types::ProcessGroupEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

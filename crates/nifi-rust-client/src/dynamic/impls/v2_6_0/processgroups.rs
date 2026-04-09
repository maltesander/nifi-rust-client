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
pub(crate) struct V2_6_0ProcessGroupsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProcessGroupsApi for V2_6_0ProcessGroupsApi<'_> {
    fn connections<'b>(&'b self, id: &'b str) -> impl ProcessGroupsConnectionsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsConnectionsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsControllerServicesApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsControllerServicesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn copy<'b>(&'b self, id: &'b str) -> impl ProcessGroupsCopyApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsCopyApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn download<'b>(&'b self, id: &'b str) -> impl ProcessGroupsDownloadApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsDownloadApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsEmptyAllConnectionsRequestsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsEmptyAllConnectionsRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn flow_contents<'b>(&'b self, id: &'b str) -> impl ProcessGroupsFlowContentsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsFlowContentsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn funnels<'b>(&'b self, id: &'b str) -> impl ProcessGroupsFunnelsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsFunnelsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn input_ports<'b>(&'b self, id: &'b str) -> impl ProcessGroupsInputPortsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsInputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn labels<'b>(&'b self, id: &'b str) -> impl ProcessGroupsLabelsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsLabelsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsLocalModificationsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsLocalModificationsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn output_ports<'b>(&'b self, id: &'b str) -> impl ProcessGroupsOutputPortsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsOutputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn paste<'b>(&'b self, id: &'b str) -> impl ProcessGroupsPasteApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsPasteApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn process_groups<'b>(&'b self, id: &'b str) -> impl ProcessGroupsProcessGroupsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsProcessGroupsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn processors<'b>(&'b self, id: &'b str) -> impl ProcessGroupsProcessorsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsProcessorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> impl ProcessGroupsRemoteProcessGroupsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsRemoteProcessGroupsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn replace_requests<'b>(&'b self, id: &'b str) -> impl ProcessGroupsReplaceRequestsApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsReplaceRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    fn snippet_instance<'b>(&'b self, id: &'b str) -> impl ProcessGroupsSnippetInstanceApi + 'b {
        crate::dynamic::dispatch::ProcessGroupsSnippetInstanceApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .delete_replace_process_group_request(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_process_group(id).await?.into())
    }
    async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
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
        let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
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
        let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .update_process_group(
                id,
                &crate::v2_6_0::types::ProcessGroupEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

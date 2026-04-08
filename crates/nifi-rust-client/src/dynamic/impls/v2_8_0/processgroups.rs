// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ProcessGroupsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProcessGroupsApi for V2_8_0ProcessGroupsApi<'_> {
    async fn copy(
        &self,
        id: &str,
        body: types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsCopyApi {
            client: self.client,
            id,
        };
        Ok(api
            .copy(&crate::v2_8_0::types::CopyRequestEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn copy_snippet(
        &self,
        id: &str,
        body: types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsSnippetInstanceApi {
            client: self.client,
            id,
        };
        Ok(api
            .copy_snippet(&crate::v2_8_0::types::CopySnippetRequestEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_connection(&crate::v2_8_0::types::ConnectionEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_controller_service_1(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsControllerServicesApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_controller_service_1(&crate::v2_8_0::types::ControllerServiceEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_empty_all_connections_request(
        &self,
        id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.create_empty_all_connections_request().await?.into())
    }
    async fn create_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_funnel(&crate::v2_8_0::types::FunnelEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_input_port(&crate::v2_8_0::types::PortEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_label(
        &self,
        id: &str,
        body: types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_label(&crate::v2_8_0::types::LabelEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_output_port(&crate::v2_8_0::types::PortEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_process_group(
        &self,
        id: &str,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_process_group(
                parameter_context_handling_strategy
                    .map(crate::v2_8_0::types::ParameterContextHandlingStrategy::try_from)
                    .transpose()?,
                &crate::v2_8_0::types::ProcessGroupEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn create_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_processor(&crate::v2_8_0::types::ProcessorEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api
            .create_remote_process_group(&crate::v2_8_0::types::RemoteProcessGroupEntity::try_from(
                body,
            )?)
            .await?
            .into())
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
    async fn export_process_group(
        &self,
        id: &str,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsDownloadApi {
            client: self.client,
            id,
        };
        api.export_process_group(include_referenced_services).await
    }
    async fn get_connections(&self, id: &str) -> Result<types::ConnectionsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
            client: self.client,
            id,
        };
        Ok(api.get_connections().await?.into())
    }
    async fn get_drop_all_flowfiles_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_drop_all_flowfiles_request(drop_request_id)
            .await?
            .into())
    }
    async fn get_funnels(&self, id: &str) -> Result<types::FunnelsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
            client: self.client,
            id,
        };
        Ok(api.get_funnels().await?.into())
    }
    async fn get_input_ports(&self, id: &str) -> Result<types::InputPortsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
            client: self.client,
            id,
        };
        Ok(api.get_input_ports().await?.into())
    }
    async fn get_labels(&self, id: &str) -> Result<types::LabelsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
            client: self.client,
            id,
        };
        Ok(api.get_labels().await?.into())
    }
    async fn get_local_modifications(
        &self,
        id: &str,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsLocalModificationsApi {
            client: self.client,
            id,
        };
        Ok(api.get_local_modifications().await?.into())
    }
    async fn get_output_ports(&self, id: &str) -> Result<types::OutputPortsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        };
        Ok(api.get_output_ports().await?.into())
    }
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_process_group(id).await?.into())
    }
    async fn get_process_groups(&self, id: &str) -> Result<types::ProcessGroupsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api.get_process_groups().await?.into())
    }
    async fn get_processors(
        &self,
        id: &str,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
            client: self.client,
            id,
        };
        Ok(api.get_processors(include_descendant_groups).await?.into())
    }
    async fn get_remote_process_groups(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api.get_remote_process_groups().await?.into())
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
    async fn import_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api
            .import_process_group(&crate::v2_8_0::types::ProcessGroupUploadEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn initiate_replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsReplaceRequestsApi {
            client: self.client,
            id,
        };
        Ok(api
            .initiate_replace_process_group(
                &crate::v2_8_0::types::ProcessGroupImportEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn paste(
        &self,
        id: &str,
        body: types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsPasteApi {
            client: self.client,
            id,
        };
        Ok(api
            .paste(&crate::v2_8_0::types::PasteRequestEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn remove_drop_request_1(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.remove_drop_request_1(drop_request_id).await?.into())
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
    async fn replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsFlowContentsApi {
            client: self.client,
            id,
        };
        Ok(api
            .replace_process_group(&crate::v2_8_0::types::ProcessGroupImportEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn update_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .update_process_group(
                id,
                &crate::v2_8_0::types::ProcessGroupEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn upload_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
            client: self.client,
            id,
        };
        Ok(api.upload_process_group().await?.into())
    }
}

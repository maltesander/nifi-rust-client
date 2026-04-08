// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessGroupsApi;
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
impl ProcessGroupsApi for ProcessGroupsApiDispatch<'_> {
    async fn copy(
        &self,
        id: &str,
        body: types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.copy(id, body).await,
            Self::V2_7_2(api) => api.copy(id, body).await,
            Self::V2_8_0(api) => api.copy(id, body).await,
        }
    }
    async fn copy_snippet(
        &self,
        id: &str,
        body: types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.copy_snippet(id, body).await,
            Self::V2_7_2(api) => api.copy_snippet(id, body).await,
            Self::V2_8_0(api) => api.copy_snippet(id, body).await,
        }
    }
    async fn create_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_connection(id, body).await,
            Self::V2_7_2(api) => api.create_connection(id, body).await,
            Self::V2_8_0(api) => api.create_connection(id, body).await,
        }
    }
    async fn create_controller_service_1(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_controller_service_1(id, body).await,
            Self::V2_7_2(api) => api.create_controller_service_1(id, body).await,
            Self::V2_8_0(api) => api.create_controller_service_1(id, body).await,
        }
    }
    async fn create_empty_all_connections_request(
        &self,
        id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_empty_all_connections_request(id).await,
            Self::V2_7_2(api) => api.create_empty_all_connections_request(id).await,
            Self::V2_8_0(api) => api.create_empty_all_connections_request(id).await,
        }
    }
    async fn create_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_funnel(id, body).await,
            Self::V2_7_2(api) => api.create_funnel(id, body).await,
            Self::V2_8_0(api) => api.create_funnel(id, body).await,
        }
    }
    async fn create_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_input_port(id, body).await,
            Self::V2_7_2(api) => api.create_input_port(id, body).await,
            Self::V2_8_0(api) => api.create_input_port(id, body).await,
        }
    }
    async fn create_label(
        &self,
        id: &str,
        body: types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_label(id, body).await,
            Self::V2_7_2(api) => api.create_label(id, body).await,
            Self::V2_8_0(api) => api.create_label(id, body).await,
        }
    }
    async fn create_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_output_port(id, body).await,
            Self::V2_7_2(api) => api.create_output_port(id, body).await,
            Self::V2_8_0(api) => api.create_output_port(id, body).await,
        }
    }
    async fn create_process_group(
        &self,
        id: &str,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.create_process_group(id, parameter_context_handling_strategy, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.create_process_group(id, parameter_context_handling_strategy, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.create_process_group(id, parameter_context_handling_strategy, body)
                    .await
            }
        }
    }
    async fn create_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_processor(id, body).await,
            Self::V2_7_2(api) => api.create_processor(id, body).await,
            Self::V2_8_0(api) => api.create_processor(id, body).await,
        }
    }
    async fn create_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_remote_process_group(id, body).await,
            Self::V2_7_2(api) => api.create_remote_process_group(id, body).await,
            Self::V2_8_0(api) => api.create_remote_process_group(id, body).await,
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
    async fn export_process_group(
        &self,
        id: &str,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.export_process_group(id, include_referenced_services)
                    .await
            }
            Self::V2_7_2(api) => {
                api.export_process_group(id, include_referenced_services)
                    .await
            }
            Self::V2_8_0(api) => {
                api.export_process_group(id, include_referenced_services)
                    .await
            }
        }
    }
    async fn get_connections(&self, id: &str) -> Result<types::ConnectionsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_connections(id).await,
            Self::V2_7_2(api) => api.get_connections(id).await,
            Self::V2_8_0(api) => api.get_connections(id).await,
        }
    }
    async fn get_drop_all_flowfiles_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_drop_all_flowfiles_request(id, drop_request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_drop_all_flowfiles_request(id, drop_request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_drop_all_flowfiles_request(id, drop_request_id)
                    .await
            }
        }
    }
    async fn get_funnels(&self, id: &str) -> Result<types::FunnelsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_funnels(id).await,
            Self::V2_7_2(api) => api.get_funnels(id).await,
            Self::V2_8_0(api) => api.get_funnels(id).await,
        }
    }
    async fn get_input_ports(&self, id: &str) -> Result<types::InputPortsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_input_ports(id).await,
            Self::V2_7_2(api) => api.get_input_ports(id).await,
            Self::V2_8_0(api) => api.get_input_ports(id).await,
        }
    }
    async fn get_labels(&self, id: &str) -> Result<types::LabelsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_labels(id).await,
            Self::V2_7_2(api) => api.get_labels(id).await,
            Self::V2_8_0(api) => api.get_labels(id).await,
        }
    }
    async fn get_local_modifications(
        &self,
        id: &str,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_local_modifications(id).await,
            Self::V2_7_2(api) => api.get_local_modifications(id).await,
            Self::V2_8_0(api) => api.get_local_modifications(id).await,
        }
    }
    async fn get_output_ports(&self, id: &str) -> Result<types::OutputPortsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_output_ports(id).await,
            Self::V2_7_2(api) => api.get_output_ports(id).await,
            Self::V2_8_0(api) => api.get_output_ports(id).await,
        }
    }
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_process_group(id).await,
            Self::V2_7_2(api) => api.get_process_group(id).await,
            Self::V2_8_0(api) => api.get_process_group(id).await,
        }
    }
    async fn get_process_groups(&self, id: &str) -> Result<types::ProcessGroupsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_process_groups(id).await,
            Self::V2_7_2(api) => api.get_process_groups(id).await,
            Self::V2_8_0(api) => api.get_process_groups(id).await,
        }
    }
    async fn get_processors(
        &self,
        id: &str,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processors(id, include_descendant_groups).await,
            Self::V2_7_2(api) => api.get_processors(id, include_descendant_groups).await,
            Self::V2_8_0(api) => api.get_processors(id, include_descendant_groups).await,
        }
    }
    async fn get_remote_process_groups(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_remote_process_groups(id).await,
            Self::V2_7_2(api) => api.get_remote_process_groups(id).await,
            Self::V2_8_0(api) => api.get_remote_process_groups(id).await,
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
    async fn import_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.import_process_group(id, body).await,
            Self::V2_7_2(api) => api.import_process_group(id, body).await,
            Self::V2_8_0(api) => api.import_process_group(id, body).await,
        }
    }
    async fn initiate_replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.initiate_replace_process_group(id, body).await,
            Self::V2_7_2(api) => api.initiate_replace_process_group(id, body).await,
            Self::V2_8_0(api) => api.initiate_replace_process_group(id, body).await,
        }
    }
    async fn paste(
        &self,
        id: &str,
        body: types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.paste(id, body).await,
            Self::V2_7_2(api) => api.paste(id, body).await,
            Self::V2_8_0(api) => api.paste(id, body).await,
        }
    }
    async fn remove_drop_request_1(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.remove_drop_request_1(id, drop_request_id).await,
            Self::V2_7_2(api) => api.remove_drop_request_1(id, drop_request_id).await,
            Self::V2_8_0(api) => api.remove_drop_request_1(id, drop_request_id).await,
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
    async fn replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.replace_process_group(id, body).await,
            Self::V2_7_2(api) => api.replace_process_group(id, body).await,
            Self::V2_8_0(api) => api.replace_process_group(id, body).await,
        }
    }
    async fn update_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_process_group(id, body).await,
            Self::V2_7_2(api) => api.update_process_group(id, body).await,
            Self::V2_8_0(api) => api.update_process_group(id, body).await,
        }
    }
    async fn upload_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.upload_process_group(id).await,
            Self::V2_7_2(api) => api.upload_process_group(id).await,
            Self::V2_8_0(api) => api.upload_process_group(id).await,
        }
    }
}

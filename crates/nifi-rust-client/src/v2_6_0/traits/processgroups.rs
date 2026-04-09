// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `connections` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsConnectionsApi {
    /// Gets all connections
    async fn get_connections(&self) -> Result<crate::v2_6_0::types::ConnectionsEntity, NifiError>;
    /// Creates a connection
    async fn create_connection(
        &self,
        body: &crate::v2_6_0::types::ConnectionEntity,
    ) -> Result<crate::v2_6_0::types::ConnectionEntity, NifiError>;
}
/// Sub-resource trait for the `controller_services` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsControllerServicesApi {
    /// Creates a new controller service
    async fn create_controller_service_1(
        &self,
        body: &crate::v2_6_0::types::ControllerServiceEntity,
    ) -> Result<crate::v2_6_0::types::ControllerServiceEntity, NifiError>;
}
/// Sub-resource trait for the `copy` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsCopyApi {
    /// Generates a copy response for the given copy request
    async fn copy(
        &self,
        body: &crate::v2_6_0::types::CopyRequestEntity,
    ) -> Result<crate::v2_6_0::types::CopyResponseEntity, NifiError>;
}
/// Sub-resource trait for the `download` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsDownloadApi {
    /// Gets a process group for download
    async fn export_process_group(
        &self,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError>;
}
/// Sub-resource trait for the `empty_all_connections_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsEmptyAllConnectionsRequestsApi {
    /// Creates a request to drop all flowfiles of all connection queues in this process group.
    async fn create_empty_all_connections_request(
        &self,
    ) -> Result<crate::v2_6_0::types::DropRequestDto, NifiError>;
    /// Cancels and/or removes a request to drop all flowfiles.
    async fn remove_drop_request_1(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_6_0::types::DropRequestDto, NifiError>;
    /// Gets the current status of a drop all flowfiles request.
    async fn get_drop_all_flowfiles_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_6_0::types::DropRequestDto, NifiError>;
}
/// Sub-resource trait for the `flow_contents` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsFlowContentsApi {
    /// Replace Process Group contents with the given ID with the specified Process Group contents
    async fn replace_process_group(
        &self,
        body: &crate::v2_6_0::types::ProcessGroupImportEntity,
    ) -> Result<crate::v2_6_0::types::ProcessGroupImportEntity, NifiError>;
}
/// Sub-resource trait for the `funnels` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsFunnelsApi {
    /// Gets all funnels
    async fn get_funnels(&self) -> Result<crate::v2_6_0::types::FunnelsEntity, NifiError>;
    /// Creates a funnel
    async fn create_funnel(
        &self,
        body: &crate::v2_6_0::types::FunnelEntity,
    ) -> Result<crate::v2_6_0::types::FunnelEntity, NifiError>;
}
/// Sub-resource trait for the `input_ports` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsInputPortsApi {
    /// Gets all input ports
    async fn get_input_ports(&self) -> Result<crate::v2_6_0::types::InputPortsEntity, NifiError>;
    /// Creates an input port
    async fn create_input_port(
        &self,
        body: &crate::v2_6_0::types::PortEntity,
    ) -> Result<crate::v2_6_0::types::PortEntity, NifiError>;
}
/// Sub-resource trait for the `labels` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsLabelsApi {
    /// Gets all labels
    async fn get_labels(&self) -> Result<crate::v2_6_0::types::LabelsEntity, NifiError>;
    /// Creates a label
    async fn create_label(
        &self,
        body: &crate::v2_6_0::types::LabelEntity,
    ) -> Result<crate::v2_6_0::types::LabelEntity, NifiError>;
}
/// Sub-resource trait for the `local_modifications` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsLocalModificationsApi {
    /// Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
    async fn get_local_modifications(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowComparisonEntity, NifiError>;
}
/// Sub-resource trait for the `output_ports` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsOutputPortsApi {
    /// Gets all output ports
    async fn get_output_ports(&self) -> Result<crate::v2_6_0::types::OutputPortsEntity, NifiError>;
    /// Creates an output port
    async fn create_output_port(
        &self,
        body: &crate::v2_6_0::types::PortEntity,
    ) -> Result<crate::v2_6_0::types::PortEntity, NifiError>;
}
/// Sub-resource trait for the `paste` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsPasteApi {
    /// Pastes into the specified process group
    async fn paste(
        &self,
        body: &crate::v2_6_0::types::PasteRequestEntity,
    ) -> Result<crate::v2_6_0::types::PasteResponseEntity, NifiError>;
}
/// Sub-resource trait for the `process_groups` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsProcessGroupsApi {
    /// Gets all process groups
    async fn get_process_groups(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessGroupsEntity, NifiError>;
    /// Creates a process group
    async fn create_process_group(
        &self,
        parameter_context_handling_strategy: Option<
            crate::v2_6_0::types::ParameterContextHandlingStrategy,
        >,
        body: &crate::v2_6_0::types::ProcessGroupEntity,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
    /// Imports a specified process group
    async fn import_process_group(
        &self,
        body: &crate::v2_6_0::types::ProcessGroupUploadEntity,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
    /// Uploads a versioned flow definition and creates a process group
    async fn upload_process_group(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
}
/// Sub-resource trait for the `processors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsProcessorsApi {
    /// Gets all processors
    async fn get_processors(
        &self,
        include_descendant_groups: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessorsEntity, NifiError>;
    /// Creates a new processor
    async fn create_processor(
        &self,
        body: &crate::v2_6_0::types::ProcessorEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError>;
}
/// Sub-resource trait for the `remote_process_groups` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsRemoteProcessGroupsApi {
    /// Gets all remote process groups
    async fn get_remote_process_groups(
        &self,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupsEntity, NifiError>;
    /// Creates a new process group
    async fn create_remote_process_group(
        &self,
        body: &crate::v2_6_0::types::RemoteProcessGroupEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
}
/// Sub-resource trait for the `replace_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsReplaceRequestsApi {
    /// Initiate the Replace Request of a Process Group with the given ID
    async fn initiate_replace_process_group(
        &self,
        body: &crate::v2_6_0::types::ProcessGroupImportEntity,
    ) -> Result<crate::v2_6_0::types::ProcessGroupReplaceRequestEntity, NifiError>;
}
/// Sub-resource trait for the `snippet_instance` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsSnippetInstanceApi {
    /// Copies a snippet and discards it.
    async fn copy_snippet(
        &self,
        body: &crate::v2_6_0::types::CopySnippetRequestEntity,
    ) -> Result<crate::v2_6_0::types::FlowDto, NifiError>;
}
/// The ProcessGroups API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsApi {
    type ProcessGroupsConnectionsApi<'b>: ProcessGroupsConnectionsApi
    where
        Self: 'b;
    fn connections<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsConnectionsApi<'b>;
    type ProcessGroupsControllerServicesApi<'b>: ProcessGroupsControllerServicesApi
    where
        Self: 'b;
    fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsControllerServicesApi<'b>;
    type ProcessGroupsCopyApi<'b>: ProcessGroupsCopyApi
    where
        Self: 'b;
    fn copy<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsCopyApi<'b>;
    type ProcessGroupsDownloadApi<'b>: ProcessGroupsDownloadApi
    where
        Self: 'b;
    fn download<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsDownloadApi<'b>;
    type ProcessGroupsEmptyAllConnectionsRequestsApi<
        'b,
    >: ProcessGroupsEmptyAllConnectionsRequestsApi
    where
        Self: 'b;
    fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsEmptyAllConnectionsRequestsApi<'b>;
    type ProcessGroupsFlowContentsApi<'b>: ProcessGroupsFlowContentsApi
    where
        Self: 'b;
    fn flow_contents<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFlowContentsApi<'b>;
    type ProcessGroupsFunnelsApi<'b>: ProcessGroupsFunnelsApi
    where
        Self: 'b;
    fn funnels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFunnelsApi<'b>;
    type ProcessGroupsInputPortsApi<'b>: ProcessGroupsInputPortsApi
    where
        Self: 'b;
    fn input_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsInputPortsApi<'b>;
    type ProcessGroupsLabelsApi<'b>: ProcessGroupsLabelsApi
    where
        Self: 'b;
    fn labels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsLabelsApi<'b>;
    type ProcessGroupsLocalModificationsApi<'b>: ProcessGroupsLocalModificationsApi
    where
        Self: 'b;
    fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsLocalModificationsApi<'b>;
    type ProcessGroupsOutputPortsApi<'b>: ProcessGroupsOutputPortsApi
    where
        Self: 'b;
    fn output_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsOutputPortsApi<'b>;
    type ProcessGroupsPasteApi<'b>: ProcessGroupsPasteApi
    where
        Self: 'b;
    fn paste<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsPasteApi<'b>;
    type ProcessGroupsProcessGroupsApi<'b>: ProcessGroupsProcessGroupsApi
    where
        Self: 'b;
    fn process_groups<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessGroupsApi<'b>;
    type ProcessGroupsProcessorsApi<'b>: ProcessGroupsProcessorsApi
    where
        Self: 'b;
    fn processors<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessorsApi<'b>;
    type ProcessGroupsRemoteProcessGroupsApi<'b>: ProcessGroupsRemoteProcessGroupsApi
    where
        Self: 'b;
    fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsRemoteProcessGroupsApi<'b>;
    type ProcessGroupsReplaceRequestsApi<'b>: ProcessGroupsReplaceRequestsApi
    where
        Self: 'b;
    fn replace_requests<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsReplaceRequestsApi<'b>;
    type ProcessGroupsSnippetInstanceApi<'b>: ProcessGroupsSnippetInstanceApi
    where
        Self: 'b;
    fn snippet_instance<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsSnippetInstanceApi<'b>;
    /// Deletes the Replace Request with the given ID
    async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupReplaceRequestEntity, NifiError>;
    /// Returns the Replace Request with the given ID
    async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ProcessGroupReplaceRequestEntity, NifiError>;
    /// Deletes a process group
    async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
    /// Gets a process group
    async fn get_process_group(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
    /// Updates a process group
    async fn update_process_group(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ProcessGroupEntity,
    ) -> Result<crate::v2_6_0::types::ProcessGroupEntity, NifiError>;
}

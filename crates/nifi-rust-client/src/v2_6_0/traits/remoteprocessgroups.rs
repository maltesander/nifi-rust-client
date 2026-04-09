// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `input_ports` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsInputPortsApi {
    /// Updates a remote port
    async fn update_remote_process_group_input_port(
        &self,
        port_id: &str,
        body: &crate::v2_6_0::types::RemoteProcessGroupPortEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupPortEntity, NifiError>;
    /// Updates run status of a remote port
    async fn update_remote_process_group_input_port_run_status(
        &self,
        port_id: &str,
        body: &crate::v2_6_0::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupPortEntity, NifiError>;
}
/// Sub-resource trait for the `output_ports` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsOutputPortsApi {
    /// Updates a remote port
    async fn update_remote_process_group_output_port(
        &self,
        port_id: &str,
        body: &crate::v2_6_0::types::RemoteProcessGroupPortEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupPortEntity, NifiError>;
    /// Updates run status of a remote port
    async fn update_remote_process_group_output_port_run_status(
        &self,
        port_id: &str,
        body: &crate::v2_6_0::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupPortEntity, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsRunStatusApi {
    /// Updates run status of all remote process groups in a process group (recursively)
    async fn update_remote_process_group_run_statuses(
        &self,
        body: &crate::v2_6_0::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
    /// Updates run status of a remote process group
    async fn update_remote_process_group_run_status(
        &self,
        body: &crate::v2_6_0::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsStateApi {
    /// Gets the state for a RemoteProcessGroup
    async fn get_state_3(&self) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError>;
}
/// The RemoteProcessGroups API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsApi {
    fn input_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsInputPortsApi + 'b;
    fn output_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsOutputPortsApi + 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsRunStatusApi + 'b;
    fn state<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsStateApi + 'b;
    /// Deletes a remote process group
    async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
    /// Gets a remote process group
    async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
    /// Updates a remote process group
    async fn update_remote_process_group(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::RemoteProcessGroupEntity,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupEntity, NifiError>;
}

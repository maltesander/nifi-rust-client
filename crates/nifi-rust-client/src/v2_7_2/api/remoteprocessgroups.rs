use crate::NifiClient;
use crate::NifiError;
pub struct RemoteProcessGroupsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsApi<'a> {
    /// Deletes a remote process group
    ///
    /// Calls `DELETE /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = version {
            query.push(("version", v.to_string()));
        }
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/remote-process-groups/{id}"), &query)
            .await
    }
    /// Gets a remote process group
    ///
    /// Calls `GET /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    pub async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupEntity, NifiError> {
        self.client
            .get(&format!("/remote-process-groups/{id}"))
            .await
    }
    /// Updates a remote process group
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `body`: The remote process group.
    pub async fn update_remote_process_group(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::RemoteProcessGroupEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupEntity, NifiError> {
        self.client
            .put(&format!("/remote-process-groups/{id}"), body)
            .await
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The remote process group id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> RemoteProcessGroupsBulletinsApi<'b> {
        RemoteProcessGroupsBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `input_ports` sub-resource of a specific process group.
    ///
    /// - `id`: The remote process group id.
    pub fn input_ports<'b>(&'b self, id: &'b str) -> RemoteProcessGroupsInputPortsApi<'b> {
        RemoteProcessGroupsInputPortsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `output_ports` sub-resource of a specific process group.
    ///
    /// - `id`: The remote process group id.
    pub fn output_ports<'b>(&'b self, id: &'b str) -> RemoteProcessGroupsOutputPortsApi<'b> {
        RemoteProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> RemoteProcessGroupsRunStatusApi<'b> {
        RemoteProcessGroupsRunStatusApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn state<'b>(&'b self, id: &'b str) -> RemoteProcessGroupsStateApi<'b> {
        RemoteProcessGroupsStateApi {
            client: self.client,
            id,
        }
    }
}
pub struct RemoteProcessGroupsBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsBulletinsApi<'a> {
    /// Clears bulletins for a remote process group
    ///
    /// Calls `POST /nifi-api/remote-process-groups/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The clear bulletin request.
    pub async fn clear_bulletins_6(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/remote-process-groups/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct RemoteProcessGroupsInputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsInputPortsApi<'a> {
    /// Updates a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/input-ports/{port-id}`.
    ///
    /// # Parameters
    /// - `port_id`: The remote process group port id.
    /// - `body`: The remote process group port.
    pub async fn update_remote_process_group_input_port(
        &self,
        port_id: &str,
        body: &crate::v2_7_2::types::RemoteProcessGroupPortEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupPortEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/remote-process-groups/{id}/input-ports/{port_id}"),
                body,
            )
            .await
    }
    /// Updates run status of a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/input-ports/{port-id}/run-status`.
    ///
    /// # Parameters
    /// - `port_id`: The remote process group port id.
    /// - `body`: The remote process group port.
    pub async fn update_remote_process_group_input_port_run_status(
        &self,
        port_id: &str,
        body: &crate::v2_7_2::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupPortEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/remote-process-groups/{id}/input-ports/{port_id}/run-status"),
                body,
            )
            .await
    }
}
pub struct RemoteProcessGroupsOutputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsOutputPortsApi<'a> {
    /// Updates a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/output-ports/{port-id}`.
    ///
    /// # Parameters
    /// - `port_id`: The remote process group port id.
    /// - `body`: The remote process group port.
    pub async fn update_remote_process_group_output_port(
        &self,
        port_id: &str,
        body: &crate::v2_7_2::types::RemoteProcessGroupPortEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupPortEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/remote-process-groups/{id}/output-ports/{port_id}"),
                body,
            )
            .await
    }
    /// Updates run status of a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/output-ports/{port-id}/run-status`.
    ///
    /// # Parameters
    /// - `port_id`: The remote process group port id.
    /// - `body`: The remote process group port.
    pub async fn update_remote_process_group_output_port_run_status(
        &self,
        port_id: &str,
        body: &crate::v2_7_2::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupPortEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/remote-process-groups/{id}/output-ports/{port_id}/run-status"),
                body,
            )
            .await
    }
}
pub struct RemoteProcessGroupsRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsRunStatusApi<'a> {
    /// Updates run status of all remote process groups in a process group (recursively)
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/process-group/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The remote process groups run status.
    pub async fn update_remote_process_group_run_statuses(
        &self,
        body: &crate::v2_7_2::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/remote-process-groups/process-group/{id}/run-status"),
                body,
            )
            .await
    }
    /// Updates run status of a remote process group
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The remote process group run status.
    pub async fn update_remote_process_group_run_status(
        &self,
        body: &crate::v2_7_2::types::RemotePortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::RemoteProcessGroupEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/remote-process-groups/{id}/run-status"), body)
            .await
    }
}
pub struct RemoteProcessGroupsStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> RemoteProcessGroupsStateApi<'a> {
    /// Gets the state for a RemoteProcessGroup
    ///
    /// Calls `GET /nifi-api/remote-process-groups/{id}/state`.
    pub async fn get_state_3(&self) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ComponentStateEntity = self
            .client
            .get(&format!("/remote-process-groups/{id}/state"))
            .await?;
        Ok(e.component_state)
    }
}

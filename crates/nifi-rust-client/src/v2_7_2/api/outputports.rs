use crate::NifiClient;
use crate::NifiError;
pub struct OutputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> OutputPortsApi<'a> {
    /// Deletes an output port
    ///
    /// Calls `DELETE /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::types::PortEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/output-ports/{id}"), &query)
            .await
    }
    /// Gets an output port
    ///
    /// Calls `GET /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    pub async fn get_output_port(&self, id: &str) -> Result<crate::types::PortEntity, NifiError> {
        self.client.get(&format!("/output-ports/{id}")).await
    }
    /// Updates an output port
    ///
    /// Calls `PUT /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    /// - `body`: The output port configuration details.
    pub async fn update_output_port(
        &self,
        id: &str,
        body: &crate::types::PortEntity,
    ) -> Result<crate::types::PortEntity, NifiError> {
        self.client.put(&format!("/output-ports/{id}"), body).await
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The output port id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> OutputPortsBulletinsApi<'b> {
        OutputPortsBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The port id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> OutputPortsRunStatusApi<'b> {
        OutputPortsRunStatusApi {
            client: self.client,
            id,
        }
    }
}
pub struct OutputPortsBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> OutputPortsBulletinsApi<'a> {
    /// Clears bulletins for an output port
    ///
    /// Calls `POST /nifi-api/output-ports/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The request to clear bulletins.
    pub async fn clear_bulletins_3(
        &self,
        body: &crate::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/output-ports/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct OutputPortsRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> OutputPortsRunStatusApi<'a> {
    /// Updates run status of an output-port
    ///
    /// Calls `PUT /nifi-api/output-ports/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The port run status.
    pub async fn update_run_status_3(
        &self,
        body: &crate::types::PortRunStatusEntity,
    ) -> Result<crate::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/output-ports/{id}/run-status"), body)
            .await
    }
}

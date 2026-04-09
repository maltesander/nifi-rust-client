// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write - /output-ports/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    pub async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /output-ports/{uuid}`.
    pub async fn get_output_port(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
        self.client.get(&format!("/output-ports/{id}")).await
    }
    /// Updates an output port
    ///
    /// Calls `PUT /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    /// - `body`: The output port configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid}`.
    pub async fn update_output_port(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::PortEntity,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid}`.
    pub async fn clear_bulletins_3(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError> {
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid} or /operation/output-ports/{uuid}`.
    pub async fn update_run_status_3(
        &self,
        body: &crate::v2_7_2::types::PortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/output-ports/{id}/run-status"), body)
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_7_2::traits::OutputPortsApi for OutputPortsApi<'_> {
    type OutputPortsBulletinsApi<'b>
        = OutputPortsBulletinsApi<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::OutputPortsBulletinsApi<'b> {
        OutputPortsBulletinsApi {
            client: self.client,
            id,
        }
    }
    type OutputPortsRunStatusApi<'b>
        = OutputPortsRunStatusApi<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::OutputPortsRunStatusApi<'b> {
        OutputPortsRunStatusApi {
            client: self.client,
            id,
        }
    }
    async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
        self.remove_output_port(id, version, client_id, disconnected_node_acknowledged)
            .await
    }
    async fn get_output_port(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
        self.get_output_port(id).await
    }
    async fn update_output_port(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::PortEntity,
    ) -> Result<crate::v2_7_2::types::PortEntity, NifiError> {
        self.update_output_port(id, body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_7_2::traits::OutputPortsBulletinsApi for OutputPortsBulletinsApi<'_> {
    async fn clear_bulletins_3(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError> {
        self.clear_bulletins_3(body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_7_2::traits::OutputPortsRunStatusApi for OutputPortsRunStatusApi<'_> {
    async fn update_run_status_3(
        &self,
        body: &crate::v2_7_2::types::PortRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError> {
        self.update_run_status_3(body).await
    }
}

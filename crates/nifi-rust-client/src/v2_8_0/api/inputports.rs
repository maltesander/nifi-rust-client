// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct InputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> InputPortsApi<'a> {
    /// Deletes an input port
    ///
    /// Calls `DELETE /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
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
    /// - `Write - /input-ports/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    pub async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/input-ports/{id}"), &query)
            .await
    }
    /// Gets an input port
    ///
    /// Calls `GET /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /input-ports/{uuid}`.
    pub async fn get_input_port(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
        self.client.get(&format!("/input-ports/{id}")).await
    }
    /// Updates an input port
    ///
    /// Calls `PUT /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
    /// - `body`: The input port configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /input-ports/{uuid}`.
    pub async fn update_input_port(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::PortEntity,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
        self.client.put(&format!("/input-ports/{id}"), body).await
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The input port id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> InputPortsBulletinsApi<'b> {
        InputPortsBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The port id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> InputPortsRunStatusApi<'b> {
        InputPortsRunStatusApi {
            client: self.client,
            id,
        }
    }
}
pub struct InputPortsBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> InputPortsBulletinsApi<'a> {
    /// Clears bulletins for an input port
    ///
    /// Calls `POST /nifi-api/input-ports/{id}/bulletins/clear-requests`.
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
    /// Requires `Write - /input-ports/{uuid}`.
    pub async fn clear_bulletins_2(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/input-ports/{id}/bulletins/clear-requests"), body)
            .await
    }
}
pub struct InputPortsRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> InputPortsRunStatusApi<'a> {
    /// Updates run status of an input-port
    ///
    /// Calls `PUT /nifi-api/input-ports/{id}/run-status`.
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
    /// Requires `Write - /input-ports/{uuid} or /operation/input-ports/{uuid}`.
    pub async fn update_run_status_2(
        &self,
        body: &crate::v2_8_0::types::PortRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/input-ports/{id}/run-status"), body)
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::InputPortsApi for InputPortsApi<'_> {
    fn bulletins<'b>(
        &'b self,
        id: &'b str,
    ) -> impl crate::v2_8_0::traits::InputPortsBulletinsApi + 'b {
        InputPortsBulletinsApi {
            client: self.client,
            id,
        }
    }
    fn run_status<'b>(
        &'b self,
        id: &'b str,
    ) -> impl crate::v2_8_0::traits::InputPortsRunStatusApi + 'b {
        InputPortsRunStatusApi {
            client: self.client,
            id,
        }
    }
    async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
        self.remove_input_port(id, version, client_id, disconnected_node_acknowledged)
            .await
    }
    async fn get_input_port(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
        self.get_input_port(id).await
    }
    async fn update_input_port(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::PortEntity,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError> {
        self.update_input_port(id, body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::InputPortsBulletinsApi for InputPortsBulletinsApi<'_> {
    async fn clear_bulletins_2(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        self.clear_bulletins_2(body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::InputPortsRunStatusApi for InputPortsRunStatusApi<'_> {
    async fn update_run_status_2(
        &self,
        body: &crate::v2_8_0::types::PortRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::ProcessorEntity, NifiError> {
        self.update_run_status_2(body).await
    }
}

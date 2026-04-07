// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct LabelsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> LabelsApi<'a> {
    /// Deletes a label
    ///
    /// Calls `DELETE /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
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
    /// - `Write - /labels/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    pub async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::LabelEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/labels/{id}"), &query)
            .await
    }
    /// Gets a label
    ///
    /// Calls `GET /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /labels/{uuid}`.
    pub async fn get_label(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::LabelEntity, NifiError> {
        self.client.get(&format!("/labels/{id}")).await
    }
    /// Updates a label
    ///
    /// Calls `PUT /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
    /// - `body`: The label configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /labels/{uuid}`.
    pub async fn update_label(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::LabelEntity,
    ) -> Result<crate::v2_7_2::types::LabelEntity, NifiError> {
        self.client.put(&format!("/labels/{id}"), body).await
    }
}

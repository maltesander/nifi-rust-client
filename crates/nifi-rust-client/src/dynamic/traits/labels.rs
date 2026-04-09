// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Labels API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait LabelsApi {
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
    async fn get_label(&self, id: &str) -> Result<types::LabelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_label".to_string(),
            version: "unknown".to_string(),
        })
    }
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
    /// Requires `Write - /labels/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::LabelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_label".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a label
    ///
    /// Calls `PUT /nifi-api/labels/{id}`.
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
    /// Requires `Write - /labels/{uuid}`.
    async fn update_label(
        &self,
        id: &str,
        body: &types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_label".to_string(),
            version: "unknown".to_string(),
        })
    }
}

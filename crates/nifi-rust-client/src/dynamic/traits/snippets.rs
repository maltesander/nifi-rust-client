// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Snippets API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SnippetsApi {
    /// Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.
    ///
    /// Calls `POST /nifi-api/snippets`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read or Write - /{component-type}/{uuid} - For every component (all Read or all Write) in the Snippet and their descendant components`.
    async fn create_snippet(
        &self,
        body: types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_snippet".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the components in a snippet and discards the snippet
    ///
    /// Calls `DELETE /nifi-api/snippets/{id}`.
    ///
    /// # Parameters
    /// - `id`: The snippet id.
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
    /// Requires `Write - /{component-type}/{uuid} - For each component in the Snippet and their descendant components`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::SnippetEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_snippet".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Move's the components in this Snippet into a new Process Group and discards the snippet
    ///
    /// Calls `PUT /nifi-api/snippets/{id}`.
    ///
    /// # Parameters
    /// - `id`: The snippet id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write Process Group - /process-groups/{uuid}`.
    /// Requires `Write - /{component-type}/{uuid} - For each component in the Snippet and their descendant components`.
    async fn update_snippet(
        &self,
        id: &str,
        body: types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_snippet".to_string(),
            version: "unknown".to_string(),
        })
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct SnippetsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> SnippetsApi<'a> {
    /// Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.
    ///
    /// Calls `POST /nifi-api/snippets`.
    ///
    /// # Parameters
    /// - `body`: The snippet configuration details.
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
    pub async fn create_snippet(
        &self,
        body: &crate::v2_6_0::types::SnippetEntity,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError> {
        self.client.post("/snippets", body).await
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
    /// - `Write - /{component-type}/{uuid} - For each component in the Snippet and their descendant components`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    pub async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/snippets/{id}"), &query)
            .await
    }
    /// Move's the components in this Snippet into a new Process Group and discards the snippet
    ///
    /// Calls `PUT /nifi-api/snippets/{id}`.
    ///
    /// # Parameters
    /// - `id`: The snippet id.
    /// - `body`: The snippet configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write Process Group - /process-groups/{uuid}`
    /// - `Write - /{component-type}/{uuid} - For each component in the Snippet and their descendant components`
    pub async fn update_snippet(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::SnippetEntity,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError> {
        self.client.put(&format!("/snippets/{id}"), body).await
    }
}

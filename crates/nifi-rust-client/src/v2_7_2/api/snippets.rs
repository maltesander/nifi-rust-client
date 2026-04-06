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
    pub async fn create_snippet(
        &self,
        body: &crate::v2_7_2::types::SnippetEntity,
    ) -> Result<crate::v2_7_2::types::SnippetEntity, NifiError> {
        self.client.post("/snippets", body).await
    }
    /// Deletes the components in a snippet and discards the snippet
    ///
    /// Calls `DELETE /nifi-api/snippets/{id}`.
    ///
    /// # Parameters
    /// - `id`: The snippet id.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::SnippetEntity, NifiError> {
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
    pub async fn update_snippet(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::SnippetEntity,
    ) -> Result<crate::v2_7_2::types::SnippetEntity, NifiError> {
        self.client.put(&format!("/snippets/{id}"), body).await
    }
}

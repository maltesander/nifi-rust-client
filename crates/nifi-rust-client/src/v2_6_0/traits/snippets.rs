// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Snippets API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SnippetsApi {
    /// Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.
    async fn create_snippet(
        &self,
        body: &crate::v2_6_0::types::SnippetEntity,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError>;
    /// Deletes the components in a snippet and discards the snippet
    async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError>;
    /// Move's the components in this Snippet into a new Process Group and discards the snippet
    async fn update_snippet(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::SnippetEntity,
    ) -> Result<crate::v2_6_0::types::SnippetEntity, NifiError>;
}

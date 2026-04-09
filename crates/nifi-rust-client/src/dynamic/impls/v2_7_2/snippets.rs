// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SnippetsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2SnippetsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl SnippetsApi for V2_7_2SnippetsApi<'_> {
    async fn create_snippet(
        &self,
        body: &types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        let api = crate::v2_7_2::api::snippets::SnippetsApi {
            client: self.client,
        };
        Ok(api
            .create_snippet(&crate::v2_7_2::types::SnippetEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::SnippetEntity, NifiError> {
        let api = crate::v2_7_2::api::snippets::SnippetsApi {
            client: self.client,
        };
        Ok(api
            .delete_snippet(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_snippet(
        &self,
        id: &str,
        body: &types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        let api = crate::v2_7_2::api::snippets::SnippetsApi {
            client: self.client,
        };
        Ok(api
            .update_snippet(
                id,
                &crate::v2_7_2::types::SnippetEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SnippetsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Snippets API. Use via the [`SnippetsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum SnippetsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0SnippetsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2SnippetsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0SnippetsApi<'a>),
}
impl<'a> SnippetsApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl SnippetsApi for SnippetsApiDispatch<'_> {
    async fn create_snippet(
        &self,
        body: &types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_snippet(body).await,
            Self::V2_7_2(api) => api.create_snippet(body).await,
            Self::V2_8_0(api) => api.create_snippet(body).await,
        }
    }
    async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_snippet(id, disconnected_node_acknowledged).await,
            Self::V2_7_2(api) => api.delete_snippet(id, disconnected_node_acknowledged).await,
            Self::V2_8_0(api) => api.delete_snippet(id, disconnected_node_acknowledged).await,
        }
    }
    async fn update_snippet(
        &self,
        id: &str,
        body: &types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_snippet(id, body).await,
            Self::V2_7_2(api) => api.update_snippet(id, body).await,
            Self::V2_8_0(api) => api.update_snippet(id, body).await,
        }
    }
}

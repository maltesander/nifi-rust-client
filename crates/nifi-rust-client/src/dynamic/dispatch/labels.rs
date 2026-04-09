// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::LabelsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Labels API. Use via the [`LabelsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum LabelsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0LabelsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2LabelsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0LabelsApi<'a>),
}
impl LabelsApi for LabelsApiDispatch<'_> {
    async fn get_label(&self, id: &str) -> Result<types::LabelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_label(id).await,
            Self::V2_7_2(api) => api.get_label(id).await,
            Self::V2_8_0(api) => api.get_label(id).await,
        }
    }
    async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::LabelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_label(
        &self,
        id: &str,
        body: &types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_label(id, body).await,
            Self::V2_7_2(api) => api.update_label(id, body).await,
            Self::V2_8_0(api) => api.update_label(id, body).await,
        }
    }
}

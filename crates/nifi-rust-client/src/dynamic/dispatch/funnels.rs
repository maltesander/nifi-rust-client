// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FunnelsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Funnels API. Use via the [`FunnelsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum FunnelsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0FunnelsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2FunnelsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0FunnelsApi<'a>),
}
impl FunnelsApi for FunnelsApiDispatch<'_> {
    async fn get_funnel(&self, id: &str) -> Result<types::FunnelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_funnel(id).await,
            Self::V2_7_2(api) => api.get_funnel(id).await,
            Self::V2_8_0(api) => api.get_funnel(id).await,
        }
    }
    async fn remove_funnel(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_funnel(id, body).await,
            Self::V2_7_2(api) => api.update_funnel(id, body).await,
            Self::V2_8_0(api) => api.update_funnel(id, body).await,
        }
    }
}

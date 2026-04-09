// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ConnectionsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Connections API. Use via the [`ConnectionsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ConnectionsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ConnectionsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ConnectionsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ConnectionsApi<'a>),
}
impl ConnectionsApi for ConnectionsApiDispatch<'_> {
    async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn get_connection(&self, id: &str) -> Result<types::ConnectionEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_connection(id).await,
            Self::V2_7_2(api) => api.get_connection(id).await,
            Self::V2_8_0(api) => api.get_connection(id).await,
        }
    }
    async fn update_connection(
        &self,
        id: &str,
        body: &types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_connection(id, body).await,
            Self::V2_7_2(api) => api.update_connection(id, body).await,
            Self::V2_8_0(api) => api.update_connection(id, body).await,
        }
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::InputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the InputPorts API. Use via the [`InputPortsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum InputPortsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0InputPortsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2InputPortsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0InputPortsApi<'a>),
}
impl InputPortsApi for InputPortsApiDispatch<'_> {
    async fn clear_bulletins_2(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_2(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_2(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_2(id, body).await,
        }
    }
    async fn get_input_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_input_port(id).await,
            Self::V2_7_2(api) => api.get_input_port(id).await,
            Self::V2_8_0(api) => api.get_input_port(id).await,
        }
    }
    async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_input_port(id, body).await,
            Self::V2_7_2(api) => api.update_input_port(id, body).await,
            Self::V2_8_0(api) => api.update_input_port(id, body).await,
        }
    }
    async fn update_run_status_2(
        &self,
        id: &str,
        body: types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_run_status_2(id, body).await,
            Self::V2_7_2(api) => api.update_run_status_2(id, body).await,
            Self::V2_8_0(api) => api.update_run_status_2(id, body).await,
        }
    }
}

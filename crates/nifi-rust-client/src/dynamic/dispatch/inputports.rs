// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::InputPortsApi;
use crate::dynamic::traits::InputPortsBulletinsApi;
use crate::dynamic::traits::InputPortsRunStatusApi;
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
impl<'a> InputPortsApiDispatch<'a> {
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
impl InputPortsApi for InputPortsApiDispatch<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl InputPortsBulletinsApi + 'b {
        InputPortsBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl InputPortsRunStatusApi + 'b {
        InputPortsRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
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
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_input_port(id, body).await,
            Self::V2_7_2(api) => api.update_input_port(id, body).await,
            Self::V2_8_0(api) => api.update_input_port(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [InputPortsBulletinsApi].
pub struct InputPortsBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl InputPortsBulletinsApi for InputPortsBulletinsApiDispatch<'_> {
    async fn clear_bulletins_2(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_2".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_2(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_2(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [InputPortsRunStatusApi].
pub struct InputPortsRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl InputPortsRunStatusApi for InputPortsRunStatusApiDispatch<'_> {
    async fn update_run_status_2(
        &self,
        body: &types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_6_0::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_7_2::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_8_0::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

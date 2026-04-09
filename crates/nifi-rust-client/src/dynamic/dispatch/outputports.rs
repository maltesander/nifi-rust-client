// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::OutputPortsApi;
use crate::dynamic::traits::OutputPortsBulletinsApi;
use crate::dynamic::traits::OutputPortsRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the OutputPorts API. Use via the [`OutputPortsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum OutputPortsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0OutputPortsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2OutputPortsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0OutputPortsApi<'a>),
}
impl<'a> OutputPortsApiDispatch<'a> {
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
impl OutputPortsApi for OutputPortsApiDispatch<'_> {
    type OutputPortsBulletinsApi<'b>
        = OutputPortsBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::OutputPortsBulletinsApi<'b> {
        OutputPortsBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type OutputPortsRunStatusApi<'b>
        = OutputPortsRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::OutputPortsRunStatusApi<'b> {
        OutputPortsRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn get_output_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_output_port(id).await,
            Self::V2_7_2(api) => api.get_output_port(id).await,
            Self::V2_8_0(api) => api.get_output_port(id).await,
        }
    }
    async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_output_port(
        &self,
        id: &str,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_output_port(id, body).await,
            Self::V2_7_2(api) => api.update_output_port(id, body).await,
            Self::V2_8_0(api) => api.update_output_port(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [OutputPortsBulletinsApi].
pub struct OutputPortsBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl OutputPortsBulletinsApi for OutputPortsBulletinsApiDispatch<'_> {
    async fn clear_bulletins_3(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_3".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_3(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_3(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [OutputPortsRunStatusApi].
pub struct OutputPortsRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl OutputPortsRunStatusApi for OutputPortsRunStatusApiDispatch<'_> {
    async fn update_run_status_3(
        &self,
        body: &types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_6_0::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_7_2::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_8_0::types::PortRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

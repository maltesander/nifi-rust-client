// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessorsApi;
use crate::dynamic::traits::ProcessorsBulletinsApi;
use crate::dynamic::traits::ProcessorsConfigApi;
use crate::dynamic::traits::ProcessorsDescriptorsApi;
use crate::dynamic::traits::ProcessorsDiagnosticsApi;
use crate::dynamic::traits::ProcessorsRunStatusApi;
use crate::dynamic::traits::ProcessorsStateApi;
use crate::dynamic::traits::ProcessorsThreadsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Processors API. Use via the [`ProcessorsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ProcessorsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ProcessorsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ProcessorsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ProcessorsApi<'a>),
}
impl<'a> ProcessorsApiDispatch<'a> {
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
impl ProcessorsApi for ProcessorsApiDispatch<'_> {
    type ProcessorsBulletinsApi<'b>
        = ProcessorsBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ProcessorsBulletinsApi<'b> {
        ProcessorsBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsConfigApi<'b>
        = ProcessorsConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ProcessorsConfigApi<'b> {
        ProcessorsConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsDescriptorsApi<'b>
        = ProcessorsDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ProcessorsDescriptorsApi<'b> {
        ProcessorsDescriptorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsDiagnosticsApi<'b>
        = ProcessorsDiagnosticsApiDispatch<'b>
    where
        Self: 'b;
    fn diagnostics<'b>(&'b self, id: &'b str) -> Self::ProcessorsDiagnosticsApi<'b> {
        ProcessorsDiagnosticsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsRunStatusApi<'b>
        = ProcessorsRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ProcessorsRunStatusApi<'b> {
        ProcessorsRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsStateApi<'b>
        = ProcessorsStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ProcessorsStateApi<'b> {
        ProcessorsStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ProcessorsThreadsApi<'b>
        = ProcessorsThreadsApiDispatch<'b>
    where
        Self: 'b;
    fn threads<'b>(&'b self, id: &'b str) -> Self::ProcessorsThreadsApi<'b> {
        ProcessorsThreadsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor(id).await,
            Self::V2_7_2(api) => api.get_processor(id).await,
            Self::V2_8_0(api) => api.get_processor(id).await,
        }
    }
    async fn get_processor_run_status_details(
        &self,
        body: &types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor_run_status_details(body).await,
            Self::V2_7_2(api) => api.get_processor_run_status_details(body).await,
            Self::V2_8_0(api) => api.get_processor_run_status_details(body).await,
        }
    }
    async fn update_processor(
        &self,
        id: &str,
        body: &types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_processor(id, body).await,
            Self::V2_7_2(api) => api.update_processor(id, body).await,
            Self::V2_8_0(api) => api.update_processor(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsBulletinsApi].
pub struct ProcessorsBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsBulletinsApi for ProcessorsBulletinsApiDispatch<'_> {
    async fn clear_bulletins_5(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_5".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_5(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_5(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_5".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsConfigApi].
pub struct ProcessorsConfigApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsConfigApi for ProcessorsConfigApiDispatch<'_> {
    async fn analyze_configuration_2(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_configuration_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_verification_request_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_verification_request_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_processor_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_processor_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsDescriptorsApi].
pub struct ProcessorsDescriptorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsDescriptorsApi for ProcessorsDescriptorsApiDispatch<'_> {
    async fn get_property_descriptor_3(
        &self,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_property_descriptor_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsDiagnosticsApi].
pub struct ProcessorsDiagnosticsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsDiagnosticsApi for ProcessorsDiagnosticsApiDispatch<'_> {
    async fn get_processor_diagnostics(&self) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_processor_diagnostics".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsRunStatusApi].
pub struct ProcessorsRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsRunStatusApi for ProcessorsRunStatusApiDispatch<'_> {
    async fn update_run_status_4(
        &self,
        body: &types::ProcessorRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_6_0::types::ProcessorRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_7_2::types::ProcessorRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_8_0::types::ProcessorRunStatusEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status_4".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsStateApi].
pub struct ProcessorsStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsStateApi for ProcessorsStateApiDispatch<'_> {
    async fn clear_state_3(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_6_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_7_2::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_8_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_state_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_state_2(&self) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_2().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_2().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_2().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_state_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ProcessorsThreadsApi].
pub struct ProcessorsThreadsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ProcessorsThreadsApi for ProcessorsThreadsApiDispatch<'_> {
    async fn terminate_processor(&self) -> Result<types::ProcessorEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.terminate_processor().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.terminate_processor().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.terminate_processor().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "terminate_processor".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

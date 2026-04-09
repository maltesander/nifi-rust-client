// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ReportingTasksApi;
use crate::dynamic::traits::ReportingTasksBulletinsApi;
use crate::dynamic::traits::ReportingTasksConfigApi;
use crate::dynamic::traits::ReportingTasksDescriptorsApi;
use crate::dynamic::traits::ReportingTasksRunStatusApi;
use crate::dynamic::traits::ReportingTasksStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ReportingTasks API. Use via the [`ReportingTasksApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ReportingTasksApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ReportingTasksApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ReportingTasksApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ReportingTasksApi<'a>),
}
impl<'a> ReportingTasksApiDispatch<'a> {
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
impl ReportingTasksApi for ReportingTasksApiDispatch<'_> {
    type ReportingTasksBulletinsApi<'b>
        = ReportingTasksBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ReportingTasksBulletinsApi<'b> {
        ReportingTasksBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ReportingTasksConfigApi<'b>
        = ReportingTasksConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ReportingTasksConfigApi<'b> {
        ReportingTasksConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ReportingTasksDescriptorsApi<'b>
        = ReportingTasksDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ReportingTasksDescriptorsApi<'b> {
        ReportingTasksDescriptorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ReportingTasksRunStatusApi<'b>
        = ReportingTasksRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ReportingTasksRunStatusApi<'b> {
        ReportingTasksRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ReportingTasksStateApi<'b>
        = ReportingTasksStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ReportingTasksStateApi<'b> {
        ReportingTasksStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn get_reporting_task(&self, id: &str) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_reporting_task(id).await,
            Self::V2_7_2(api) => api.get_reporting_task(id).await,
            Self::V2_8_0(api) => api.get_reporting_task(id).await,
        }
    }
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_reporting_task(
        &self,
        id: &str,
        body: &types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_reporting_task(id, body).await,
            Self::V2_7_2(api) => api.update_reporting_task(id, body).await,
            Self::V2_8_0(api) => api.update_reporting_task(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ReportingTasksBulletinsApi].
pub struct ReportingTasksBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ReportingTasksBulletinsApi for ReportingTasksBulletinsApiDispatch<'_> {
    async fn clear_bulletins_7(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_7".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_7(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_7(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_7".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ReportingTasksConfigApi].
pub struct ReportingTasksConfigApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ReportingTasksConfigApi for ReportingTasksConfigApiDispatch<'_> {
    async fn analyze_configuration_3(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_configuration_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_verification_request_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_verification_request_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_config_verification_request_2(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_config_verification_request_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ReportingTasksDescriptorsApi].
pub struct ReportingTasksDescriptorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ReportingTasksDescriptorsApi for ReportingTasksDescriptorsApiDispatch<'_> {
    async fn get_property_descriptor_4(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_property_descriptor_4".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ReportingTasksRunStatusApi].
pub struct ReportingTasksRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ReportingTasksRunStatusApi for ReportingTasksRunStatusApiDispatch<'_> {
    async fn update_run_status_5(
        &self,
        body: &types::ReportingTaskRunStatusEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_6_0::types::ReportingTaskRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_7_2::types::ReportingTaskRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_8_0::types::ReportingTaskRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status_5".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ReportingTasksStateApi].
pub struct ReportingTasksStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ReportingTasksStateApi for ReportingTasksStateApiDispatch<'_> {
    async fn clear_state_4(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_6_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_7_2::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_8_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_state_4".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_state_4(&self) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_4().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_4().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_4().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_state_4".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

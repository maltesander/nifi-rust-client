// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerServicesApi;
use crate::dynamic::traits::ControllerServicesBulletinsApi;
use crate::dynamic::traits::ControllerServicesConfigApi;
use crate::dynamic::traits::ControllerServicesDescriptorsApi;
use crate::dynamic::traits::ControllerServicesReferencesApi;
use crate::dynamic::traits::ControllerServicesRunStatusApi;
use crate::dynamic::traits::ControllerServicesStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Controller Services API. Use via the [`ControllerServicesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ControllerServicesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ControllerServicesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ControllerServicesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ControllerServicesApi<'a>),
}
impl<'a> ControllerServicesApiDispatch<'a> {
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
impl ControllerServicesApi for ControllerServicesApiDispatch<'_> {
    type ControllerServicesBulletinsApi<'b>
        = ControllerServicesBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ControllerServicesBulletinsApi<'b> {
        ControllerServicesBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ControllerServicesConfigApi<'b>
        = ControllerServicesConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ControllerServicesConfigApi<'b> {
        ControllerServicesConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ControllerServicesDescriptorsApi<'b>
        = ControllerServicesDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ControllerServicesDescriptorsApi<'b> {
        ControllerServicesDescriptorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ControllerServicesReferencesApi<'b>
        = ControllerServicesReferencesApiDispatch<'b>
    where
        Self: 'b;
    fn references<'b>(&'b self, id: &'b str) -> Self::ControllerServicesReferencesApi<'b> {
        ControllerServicesReferencesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ControllerServicesRunStatusApi<'b>
        = ControllerServicesRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ControllerServicesRunStatusApi<'b> {
        ControllerServicesRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type ControllerServicesStateApi<'b>
        = ControllerServicesStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ControllerServicesStateApi<'b> {
        ControllerServicesStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_controller_service(id, ui_only).await,
            Self::V2_7_2(api) => api.get_controller_service(id, ui_only).await,
            Self::V2_8_0(api) => api.get_controller_service(id, ui_only).await,
        }
    }
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn update_controller_service(
        &self,
        id: &str,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_controller_service(id, body).await,
            Self::V2_7_2(api) => api.update_controller_service(id, body).await,
            Self::V2_8_0(api) => api.update_controller_service(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesBulletinsApi].
pub struct ControllerServicesBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesBulletinsApi for ControllerServicesBulletinsApiDispatch<'_> {
    async fn clear_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesConfigApi].
pub struct ControllerServicesConfigApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesConfigApi for ControllerServicesConfigApiDispatch<'_> {
    async fn analyze_configuration(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_configuration".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_config_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_config_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesDescriptorsApi].
pub struct ControllerServicesDescriptorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesDescriptorsApi for ControllerServicesDescriptorsApiDispatch<'_> {
    async fn get_property_descriptor_1(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_property_descriptor_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesReferencesApi].
pub struct ControllerServicesReferencesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesReferencesApi for ControllerServicesReferencesApiDispatch<'_> {
    async fn get_controller_service_references(
        &self,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_controller_service_references".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn update_controller_service_references(
        &self,
        body: &types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_6_0::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_8_0::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_controller_service_references".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesRunStatusApi].
pub struct ControllerServicesRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesRunStatusApi for ControllerServicesRunStatusApiDispatch<'_> {
    async fn update_run_status_1(
        &self,
        body: &types::ControllerServiceRunStatusEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_6_0::types::ControllerServiceRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_7_2::types::ControllerServiceRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_8_0::types::ControllerServiceRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerServicesStateApi].
pub struct ControllerServicesStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerServicesStateApi for ControllerServicesStateApiDispatch<'_> {
    async fn clear_state_1(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_6_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_7_2::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_8_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_state_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_state(&self) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_state".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

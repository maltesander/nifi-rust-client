// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterProvidersApi;
use crate::dynamic::traits::ParameterProvidersApplyParametersRequestsApi;
use crate::dynamic::traits::ParameterProvidersBulletinsApi;
use crate::dynamic::traits::ParameterProvidersConfigApi;
use crate::dynamic::traits::ParameterProvidersDescriptorsApi;
use crate::dynamic::traits::ParameterProvidersParametersApi;
use crate::dynamic::traits::ParameterProvidersReferencesApi;
use crate::dynamic::traits::ParameterProvidersStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ParameterProviders API. Use via the [`ParameterProvidersApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ParameterProvidersApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ParameterProvidersApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ParameterProvidersApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ParameterProvidersApi<'a>),
}
impl<'a> ParameterProvidersApiDispatch<'a> {
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
impl ParameterProvidersApi for ParameterProvidersApiDispatch<'_> {
    fn apply_parameters_requests<'b>(
        &'b self,
        provider_id: &'b str,
    ) -> impl ParameterProvidersApplyParametersRequestsApi + 'b {
        ParameterProvidersApplyParametersRequestsApiDispatch {
            client: self.client(),
            provider_id: provider_id.to_string(),
            version: self.version(),
        }
    }
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ParameterProvidersBulletinsApi + 'b {
        ParameterProvidersBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn config<'b>(&'b self, id: &'b str) -> impl ParameterProvidersConfigApi + 'b {
        ParameterProvidersConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ParameterProvidersDescriptorsApi + 'b {
        ParameterProvidersDescriptorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn parameters<'b>(&'b self, id: &'b str) -> impl ParameterProvidersParametersApi + 'b {
        ParameterProvidersParametersApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn references<'b>(&'b self, id: &'b str) -> impl ParameterProvidersReferencesApi + 'b {
        ParameterProvidersReferencesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl ParameterProvidersStateApi + 'b {
        ParameterProvidersStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_parameter_provider(id).await,
            Self::V2_7_2(api) => api.get_parameter_provider(id).await,
            Self::V2_8_0(api) => api.get_parameter_provider(id).await,
        }
    }
    async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_parameter_provider(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.remove_parameter_provider(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.remove_parameter_provider(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_parameter_provider(id, body).await,
            Self::V2_7_2(api) => api.update_parameter_provider(id, body).await,
            Self::V2_8_0(api) => api.update_parameter_provider(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersApplyParametersRequestsApi].
pub struct ParameterProvidersApplyParametersRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) provider_id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersApplyParametersRequestsApi
    for ParameterProvidersApplyParametersRequestsApiDispatch<'_>
{
    async fn delete_apply_parameters_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_apply_parameters_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_parameter_provider_apply_parameters_request(
        &self,
        request_id: &str,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_parameter_provider_apply_parameters_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_apply_parameters(
        &self,
        body: &types::ParameterProviderParameterApplicationEntity,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_6_0::types::ParameterProviderParameterApplicationEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_7_2::types::ParameterProviderParameterApplicationEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id: &self.provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_8_0::types::ParameterProviderParameterApplicationEntity::try_from(
                                body.clone(),
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_apply_parameters".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersBulletinsApi].
pub struct ParameterProvidersBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersBulletinsApi for ParameterProvidersBulletinsApiDispatch<'_> {
    async fn clear_bulletins_4(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_4".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_4(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_4(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_4".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersConfigApi].
pub struct ParameterProvidersConfigApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersConfigApi for ParameterProvidersConfigApiDispatch<'_> {
    async fn analyze_configuration_1(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_configuration_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_verification_request_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_verification_request_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_config_verification_request_1(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_config_verification_request_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersDescriptorsApi].
pub struct ParameterProvidersDescriptorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersDescriptorsApi for ParameterProvidersDescriptorsApiDispatch<'_> {
    async fn get_property_descriptor_2(
        &self,
        property_name: &str,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_property_descriptor_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersParametersApi].
pub struct ParameterProvidersParametersApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersParametersApi for ParameterProvidersParametersApiDispatch<'_> {
    async fn fetch_parameters(
        &self,
        body: &types::ParameterProviderParameterFetchEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_6_0::types::ParameterProviderParameterFetchEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_7_2::types::ParameterProviderParameterFetchEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_8_0::types::ParameterProviderParameterFetchEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "fetch_parameters".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersReferencesApi].
pub struct ParameterProvidersReferencesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersReferencesApi for ParameterProvidersReferencesApiDispatch<'_> {
    async fn get_parameter_provider_references(
        &self,
    ) -> Result<types::ParameterProviderReferencingComponentsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_parameter_provider_references".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterProvidersStateApi].
pub struct ParameterProvidersStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterProvidersStateApi for ParameterProvidersStateApiDispatch<'_> {
    async fn clear_state_2(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_6_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_7_2::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_8_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_state_2".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_state_1(&self) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_1().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_1().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_1().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_state_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

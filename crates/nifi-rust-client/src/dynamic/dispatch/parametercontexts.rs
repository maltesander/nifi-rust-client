// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterContextsApi;
use crate::dynamic::traits::ParameterContextsAssetsApi;
use crate::dynamic::traits::ParameterContextsUpdateRequestsApi;
use crate::dynamic::traits::ParameterContextsValidationRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ParameterContexts API. Use via the [`ParameterContextsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ParameterContextsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ParameterContextsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ParameterContextsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ParameterContextsApi<'a>),
}
impl<'a> ParameterContextsApiDispatch<'a> {
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
impl ParameterContextsApi for ParameterContextsApiDispatch<'_> {
    fn assets<'b>(&'b self, context_id: &'b str) -> impl ParameterContextsAssetsApi + 'b {
        ParameterContextsAssetsApiDispatch {
            client: self.client(),
            context_id: context_id.to_string(),
            version: self.version(),
        }
    }
    fn update_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsUpdateRequestsApi + 'b {
        ParameterContextsUpdateRequestsApiDispatch {
            client: self.client(),
            context_id: context_id.to_string(),
            version: self.version(),
        }
    }
    fn validation_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsValidationRequestsApi + 'b {
        ParameterContextsValidationRequestsApiDispatch {
            client: self.client(),
            context_id: context_id.to_string(),
            version: self.version(),
        }
    }
    async fn create_parameter_context(
        &self,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_parameter_context(body).await,
            Self::V2_7_2(api) => api.create_parameter_context(body).await,
            Self::V2_8_0(api) => api.create_parameter_context(body).await,
        }
    }
    async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_parameter_context(id, include_inherited_parameters)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_parameter_context(id, include_inherited_parameters)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_parameter_context(id, include_inherited_parameters)
                    .await
            }
        }
    }
    async fn update_parameter_context(
        &self,
        id: &str,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_parameter_context(id, body).await,
            Self::V2_7_2(api) => api.update_parameter_context(id, body).await,
            Self::V2_8_0(api) => api.update_parameter_context(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [ParameterContextsAssetsApi].
pub struct ParameterContextsAssetsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) context_id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterContextsAssetsApi for ParameterContextsAssetsApiDispatch<'_> {
    async fn create_asset(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::AssetDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_asset".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_asset(
        &self,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AssetDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_asset".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_asset_content(&self, asset_id: &str) -> Result<(), NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                api.get_asset_content(asset_id).await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                api.get_asset_content(asset_id).await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                api.get_asset_content(asset_id).await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_asset_content".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_assets(&self) -> Result<types::AssetsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.get_assets().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.get_assets().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id: &self.context_id,
                };
                Ok(api.get_assets().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_assets".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterContextsUpdateRequestsApi].
pub struct ParameterContextsUpdateRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) context_id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterContextsUpdateRequestsApi for ParameterContextsUpdateRequestsApiDispatch<'_> {
    async fn delete_update_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_update_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_parameter_context_update(
        &self,
        request_id: &str,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_parameter_context_update".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_parameter_context_update(
        &self,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_6_0::types::ParameterContextEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_7_2::types::ParameterContextEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_8_0::types::ParameterContextEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_parameter_context_update".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ParameterContextsValidationRequestsApi].
pub struct ParameterContextsValidationRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) context_id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ParameterContextsValidationRequestsApi for ParameterContextsValidationRequestsApiDispatch<'_> {
    async fn delete_validation_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_validation_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_validation_request(
        &self,
        id: &str,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_validation_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_validation_request(
        &self,
        body: &types::ParameterContextValidationRequestEntity,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_6_0::types::ParameterContextValidationRequestEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_7_2::types::ParameterContextValidationRequestEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id: &self.context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_8_0::types::ParameterContextValidationRequestEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_validation_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

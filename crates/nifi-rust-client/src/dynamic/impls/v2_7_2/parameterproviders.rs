// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterProvidersApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ParameterProvidersApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ParameterProvidersApi for V2_7_2ParameterProvidersApi<'_> {
    async fn analyze_configuration_1(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_configuration_1(
                &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_bulletins_4(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_bulletins_4(&crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn clear_state_2(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_state_2(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn delete_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let api =
            crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                client: self.client,
                provider_id,
            };
        Ok(api
            .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
            client: self.client,
            id,
        };
        Ok(api.delete_verification_request_1(request_id).await?.into())
    }
    async fn fetch_parameters(
        &self,
        id: &str,
        body: types::ParameterProviderParameterFetchEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersParametersApi {
            client: self.client,
            id,
        };
        Ok(api
            .fetch_parameters(
                &crate::v2_7_2::types::ParameterProviderParameterFetchEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api.get_parameter_provider(id).await?.into())
    }
    async fn get_parameter_provider_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let api =
            crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                client: self.client,
                provider_id,
            };
        Ok(api
            .get_parameter_provider_apply_parameters_request(request_id)
            .await?
            .into())
    }
    async fn get_parameter_provider_references(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderReferencingComponentsEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersReferencesApi {
            client: self.client,
            id,
        };
        Ok(api.get_parameter_provider_references().await?.into())
    }
    async fn get_property_descriptor_2(
        &self,
        id: &str,
        property_name: &str,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api.get_property_descriptor_2(property_name).await?.into())
    }
    async fn get_state_1(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_state_1().await?.into())
    }
    async fn get_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
            client: self.client,
            id,
        };
        Ok(api.get_verification_request_1(request_id).await?.into())
    }
    async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api
            .remove_parameter_provider(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn submit_apply_parameters(
        &self,
        provider_id: &str,
        body: types::ParameterProviderParameterApplicationEntity,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let api =
            crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                client: self.client,
                provider_id,
            };
        Ok(api
            .submit_apply_parameters(
                &crate::v2_7_2::types::ParameterProviderParameterApplicationEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn submit_config_verification_request_1(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_config_verification_request_1(
                &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api
            .update_parameter_provider(
                id,
                &crate::v2_7_2::types::ParameterProviderEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

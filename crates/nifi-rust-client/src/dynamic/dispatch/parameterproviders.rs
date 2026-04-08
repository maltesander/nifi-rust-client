// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterProvidersApi;
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
impl ParameterProvidersApi for ParameterProvidersApiDispatch<'_> {
    async fn analyze_configuration_1(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.analyze_configuration_1(id, body).await,
            Self::V2_7_2(api) => api.analyze_configuration_1(id, body).await,
            Self::V2_8_0(api) => api.analyze_configuration_1(id, body).await,
        }
    }
    async fn clear_bulletins_4(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_4(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_4(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_4(id, body).await,
        }
    }
    async fn clear_state_2(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_state_2(id, body).await,
            Self::V2_7_2(api) => api.clear_state_2(id, body).await,
            Self::V2_8_0(api) => api.clear_state_2(id, body).await,
        }
    }
    async fn delete_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_apply_parameters_request(
                    provider_id,
                    request_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.delete_apply_parameters_request(
                    provider_id,
                    request_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.delete_apply_parameters_request(
                    provider_id,
                    request_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn delete_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_verification_request_1(id, request_id).await,
            Self::V2_7_2(api) => api.delete_verification_request_1(id, request_id).await,
            Self::V2_8_0(api) => api.delete_verification_request_1(id, request_id).await,
        }
    }
    async fn fetch_parameters(
        &self,
        id: &str,
        body: types::ParameterProviderParameterFetchEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.fetch_parameters(id, body).await,
            Self::V2_7_2(api) => api.fetch_parameters(id, body).await,
            Self::V2_8_0(api) => api.fetch_parameters(id, body).await,
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
    async fn get_parameter_provider_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_parameter_provider_apply_parameters_request(provider_id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_parameter_provider_apply_parameters_request(provider_id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_parameter_provider_apply_parameters_request(provider_id, request_id)
                    .await
            }
        }
    }
    async fn get_parameter_provider_references(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderReferencingComponentsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_parameter_provider_references(id).await,
            Self::V2_7_2(api) => api.get_parameter_provider_references(id).await,
            Self::V2_8_0(api) => api.get_parameter_provider_references(id).await,
        }
    }
    async fn get_property_descriptor_2(
        &self,
        id: &str,
        property_name: &str,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_property_descriptor_2(id, property_name).await,
            Self::V2_7_2(api) => api.get_property_descriptor_2(id, property_name).await,
            Self::V2_8_0(api) => api.get_property_descriptor_2(id, property_name).await,
        }
    }
    async fn get_state_1(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_state_1(id).await,
            Self::V2_7_2(api) => api.get_state_1(id).await,
            Self::V2_8_0(api) => api.get_state_1(id).await,
        }
    }
    async fn get_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_verification_request_1(id, request_id).await,
            Self::V2_7_2(api) => api.get_verification_request_1(id, request_id).await,
            Self::V2_8_0(api) => api.get_verification_request_1(id, request_id).await,
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
    async fn submit_apply_parameters(
        &self,
        provider_id: &str,
        body: types::ParameterProviderParameterApplicationEntity,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_apply_parameters(provider_id, body).await,
            Self::V2_7_2(api) => api.submit_apply_parameters(provider_id, body).await,
            Self::V2_8_0(api) => api.submit_apply_parameters(provider_id, body).await,
        }
    }
    async fn submit_config_verification_request_1(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_config_verification_request_1(id, body).await,
            Self::V2_7_2(api) => api.submit_config_verification_request_1(id, body).await,
            Self::V2_8_0(api) => api.submit_config_verification_request_1(id, body).await,
        }
    }
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_parameter_provider(id, body).await,
            Self::V2_7_2(api) => api.update_parameter_provider(id, body).await,
            Self::V2_8_0(api) => api.update_parameter_provider(id, body).await,
        }
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterContextsApi;
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
impl ParameterContextsApi for ParameterContextsApiDispatch<'_> {
    async fn create_asset(
        &self,
        context_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::AssetDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_asset(context_id, filename, data).await,
            Self::V2_7_2(api) => api.create_asset(context_id, filename, data).await,
            Self::V2_8_0(api) => api.create_asset(context_id, filename, data).await,
        }
    }
    async fn create_parameter_context(
        &self,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_parameter_context(body).await,
            Self::V2_7_2(api) => api.create_parameter_context(body).await,
            Self::V2_8_0(api) => api.create_parameter_context(body).await,
        }
    }
    async fn delete_asset(
        &self,
        context_id: &str,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AssetDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_asset(context_id, asset_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_asset(context_id, asset_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_asset(context_id, asset_id, disconnected_node_acknowledged)
                    .await
            }
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
    async fn delete_update_request(
        &self,
        context_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_update_request(context_id, request_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_update_request(context_id, request_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_update_request(context_id, request_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn delete_validation_request(
        &self,
        context_id: &str,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_validation_request(context_id, id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_validation_request(context_id, id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_validation_request(context_id, id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn get_asset_content(&self, context_id: &str, asset_id: &str) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_asset_content(context_id, asset_id).await,
            Self::V2_7_2(api) => api.get_asset_content(context_id, asset_id).await,
            Self::V2_8_0(api) => api.get_asset_content(context_id, asset_id).await,
        }
    }
    async fn get_assets(&self, context_id: &str) -> Result<types::AssetsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_assets(context_id).await,
            Self::V2_7_2(api) => api.get_assets(context_id).await,
            Self::V2_8_0(api) => api.get_assets(context_id).await,
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
    async fn get_parameter_context_update(
        &self,
        context_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_parameter_context_update(context_id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_parameter_context_update(context_id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_parameter_context_update(context_id, request_id)
                    .await
            }
        }
    }
    async fn get_validation_request(
        &self,
        context_id: &str,
        id: &str,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_validation_request(context_id, id).await,
            Self::V2_7_2(api) => api.get_validation_request(context_id, id).await,
            Self::V2_8_0(api) => api.get_validation_request(context_id, id).await,
        }
    }
    async fn submit_parameter_context_update(
        &self,
        context_id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_parameter_context_update(context_id, body).await,
            Self::V2_7_2(api) => api.submit_parameter_context_update(context_id, body).await,
            Self::V2_8_0(api) => api.submit_parameter_context_update(context_id, body).await,
        }
    }
    async fn submit_validation_request(
        &self,
        context_id: &str,
        body: types::ParameterContextValidationRequestEntity,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_validation_request(context_id, body).await,
            Self::V2_7_2(api) => api.submit_validation_request(context_id, body).await,
            Self::V2_8_0(api) => api.submit_validation_request(context_id, body).await,
        }
    }
    async fn update_parameter_context(
        &self,
        id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_parameter_context(id, body).await,
            Self::V2_7_2(api) => api.update_parameter_context(id, body).await,
            Self::V2_8_0(api) => api.update_parameter_context(id, body).await,
        }
    }
}

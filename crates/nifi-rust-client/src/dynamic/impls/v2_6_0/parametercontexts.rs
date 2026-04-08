// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterContextsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0ParameterContextsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ParameterContextsApi for V2_6_0ParameterContextsApi<'_> {
    async fn create_asset(
        &self,
        context_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::AssetDto, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
            client: self.client,
            context_id,
        };
        Ok(api.create_asset(filename, data).await?.into())
    }
    async fn create_parameter_context(
        &self,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .create_parameter_context(&crate::v2_6_0::types::ParameterContextEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn delete_asset(
        &self,
        context_id: &str,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AssetDto, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
            client: self.client,
            context_id,
        };
        Ok(api
            .delete_asset(asset_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_update_request(
        &self,
        context_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api
            .delete_update_request(request_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_validation_request(
        &self,
        context_id: &str,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api
            .delete_validation_request(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_asset_content(&self, context_id: &str, asset_id: &str) -> Result<(), NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
            client: self.client,
            context_id,
        };
        api.get_asset_content(asset_id).await
    }
    async fn get_assets(&self, context_id: &str) -> Result<types::AssetsEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
            client: self.client,
            context_id,
        };
        Ok(api.get_assets().await?.into())
    }
    async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .get_parameter_context(id, include_inherited_parameters)
            .await?
            .into())
    }
    async fn get_parameter_context_update(
        &self,
        context_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api.get_parameter_context_update(request_id).await?.into())
    }
    async fn get_validation_request(
        &self,
        context_id: &str,
        id: &str,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api.get_validation_request(id).await?.into())
    }
    async fn submit_parameter_context_update(
        &self,
        context_id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api
            .submit_parameter_context_update(
                &crate::v2_6_0::types::ParameterContextEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn submit_validation_request(
        &self,
        context_id: &str,
        body: types::ParameterContextValidationRequestEntity,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
            client: self.client,
            context_id,
        };
        Ok(api
            .submit_validation_request(
                &crate::v2_6_0::types::ParameterContextValidationRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_parameter_context(
        &self,
        id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .update_parameter_context(
                id,
                &crate::v2_6_0::types::ParameterContextEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

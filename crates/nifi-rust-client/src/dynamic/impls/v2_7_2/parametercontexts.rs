// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterContextsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterContextsAssetsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterContextsUpdateRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterContextsValidationRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ParameterContextsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ParameterContextsApi for V2_7_2ParameterContextsApi<'_> {
    type ParameterContextsAssetsApi<'b>
        = crate::dynamic::dispatch::ParameterContextsAssetsApiDispatch<'b>
    where
        Self: 'b;
    fn assets<'b>(&'b self, context_id: &'b str) -> Self::ParameterContextsAssetsApi<'b> {
        crate::dynamic::dispatch::ParameterContextsAssetsApiDispatch {
            client: self.client,
            context_id: context_id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterContextsUpdateRequestsApi<'b>
        = crate::dynamic::dispatch::ParameterContextsUpdateRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn update_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> Self::ParameterContextsUpdateRequestsApi<'b> {
        crate::dynamic::dispatch::ParameterContextsUpdateRequestsApiDispatch {
            client: self.client,
            context_id: context_id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterContextsValidationRequestsApi<'b>
        = crate::dynamic::dispatch::ParameterContextsValidationRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn validation_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> Self::ParameterContextsValidationRequestsApi<'b> {
        crate::dynamic::dispatch::ParameterContextsValidationRequestsApiDispatch {
            client: self.client,
            context_id: context_id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    async fn create_parameter_context(
        &self,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .create_parameter_context(&crate::v2_7_2::types::ParameterContextEntity::try_from(
                body.clone(),
            )?)
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
        let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .get_parameter_context(id, include_inherited_parameters)
            .await?
            .into())
    }
    async fn update_parameter_context(
        &self,
        id: &str,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
            client: self.client,
        };
        Ok(api
            .update_parameter_context(
                id,
                &crate::v2_7_2::types::ParameterContextEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

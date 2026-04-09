// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `assets` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsAssetsApi {
    /// Lists the assets that belong to the Parameter Context with the given ID
    async fn get_assets(&self) -> Result<crate::v2_7_2::types::AssetsEntity, NifiError>;
    /// Creates a new Asset in the given Parameter Context
    async fn create_asset(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<crate::v2_7_2::types::AssetDto, NifiError>;
    /// Deletes an Asset from the given Parameter Context
    async fn delete_asset(
        &self,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::AssetDto, NifiError>;
    /// Retrieves the content of the asset with the given id
    async fn get_asset_content(&self, asset_id: &str) -> Result<(), NifiError>;
}
/// Sub-resource trait for the `update_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsUpdateRequestsApi {
    /// Initiate the Update Request of a Parameter Context
    async fn submit_parameter_context_update(
        &self,
        body: &crate::v2_7_2::types::ParameterContextEntity,
    ) -> Result<crate::v2_7_2::types::ParameterContextUpdateRequestEntity, NifiError>;
    /// Deletes the Update Request with the given ID
    async fn delete_update_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ParameterContextUpdateRequestEntity, NifiError>;
    /// Returns the Update Request with the given ID
    async fn get_parameter_context_update(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::ParameterContextUpdateRequestEntity, NifiError>;
}
/// Sub-resource trait for the `validation_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsValidationRequestsApi {
    /// Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated
    async fn submit_validation_request(
        &self,
        body: &crate::v2_7_2::types::ParameterContextValidationRequestEntity,
    ) -> Result<crate::v2_7_2::types::ParameterContextValidationRequestEntity, NifiError>;
    /// Deletes the Validation Request with the given ID
    async fn delete_validation_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ParameterContextValidationRequestEntity, NifiError>;
    /// Returns the Validation Request with the given ID
    async fn get_validation_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::ParameterContextValidationRequestEntity, NifiError>;
}
/// The ParameterContexts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsApi {
    fn assets<'b>(&'b self, context_id: &'b str) -> impl ParameterContextsAssetsApi + 'b;
    fn update_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsUpdateRequestsApi + 'b;
    fn validation_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsValidationRequestsApi + 'b;
    /// Create a Parameter Context
    async fn create_parameter_context(
        &self,
        body: &crate::v2_7_2::types::ParameterContextEntity,
    ) -> Result<crate::v2_7_2::types::ParameterContextEntity, NifiError>;
    /// Deletes the Parameter Context with the given ID
    async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ParameterContextEntity, NifiError>;
    /// Returns the Parameter Context with the given ID
    async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ParameterContextEntity, NifiError>;
    /// Modifies a Parameter Context
    async fn update_parameter_context(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::ParameterContextEntity,
    ) -> Result<crate::v2_7_2::types::ParameterContextEntity, NifiError>;
}

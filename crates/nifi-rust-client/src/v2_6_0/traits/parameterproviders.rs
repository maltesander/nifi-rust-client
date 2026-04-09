// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `apply_parameters_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersApplyParametersRequestsApi {
    /// Initiate a request to apply the fetched parameters of a Parameter Provider
    async fn submit_apply_parameters(
        &self,
        body: &crate::v2_6_0::types::ParameterProviderParameterApplicationEntity,
    ) -> Result<crate::v2_6_0::types::ParameterProviderApplyParametersRequestDto, NifiError>;
    /// Deletes the Apply Parameters Request with the given ID
    async fn delete_apply_parameters_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ParameterProviderApplyParametersRequestDto, NifiError>;
    /// Returns the Apply Parameters Request with the given ID
    async fn get_parameter_provider_apply_parameters_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::ParameterProviderApplyParametersRequestDto, NifiError>;
}
/// Sub-resource trait for the `config` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_configuration_1(
        &self,
        body: &crate::v2_6_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_6_0::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Parameter Provider's configuration
    async fn submit_config_verification_request_1(
        &self,
        body: &crate::v2_6_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
}
/// Sub-resource trait for the `descriptors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersDescriptorsApi {
    /// Gets a parameter provider property descriptor
    async fn get_property_descriptor_2(
        &self,
        property_name: &str,
    ) -> Result<crate::v2_6_0::types::PropertyDescriptorDto, NifiError>;
}
/// Sub-resource trait for the `parameters` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersParametersApi {
    /// Fetches and temporarily caches the parameters for a provider
    async fn fetch_parameters(
        &self,
        body: &crate::v2_6_0::types::ParameterProviderParameterFetchEntity,
    ) -> Result<crate::v2_6_0::types::ParameterProviderEntity, NifiError>;
}
/// Sub-resource trait for the `references` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersReferencesApi {
    /// Gets all references to a parameter provider
    async fn get_parameter_provider_references(
        &self,
    ) -> Result<crate::v2_6_0::types::ParameterProviderReferencingComponentsEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersStateApi {
    /// Gets the state for a parameter provider
    async fn get_state_1(&self) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError>;
    /// Clears the state for a parameter provider
    async fn clear_state_2(
        &self,
        body: &crate::v2_6_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError>;
}
/// The ParameterProviders API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersApi {
    type ParameterProvidersApplyParametersRequestsApi<
        'b,
    >: ParameterProvidersApplyParametersRequestsApi
    where
        Self: 'b;
    fn apply_parameters_requests<'b>(
        &'b self,
        provider_id: &'b str,
    ) -> Self::ParameterProvidersApplyParametersRequestsApi<'b>;
    type ParameterProvidersConfigApi<'b>: ParameterProvidersConfigApi
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersConfigApi<'b>;
    type ParameterProvidersDescriptorsApi<'b>: ParameterProvidersDescriptorsApi
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersDescriptorsApi<'b>;
    type ParameterProvidersParametersApi<'b>: ParameterProvidersParametersApi
    where
        Self: 'b;
    fn parameters<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersParametersApi<'b>;
    type ParameterProvidersReferencesApi<'b>: ParameterProvidersReferencesApi
    where
        Self: 'b;
    fn references<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersReferencesApi<'b>;
    type ParameterProvidersStateApi<'b>: ParameterProvidersStateApi
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersStateApi<'b>;
    /// Deletes a parameter provider
    async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ParameterProviderEntity, NifiError>;
    /// Gets a parameter provider
    async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ParameterProviderEntity, NifiError>;
    /// Updates a parameter provider
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ParameterProviderEntity,
    ) -> Result<crate::v2_6_0::types::ParameterProviderEntity, NifiError>;
}

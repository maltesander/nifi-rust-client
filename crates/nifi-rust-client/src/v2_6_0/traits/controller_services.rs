// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `config` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_configuration(
        &self,
        body: &crate::v2_6_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_6_0::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Controller Service's configuration
    async fn submit_config_verification_request(
        &self,
        body: &crate::v2_6_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError>;
}
/// Sub-resource trait for the `descriptors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesDescriptorsApi {
    /// Gets a controller service property descriptor
    async fn get_property_descriptor_1(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_6_0::types::PropertyDescriptorDto, NifiError>;
}
/// Sub-resource trait for the `references` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesReferencesApi {
    /// Gets a controller service
    async fn get_controller_service_references(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerServiceReferencingComponentsEntity, NifiError>;
    /// Updates a controller services references
    async fn update_controller_service_references(
        &self,
        body: &crate::v2_6_0::types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<crate::v2_6_0::types::ControllerServiceReferencingComponentsEntity, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesRunStatusApi {
    /// Updates run status of a controller service
    async fn update_run_status_1(
        &self,
        body: &crate::v2_6_0::types::ControllerServiceRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::ControllerServiceEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesStateApi {
    /// Gets the state for a controller service
    async fn get_state(&self) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError>;
    /// Clears the state for a controller service
    async fn clear_state_1(
        &self,
        body: &crate::v2_6_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError>;
}
/// The Controller Services API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesApi {
    fn config<'b>(&'b self, id: &'b str) -> impl ControllerServicesConfigApi + 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ControllerServicesDescriptorsApi + 'b;
    fn references<'b>(&'b self, id: &'b str) -> impl ControllerServicesReferencesApi + 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> impl ControllerServicesRunStatusApi + 'b;
    fn state<'b>(&'b self, id: &'b str) -> impl ControllerServicesStateApi + 'b;
    /// Deletes a controller service
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServiceEntity, NifiError>;
    /// Gets a controller service
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServiceEntity, NifiError>;
    /// Updates a controller service
    async fn update_controller_service(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ControllerServiceEntity,
    ) -> Result<crate::v2_6_0::types::ControllerServiceEntity, NifiError>;
}

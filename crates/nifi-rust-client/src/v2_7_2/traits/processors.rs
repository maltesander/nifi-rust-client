// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `bulletins` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsBulletinsApi {
    /// Clears bulletins for a processor
    async fn clear_bulletins_5(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError>;
}
/// Sub-resource trait for the `config` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_configuration_2(
        &self,
        body: &crate::v2_7_2::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_7_2::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Processor's configuration
    async fn submit_processor_verification_request(
        &self,
        body: &crate::v2_7_2::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
}
/// Sub-resource trait for the `descriptors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsDescriptorsApi {
    /// Gets the descriptor for a processor property
    async fn get_property_descriptor_3(
        &self,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PropertyDescriptorDto, NifiError>;
}
/// Sub-resource trait for the `diagnostics` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsDiagnosticsApi {
    /// Gets diagnostics information about a processor
    async fn get_processor_diagnostics(
        &self,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsRunStatusApi {
    /// Updates run status of a processor
    async fn update_run_status_4(
        &self,
        body: &crate::v2_7_2::types::ProcessorRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsStateApi {
    /// Gets the state for a processor
    async fn get_state_2(&self) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError>;
    /// Clears the state for a processor
    async fn clear_state_3(
        &self,
        body: &crate::v2_7_2::types::ComponentStateEntity,
    ) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError>;
}
/// Sub-resource trait for the `threads` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsThreadsApi {
    /// Terminates a processor, essentially "deleting" its threads and any active tasks
    async fn terminate_processor(&self)
    -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
}
/// The Processors API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsApi {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ProcessorsBulletinsApi + 'b;
    fn config<'b>(&'b self, id: &'b str) -> impl ProcessorsConfigApi + 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ProcessorsDescriptorsApi + 'b;
    fn diagnostics<'b>(&'b self, id: &'b str) -> impl ProcessorsDiagnosticsApi + 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> impl ProcessorsRunStatusApi + 'b;
    fn state<'b>(&'b self, id: &'b str) -> impl ProcessorsStateApi + 'b;
    fn threads<'b>(&'b self, id: &'b str) -> impl ProcessorsThreadsApi + 'b;
    /// Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs
    async fn get_processor_run_status_details(
        &self,
        body: &crate::v2_7_2::types::RunStatusDetailsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity, NifiError>;
    /// Deletes a processor
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
    /// Gets a processor
    async fn get_processor(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
    /// Updates a processor
    async fn update_processor(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::ProcessorEntity,
    ) -> Result<crate::v2_7_2::types::ProcessorEntity, NifiError>;
}

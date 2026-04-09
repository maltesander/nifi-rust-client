// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `bulletins` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksBulletinsApi {
    /// Clears bulletins for a reporting task
    async fn clear_bulletins_7(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError>;
}
/// Sub-resource trait for the `config` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_configuration_3(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Reporting Task's configuration
    async fn submit_config_verification_request_2(
        &self,
        body: &crate::v2_8_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError>;
}
/// Sub-resource trait for the `descriptors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksDescriptorsApi {
    /// Gets a reporting task property descriptor
    async fn get_property_descriptor_4(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PropertyDescriptorDto, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksRunStatusApi {
    /// Updates run status of a reporting task
    async fn update_run_status_5(
        &self,
        body: &crate::v2_8_0::types::ReportingTaskRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::ReportingTaskEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksStateApi {
    /// Gets the state for a reporting task
    async fn get_state_4(&self) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError>;
    /// Clears the state for a reporting task
    async fn clear_state_4(
        &self,
        body: &crate::v2_8_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError>;
}
/// The ReportingTasks API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksApi {
    type ReportingTasksBulletinsApi<'b>: ReportingTasksBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ReportingTasksBulletinsApi<'b>;
    type ReportingTasksConfigApi<'b>: ReportingTasksConfigApi
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ReportingTasksConfigApi<'b>;
    type ReportingTasksDescriptorsApi<'b>: ReportingTasksDescriptorsApi
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ReportingTasksDescriptorsApi<'b>;
    type ReportingTasksRunStatusApi<'b>: ReportingTasksRunStatusApi
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ReportingTasksRunStatusApi<'b>;
    type ReportingTasksStateApi<'b>: ReportingTasksStateApi
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ReportingTasksStateApi<'b>;
    /// Deletes a reporting task
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ReportingTaskEntity, NifiError>;
    /// Gets a reporting task
    async fn get_reporting_task(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::ReportingTaskEntity, NifiError>;
    /// Updates a reporting task
    async fn update_reporting_task(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ReportingTaskEntity,
    ) -> Result<crate::v2_8_0::types::ReportingTaskEntity, NifiError>;
}

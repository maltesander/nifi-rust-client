// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `bulletins` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerBulletinsApi {
    /// Clears bulletins for a flow analysis rule
    async fn clear_flow_analysis_rule_bulletins(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError>;
    /// Clears bulletins for a parameter provider
    async fn clear_parameter_provider_bulletins(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError>;
    /// Clears bulletins for a registry client
    async fn clear_registry_client_bulletins(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError>;
}
/// Sub-resource trait for the `config` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_flow_analysis_rule_configuration(
        &self,
        body: &crate::v2_7_2::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_7_2::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Flow Analysis Rule's configuration
    async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        body: &crate::v2_7_2::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    async fn analyze_flow_registry_client_configuration(
        &self,
        body: &crate::v2_7_2::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_7_2::types::ConfigurationAnalysisDto, NifiError>;
    /// Performs verification of the Registry Client's configuration
    async fn submit_registry_client_config_verification_request(
        &self,
        body: &crate::v2_7_2::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Deletes the Verification Request with the given ID
    async fn delete_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
    /// Returns the Verification Request with the given ID
    async fn get_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError>;
}
/// Sub-resource trait for the `content` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerContentApi {
    /// Retrieves the content of the NAR with the given id
    async fn download_nar(&self) -> Result<(), NifiError>;
}
/// Sub-resource trait for the `descriptors` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerDescriptorsApi {
    /// Gets a flow analysis rule property descriptor
    async fn get_flow_analysis_rule_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PropertyDescriptorDto, NifiError>;
    /// Gets a flow registry client property descriptor
    async fn get_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PropertyDescriptorDto, NifiError>;
}
/// Sub-resource trait for the `details` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerDetailsApi {
    /// Retrieves the component types available from the installed NARs
    async fn get_nar_details(&self) -> Result<crate::v2_7_2::types::NarDetailsEntity, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerRunStatusApi {
    /// Updates run status of a flow analysis rule
    async fn update_run_status(
        &self,
        body: &crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRuleEntity, NifiError>;
}
/// Sub-resource trait for the `state` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerStateApi {
    /// Gets the state for a flow analysis rule
    async fn get_flow_analysis_rule_state(
        &self,
    ) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError>;
    /// Clears the state for a flow analysis rule
    async fn clear_state(
        &self,
        body: &crate::v2_7_2::types::ComponentStateEntity,
    ) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError>;
}
/// The Controller API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerApi {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ControllerBulletinsApi + 'b;
    fn config<'b>(&'b self, id: &'b str) -> impl ControllerConfigApi + 'b;
    fn content<'b>(&'b self, id: &'b str) -> impl ControllerContentApi + 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ControllerDescriptorsApi + 'b;
    fn details<'b>(&'b self, id: &'b str) -> impl ControllerDetailsApi + 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> impl ControllerRunStatusApi + 'b;
    fn state<'b>(&'b self, id: &'b str) -> impl ControllerStateApi + 'b;
    /// Creates a new bulletin
    async fn create_bulletin(
        &self,
        body: &crate::v2_7_2::types::BulletinEntity,
    ) -> Result<crate::v2_7_2::types::BulletinEntity, NifiError>;
    /// Gets the contents of the cluster
    async fn get_cluster(&self) -> Result<crate::v2_7_2::types::ClusterDto, NifiError>;
    /// Removes a node from the cluster
    async fn delete_node(&self, id: &str) -> Result<crate::v2_7_2::types::NodeDto, NifiError>;
    /// Gets a node in the cluster
    async fn get_node(&self, id: &str) -> Result<crate::v2_7_2::types::NodeDto, NifiError>;
    /// Updates a node in the cluster
    async fn update_node(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::NodeEntity,
    ) -> Result<crate::v2_7_2::types::NodeDto, NifiError>;
    /// Retrieves the configuration for this NiFi Controller
    async fn get_controller_config(
        &self,
    ) -> Result<crate::v2_7_2::types::ControllerConfigurationEntity, NifiError>;
    /// Retrieves the configuration for this NiFi
    async fn update_controller_config(
        &self,
        body: &crate::v2_7_2::types::ControllerConfigurationEntity,
    ) -> Result<crate::v2_7_2::types::ControllerConfigurationEntity, NifiError>;
    /// Creates a new controller service
    async fn create_controller_service(
        &self,
        body: &crate::v2_7_2::types::ControllerServiceEntity,
    ) -> Result<crate::v2_7_2::types::ControllerServiceEntity, NifiError>;
    /// Gets all flow analysis rules
    async fn get_flow_analysis_rules(
        &self,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRulesEntity, NifiError>;
    /// Creates a new flow analysis rule
    async fn create_flow_analysis_rule(
        &self,
        body: &crate::v2_7_2::types::FlowAnalysisRuleEntity,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRuleEntity, NifiError>;
    /// Deletes a flow analysis rule
    async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRuleEntity, NifiError>;
    /// Gets a flow analysis rule
    async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRuleEntity, NifiError>;
    /// Updates a flow analysis rule
    async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::FlowAnalysisRuleEntity,
    ) -> Result<crate::v2_7_2::types::FlowAnalysisRuleEntity, NifiError>;
    /// Purges history
    async fn delete_history(
        &self,
        end_date: &str,
    ) -> Result<crate::v2_7_2::types::HistoryDto, NifiError>;
    /// Retrieves summary information for installed NARs
    async fn get_nar_summaries(
        &self,
    ) -> Result<crate::v2_7_2::types::NarSummariesEntity, NifiError>;
    /// Uploads a NAR and requests for it to be installed
    async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<crate::v2_7_2::types::NarSummaryDto, NifiError>;
    /// Deletes an installed NAR
    async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<crate::v2_7_2::types::NarSummaryDto, NifiError>;
    /// Retrieves the summary information for the NAR with the given identifier
    async fn get_nar_summary(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::NarDetailsEntity, NifiError>;
    /// Creates a new parameter provider
    async fn create_parameter_provider(
        &self,
        body: &crate::v2_7_2::types::ParameterProviderEntity,
    ) -> Result<crate::v2_7_2::types::ParameterProviderEntity, NifiError>;
    /// Gets the listing of available flow registry clients
    async fn get_flow_registry_clients(
        &self,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientsEntity, NifiError>;
    /// Creates a new flow registry client
    async fn create_flow_registry_client(
        &self,
        body: &crate::v2_7_2::types::FlowRegistryClientEntity,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientEntity, NifiError>;
    /// Deletes a flow registry client
    async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientEntity, NifiError>;
    /// Gets a flow registry client
    async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientEntity, NifiError>;
    /// Updates a flow registry client
    async fn update_flow_registry_client(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::FlowRegistryClientEntity,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientEntity, NifiError>;
    /// Retrieves the types of flow  that this NiFi supports
    async fn get_registry_client_types(
        &self,
    ) -> Result<crate::v2_7_2::types::FlowRegistryClientTypesEntity, NifiError>;
    /// Creates a new reporting task
    async fn create_reporting_task(
        &self,
        body: &crate::v2_7_2::types::ReportingTaskEntity,
    ) -> Result<crate::v2_7_2::types::ReportingTaskEntity, NifiError>;
    /// Imports a reporting task snapshot
    async fn import_reporting_task_snapshot(
        &self,
        body: &crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<crate::v2_7_2::types::VersionedReportingTaskImportResponseEntity, NifiError>;
    /// Gets status history for the node
    async fn get_node_status_history(
        &self,
    ) -> Result<crate::v2_7_2::types::ComponentHistoryDto, NifiError>;
}

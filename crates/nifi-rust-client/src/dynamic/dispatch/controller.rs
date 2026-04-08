// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Controller API. Use via the [`ControllerApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ControllerApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ControllerApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ControllerApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ControllerApi<'a>),
}
impl ControllerApi for ControllerApiDispatch<'_> {
    async fn analyze_flow_analysis_rule_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.analyze_flow_analysis_rule_configuration(id, body).await,
            Self::V2_7_2(api) => api.analyze_flow_analysis_rule_configuration(id, body).await,
            Self::V2_8_0(api) => api.analyze_flow_analysis_rule_configuration(id, body).await,
        }
    }
    async fn analyze_flow_registry_client_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.analyze_flow_registry_client_configuration(id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.analyze_flow_registry_client_configuration(id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.analyze_flow_registry_client_configuration(id, body)
                    .await
            }
        }
    }
    async fn clear_flow_analysis_rule_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_flow_analysis_rule_bulletins(id, body).await,
            Self::V2_7_2(api) => api.clear_flow_analysis_rule_bulletins(id, body).await,
            Self::V2_8_0(api) => api.clear_flow_analysis_rule_bulletins(id, body).await,
        }
    }
    async fn clear_parameter_provider_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_parameter_provider_bulletins(id, body).await,
            Self::V2_7_2(api) => api.clear_parameter_provider_bulletins(id, body).await,
            Self::V2_8_0(api) => api.clear_parameter_provider_bulletins(id, body).await,
        }
    }
    async fn clear_registry_client_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_registry_client_bulletins(id, body).await,
            Self::V2_7_2(api) => api.clear_registry_client_bulletins(id, body).await,
            Self::V2_8_0(api) => api.clear_registry_client_bulletins(id, body).await,
        }
    }
    async fn clear_state(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_state(id, body).await,
            Self::V2_7_2(api) => api.clear_state(id, body).await,
            Self::V2_8_0(api) => api.clear_state(id, body).await,
        }
    }
    async fn create_bulletin(
        &self,
        body: types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_bulletin(body).await,
            Self::V2_7_2(api) => api.create_bulletin(body).await,
            Self::V2_8_0(api) => api.create_bulletin(body).await,
        }
    }
    async fn create_controller_service(
        &self,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_controller_service(body).await,
            Self::V2_7_2(api) => api.create_controller_service(body).await,
            Self::V2_8_0(api) => api.create_controller_service(body).await,
        }
    }
    async fn create_flow_analysis_rule(
        &self,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_flow_analysis_rule(body).await,
            Self::V2_7_2(api) => api.create_flow_analysis_rule(body).await,
            Self::V2_8_0(api) => api.create_flow_analysis_rule(body).await,
        }
    }
    async fn create_flow_registry_client(
        &self,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_flow_registry_client(body).await,
            Self::V2_7_2(api) => api.create_flow_registry_client(body).await,
            Self::V2_8_0(api) => api.create_flow_registry_client(body).await,
        }
    }
    async fn create_parameter_provider(
        &self,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_parameter_provider(body).await,
            Self::V2_7_2(api) => api.create_parameter_provider(body).await,
            Self::V2_8_0(api) => api.create_parameter_provider(body).await,
        }
    }
    async fn create_reporting_task(
        &self,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_reporting_task(body).await,
            Self::V2_7_2(api) => api.create_reporting_task(body).await,
            Self::V2_8_0(api) => api.create_reporting_task(body).await,
        }
    }
    async fn delete_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
        }
    }
    async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_flow_registry_client(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.delete_flow_registry_client(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.delete_flow_registry_client(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn delete_history(&self, end_date: &str) -> Result<types::HistoryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_history(end_date).await,
            Self::V2_7_2(api) => api.delete_history(end_date).await,
            Self::V2_8_0(api) => api.delete_history(end_date).await,
        }
    }
    async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_nar(id, disconnected_node_acknowledged, force)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_nar(id, disconnected_node_acknowledged, force)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_nar(id, disconnected_node_acknowledged, force)
                    .await
            }
        }
    }
    async fn delete_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_node(id).await,
            Self::V2_7_2(api) => api.delete_node(id).await,
            Self::V2_8_0(api) => api.delete_node(id).await,
        }
    }
    async fn delete_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_registry_client_verification_request(id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_registry_client_verification_request(id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_registry_client_verification_request(id, request_id)
                    .await
            }
        }
    }
    async fn download_nar(&self, id: &str) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.download_nar(id).await,
            Self::V2_7_2(api) => api.download_nar(id).await,
            Self::V2_8_0(api) => api.download_nar(id).await,
        }
    }
    async fn get_cluster(&self) -> Result<types::ClusterDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_cluster().await,
            Self::V2_7_2(api) => api.get_cluster().await,
            Self::V2_8_0(api) => api.get_cluster().await,
        }
    }
    async fn get_controller_config(
        &self,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_controller_config().await,
            Self::V2_7_2(api) => api.get_controller_config().await,
            Self::V2_8_0(api) => api.get_controller_config().await,
        }
    }
    async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_analysis_rule(id).await,
            Self::V2_7_2(api) => api.get_flow_analysis_rule(id).await,
            Self::V2_8_0(api) => api.get_flow_analysis_rule(id).await,
        }
    }
    async fn get_flow_analysis_rule_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_analysis_rule_property_descriptor(id, property_name, sensitive)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_analysis_rule_property_descriptor(id, property_name, sensitive)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_analysis_rule_property_descriptor(id, property_name, sensitive)
                    .await
            }
        }
    }
    async fn get_flow_analysis_rule_state(
        &self,
        id: &str,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_analysis_rule_state(id).await,
            Self::V2_7_2(api) => api.get_flow_analysis_rule_state(id).await,
            Self::V2_8_0(api) => api.get_flow_analysis_rule_state(id).await,
        }
    }
    async fn get_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_analysis_rule_verification_request(id, request_id)
                    .await
            }
        }
    }
    async fn get_flow_analysis_rules(&self) -> Result<types::FlowAnalysisRulesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_analysis_rules().await,
            Self::V2_7_2(api) => api.get_flow_analysis_rules().await,
            Self::V2_8_0(api) => api.get_flow_analysis_rules().await,
        }
    }
    async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_registry_client(id).await,
            Self::V2_7_2(api) => api.get_flow_registry_client(id).await,
            Self::V2_8_0(api) => api.get_flow_registry_client(id).await,
        }
    }
    async fn get_flow_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_registry_clients().await,
            Self::V2_7_2(api) => api.get_flow_registry_clients().await,
            Self::V2_8_0(api) => api.get_flow_registry_clients().await,
        }
    }
    async fn get_nar_details(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_nar_details(id).await,
            Self::V2_7_2(api) => api.get_nar_details(id).await,
            Self::V2_8_0(api) => api.get_nar_details(id).await,
        }
    }
    async fn get_nar_summaries(&self) -> Result<types::NarSummariesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_nar_summaries().await,
            Self::V2_7_2(api) => api.get_nar_summaries().await,
            Self::V2_8_0(api) => api.get_nar_summaries().await,
        }
    }
    async fn get_nar_summary(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_nar_summary(id).await,
            Self::V2_7_2(api) => api.get_nar_summary(id).await,
            Self::V2_8_0(api) => api.get_nar_summary(id).await,
        }
    }
    async fn get_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_node(id).await,
            Self::V2_7_2(api) => api.get_node(id).await,
            Self::V2_8_0(api) => api.get_node(id).await,
        }
    }
    async fn get_node_status_history(&self) -> Result<types::ComponentHistoryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_node_status_history().await,
            Self::V2_7_2(api) => api.get_node_status_history().await,
            Self::V2_8_0(api) => api.get_node_status_history().await,
        }
    }
    async fn get_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_property_descriptor(id, property_name, sensitive)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_property_descriptor(id, property_name, sensitive)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_property_descriptor(id, property_name, sensitive)
                    .await
            }
        }
    }
    async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_registry_client_types().await,
            Self::V2_7_2(api) => api.get_registry_client_types().await,
            Self::V2_8_0(api) => api.get_registry_client_types().await,
        }
    }
    async fn get_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_registry_client_verification_request(id, request_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_registry_client_verification_request(id, request_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_registry_client_verification_request(id, request_id)
                    .await
            }
        }
    }
    async fn import_reporting_task_snapshot(
        &self,
        body: types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<types::VersionedReportingTaskImportResponseEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.import_reporting_task_snapshot(body).await,
            Self::V2_7_2(api) => api.import_reporting_task_snapshot(body).await,
            Self::V2_8_0(api) => api.import_reporting_task_snapshot(body).await,
        }
    }
    async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_flow_analysis_rule(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.remove_flow_analysis_rule(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.remove_flow_analysis_rule(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.submit_flow_analysis_rule_config_verification_request(id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.submit_flow_analysis_rule_config_verification_request(id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.submit_flow_analysis_rule_config_verification_request(id, body)
                    .await
            }
        }
    }
    async fn submit_registry_client_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.submit_registry_client_config_verification_request(id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.submit_registry_client_config_verification_request(id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.submit_registry_client_config_verification_request(id, body)
                    .await
            }
        }
    }
    async fn update_controller_config(
        &self,
        body: types::ControllerConfigurationEntity,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_controller_config(body).await,
            Self::V2_7_2(api) => api.update_controller_config(body).await,
            Self::V2_8_0(api) => api.update_controller_config(body).await,
        }
    }
    async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_flow_analysis_rule(id, body).await,
            Self::V2_7_2(api) => api.update_flow_analysis_rule(id, body).await,
            Self::V2_8_0(api) => api.update_flow_analysis_rule(id, body).await,
        }
    }
    async fn update_flow_registry_client(
        &self,
        id: &str,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_flow_registry_client(id, body).await,
            Self::V2_7_2(api) => api.update_flow_registry_client(id, body).await,
            Self::V2_8_0(api) => api.update_flow_registry_client(id, body).await,
        }
    }
    async fn update_node(
        &self,
        id: &str,
        body: types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_node(id, body).await,
            Self::V2_7_2(api) => api.update_node(id, body).await,
            Self::V2_8_0(api) => api.update_node(id, body).await,
        }
    }
    async fn update_run_status(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_run_status(id, body).await,
            Self::V2_7_2(api) => api.update_run_status(id, body).await,
            Self::V2_8_0(api) => api.update_run_status(id, body).await,
        }
    }
    async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.upload_nar(filename, data).await,
            Self::V2_7_2(api) => api.upload_nar(filename, data).await,
            Self::V2_8_0(api) => api.upload_nar(filename, data).await,
        }
    }
}

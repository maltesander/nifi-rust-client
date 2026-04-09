// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowApi;
use crate::dynamic::traits::FlowBranchesApi;
use crate::dynamic::traits::FlowBreadcrumbsApi;
use crate::dynamic::traits::FlowBucketsApi;
use crate::dynamic::traits::FlowBulletinsApi;
use crate::dynamic::traits::FlowControllerServicesApi;
use crate::dynamic::traits::FlowStatisticsApi;
use crate::dynamic::traits::FlowStatusApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Flow API. Use via the [`FlowApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum FlowApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0FlowApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2FlowApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0FlowApi<'a>),
}
impl<'a> FlowApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl FlowApi for FlowApiDispatch<'_> {
    type FlowBranchesApi<'b>
        = FlowBranchesApiDispatch<'b>
    where
        Self: 'b;
    fn branches<'b>(&'b self, id: &'b str) -> Self::FlowBranchesApi<'b> {
        FlowBranchesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowBreadcrumbsApi<'b>
        = FlowBreadcrumbsApiDispatch<'b>
    where
        Self: 'b;
    fn breadcrumbs<'b>(&'b self, id: &'b str) -> Self::FlowBreadcrumbsApi<'b> {
        FlowBreadcrumbsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowBucketsApi<'b>
        = FlowBucketsApiDispatch<'b>
    where
        Self: 'b;
    fn buckets<'b>(&'b self, id: &'b str) -> Self::FlowBucketsApi<'b> {
        FlowBucketsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowBulletinsApi<'b>
        = FlowBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::FlowBulletinsApi<'b> {
        FlowBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowControllerServicesApi<'b>
        = FlowControllerServicesApiDispatch<'b>
    where
        Self: 'b;
    fn controller_services<'b>(&'b self, id: &'b str) -> Self::FlowControllerServicesApi<'b> {
        FlowControllerServicesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowStatisticsApi<'b>
        = FlowStatisticsApiDispatch<'b>
    where
        Self: 'b;
    fn statistics<'b>(&'b self, id: &'b str) -> Self::FlowStatisticsApi<'b> {
        FlowStatisticsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    type FlowStatusApi<'b>
        = FlowStatusApiDispatch<'b>
    where
        Self: 'b;
    fn status<'b>(&'b self, id: &'b str) -> Self::FlowStatusApi<'b> {
        FlowStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
        }
    }
    async fn generate_client_id(&self) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.generate_client_id().await,
            Self::V2_7_2(api) => api.generate_client_id().await,
            Self::V2_8_0(api) => api.generate_client_id().await,
        }
    }
    async fn get_about_info(&self) -> Result<types::AboutDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_about_info().await,
            Self::V2_7_2(api) => api.get_about_info().await,
            Self::V2_8_0(api) => api.get_about_info().await,
        }
    }
    async fn get_action(&self, id: &str) -> Result<types::ActionEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_action(id).await,
            Self::V2_7_2(api) => api.get_action(id).await,
            Self::V2_8_0(api) => api.get_action(id).await,
        }
    }
    async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::AdditionalDetailsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_additional_details(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_additional_details(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_additional_details(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_all_flow_analysis_results().await,
            Self::V2_7_2(api) => api.get_all_flow_analysis_results().await,
            Self::V2_8_0(api) => api.get_all_flow_analysis_results().await,
        }
    }
    async fn get_banners(&self) -> Result<types::BannerDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_banners().await,
            Self::V2_7_2(api) => api.get_banners().await,
            Self::V2_8_0(api) => api.get_banners().await,
        }
    }
    async fn get_bulletin_board(
        &self,
        after: Option<&str>,
        source_name: Option<&str>,
        message: Option<&str>,
        source_id: Option<&str>,
        group_id: Option<&str>,
        limit: Option<&str>,
    ) -> Result<types::BulletinBoardDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await
            }
        }
    }
    async fn get_bulletins(&self) -> Result<types::ControllerBulletinsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_bulletins().await,
            Self::V2_7_2(api) => api.get_bulletins().await,
            Self::V2_8_0(api) => api.get_bulletins().await,
        }
    }
    async fn get_cluster_summary(&self) -> Result<types::ClusterSummaryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_cluster_summary().await,
            Self::V2_7_2(api) => api.get_cluster_summary().await,
            Self::V2_8_0(api) => api.get_cluster_summary().await,
        }
    }
    async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<types::ComponentHistoryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_component_history(component_id).await,
            Self::V2_7_2(api) => api.get_component_history(component_id).await,
            Self::V2_8_0(api) => api.get_component_history(component_id).await,
        }
    }
    async fn get_content_viewers(&self) -> Result<types::ContentViewerEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_content_viewers().await,
            Self::V2_7_2(api) => api.get_content_viewers().await,
            Self::V2_8_0(api) => api.get_content_viewers().await,
        }
    }
    async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ControllerServiceDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_controller_service_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_controller_service_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_controller_service_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_controller_service_types(
        &self,
        service_type: Option<&str>,
        service_bundle_group: Option<&str>,
        service_bundle_artifact: Option<&str>,
        service_bundle_version: Option<&str>,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        type_filter: Option<&str>,
    ) -> Result<types::ControllerServiceTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_controller_service_types(
                    service_type,
                    service_bundle_group,
                    service_bundle_artifact,
                    service_bundle_version,
                    bundle_group_filter,
                    bundle_artifact_filter,
                    type_filter,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.get_controller_service_types(
                    service_type,
                    service_bundle_group,
                    service_bundle_artifact,
                    service_bundle_version,
                    bundle_group_filter,
                    bundle_artifact_filter,
                    type_filter,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.get_controller_service_types(
                    service_type,
                    service_bundle_group,
                    service_bundle_artifact,
                    service_bundle_version,
                    bundle_group_filter,
                    bundle_artifact_filter,
                    type_filter,
                )
                .await
            }
        }
    }
    async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_controller_services_from_controller(ui_only, include_referencing_components)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_controller_services_from_controller(ui_only, include_referencing_components)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_controller_services_from_controller(ui_only, include_referencing_components)
                    .await
            }
        }
    }
    async fn get_controller_status(&self) -> Result<types::ControllerStatusDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_controller_status().await,
            Self::V2_7_2(api) => api.get_controller_status().await,
            Self::V2_8_0(api) => api.get_controller_status().await,
        }
    }
    async fn get_current_user(&self) -> Result<types::CurrentUserEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_current_user().await,
            Self::V2_7_2(api) => api.get_current_user().await,
            Self::V2_8_0(api) => api.get_current_user().await,
        }
    }
    async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ProcessGroupFlowEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow(id, ui_only).await,
            Self::V2_7_2(api) => api.get_flow(id, ui_only).await,
            Self::V2_8_0(api) => api.get_flow(id, ui_only).await,
        }
    }
    async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_analysis_results(process_group_id).await,
            Self::V2_7_2(api) => api.get_flow_analysis_results(process_group_id).await,
            Self::V2_8_0(api) => api.get_flow_analysis_results(process_group_id).await,
        }
    }
    async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowAnalysisRuleDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::FlowAnalysisRuleTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_analysis_rule_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_analysis_rule_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_analysis_rule_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
        }
    }
    async fn get_flow_config(&self) -> Result<types::FlowConfigurationDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_config().await,
            Self::V2_7_2(api) => api.get_flow_config().await,
            Self::V2_8_0(api) => api.get_flow_config().await,
        }
    }
    async fn get_flow_metrics(
        &self,
        producer: &str,
        included_registries: Option<types::IncludedRegistries>,
        sample_name: Option<&str>,
        sample_label_value: Option<&str>,
        root_field_name: Option<&str>,
        flow_metrics_reporting_strategy: Option<types::FlowMetricsReportingStrategy>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_metrics(
                    producer,
                    included_registries,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                    flow_metrics_reporting_strategy,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_metrics(
                    producer,
                    included_registries,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                    flow_metrics_reporting_strategy,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_metrics(
                    producer,
                    included_registries,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                    flow_metrics_reporting_strategy,
                )
                .await
            }
        }
    }
    async fn get_flow_registry_client_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowRegistryClientDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_flow_registry_client_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_flow_registry_client_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_flow_registry_client_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_listen_ports(&self) -> Result<types::ListenPortsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_listen_ports().await,
            Self::V2_7_2(api) => api.get_listen_ports().await,
            Self::V2_8_0(api) => api.get_listen_ports().await,
        }
    }
    async fn get_parameter_contexts(&self) -> Result<types::ParameterContextsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_parameter_contexts().await,
            Self::V2_7_2(api) => api.get_parameter_contexts().await,
            Self::V2_8_0(api) => api.get_parameter_contexts().await,
        }
    }
    async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ParameterProviderDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_parameter_provider_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_parameter_provider_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_parameter_provider_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ParameterProviderTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_parameter_provider_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.get_parameter_provider_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.get_parameter_provider_types(
                    bundle_group_filter,
                    bundle_artifact_filter,
                    r#type,
                )
                .await
            }
        }
    }
    async fn get_parameter_providers(&self) -> Result<types::ParameterProvidersEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_parameter_providers().await,
            Self::V2_7_2(api) => api.get_parameter_providers().await,
            Self::V2_8_0(api) => api.get_parameter_providers().await,
        }
    }
    async fn get_prioritizers(&self) -> Result<types::PrioritizerTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_prioritizers().await,
            Self::V2_7_2(api) => api.get_prioritizers().await,
            Self::V2_8_0(api) => api.get_prioritizers().await,
        }
    }
    async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ProcessorDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_processor_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_processor_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_processor_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ProcessorTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
        }
    }
    async fn get_registry_clients(&self) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_registry_clients().await,
            Self::V2_7_2(api) => api.get_registry_clients().await,
            Self::V2_8_0(api) => api.get_registry_clients().await,
        }
    }
    async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ReportingTaskDefinition, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_reporting_task_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_reporting_task_definition(group, artifact, version, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_reporting_task_definition(group, artifact, version, r#type)
                    .await
            }
        }
    }
    async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<types::VersionedReportingTaskSnapshot, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_reporting_task_snapshot(reporting_task_id).await,
            Self::V2_7_2(api) => api.get_reporting_task_snapshot(reporting_task_id).await,
            Self::V2_8_0(api) => api.get_reporting_task_snapshot(reporting_task_id).await,
        }
    }
    async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ReportingTaskTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await
            }
        }
    }
    async fn get_reporting_tasks(&self) -> Result<types::ReportingTasksEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_reporting_tasks().await,
            Self::V2_7_2(api) => api.get_reporting_tasks().await,
            Self::V2_8_0(api) => api.get_reporting_tasks().await,
        }
    }
    async fn get_runtime_manifest(&self) -> Result<types::RuntimeManifest, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_runtime_manifest().await,
            Self::V2_7_2(api) => api.get_runtime_manifest().await,
            Self::V2_8_0(api) => api.get_runtime_manifest().await,
        }
    }
    async fn query_history(
        &self,
        offset: &str,
        count: &str,
        sort_column: Option<&str>,
        sort_order: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        user_identity: Option<&str>,
        source_id: Option<&str>,
    ) -> Result<types::HistoryDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.query_history(
                    offset,
                    count,
                    sort_column,
                    sort_order,
                    start_date,
                    end_date,
                    user_identity,
                    source_id,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.query_history(
                    offset,
                    count,
                    sort_column,
                    sort_order,
                    start_date,
                    end_date,
                    user_identity,
                    source_id,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.query_history(
                    offset,
                    count,
                    sort_column,
                    sort_order,
                    start_date,
                    end_date,
                    user_identity,
                    source_id,
                )
                .await
            }
        }
    }
    async fn schedule_components(
        &self,
        id: &str,
        body: &types::ScheduleComponentsEntity,
    ) -> Result<types::ScheduleComponentsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.schedule_components(id, body).await,
            Self::V2_7_2(api) => api.schedule_components(id, body).await,
            Self::V2_8_0(api) => api.schedule_components(id, body).await,
        }
    }
    async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<types::ClusterSearchResultsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.search_cluster(q).await,
            Self::V2_7_2(api) => api.search_cluster(q).await,
            Self::V2_8_0(api) => api.search_cluster(q).await,
        }
    }
    async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<types::SearchResultsDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.search_flow(q, a).await,
            Self::V2_7_2(api) => api.search_flow(q, a).await,
            Self::V2_8_0(api) => api.search_flow(q, a).await,
        }
    }
}
/// Sub-resource dispatch struct for [FlowBranchesApi].
pub struct FlowBranchesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowBranchesApi for FlowBranchesApiDispatch<'_> {
    async fn get_branches(&self) -> Result<types::FlowRegistryBranchesEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_branches().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_branches().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_branches().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_branches".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_version_differences(
        &self,
        registry_id: &str,
        branch_id_a: &str,
        bucket_id_a: &str,
        flow_id_a: &str,
        version_a: &str,
        branch_id_b: &str,
        bucket_id_b: &str,
        flow_id_b: &str,
        version_b: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_version_differences".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowBreadcrumbsApi].
pub struct FlowBreadcrumbsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowBreadcrumbsApi for FlowBreadcrumbsApiDispatch<'_> {
    async fn get_breadcrumbs(&self) -> Result<types::FlowBreadcrumbEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_breadcrumbs".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowBucketsApi].
pub struct FlowBucketsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowBucketsApi for FlowBucketsApiDispatch<'_> {
    async fn get_buckets(
        &self,
        branch: Option<&str>,
    ) -> Result<types::FlowRegistryBucketsEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_buckets".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_details(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowDto, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_details".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_flows(
        &self,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowsEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flows".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_versions(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_versions".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowBulletinsApi].
pub struct FlowBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowBulletinsApi for FlowBulletinsApiDispatch<'_> {
    async fn clear_bulletins_1(
        &self,
        body: &types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<types::ClearBulletinsForGroupResultsEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_1".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_1(
                        &crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_bulletins_1(
                        &crate::v2_8_0::types::ClearBulletinsForGroupRequestEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_1".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowControllerServicesApi].
pub struct FlowControllerServicesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowControllerServicesApi for FlowControllerServicesApiDispatch<'_> {
    async fn activate_controller_services(
        &self,
        body: &types::ActivateControllerServicesEntity,
    ) -> Result<types::ActivateControllerServicesEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_6_0::types::ActivateControllerServicesEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_7_2::types::ActivateControllerServicesEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_8_0::types::ActivateControllerServicesEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "activate_controller_services".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_controller_services_from_group(
        &self,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_controller_services_from_group".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowStatisticsApi].
pub struct FlowStatisticsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowStatisticsApi for FlowStatisticsApiDispatch<'_> {
    async fn get_connection_statistics(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatisticsEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_connection_statistics".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowStatusApi].
pub struct FlowStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowStatusApi for FlowStatusApiDispatch<'_> {
    async fn get_connection_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_connection_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_connection_status_history(&self) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_connection_status_history".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_input_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_input_port_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_output_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_output_port_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_process_group_status(
        &self,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessGroupStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_process_group_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_process_group_status_history(
        &self,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_process_group_status_history".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_processor_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessorStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_processor_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_processor_status_history(&self) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_processor_status_history".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_remote_process_group_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::RemoteProcessGroupStatusEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_remote_process_group_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_remote_process_group_status_history(
        &self,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_remote_process_group_status_history".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

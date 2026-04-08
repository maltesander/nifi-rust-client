// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowApi;
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
impl FlowApi for FlowApiDispatch<'_> {
    async fn activate_controller_services(
        &self,
        id: &str,
        body: types::ActivateControllerServicesEntity,
    ) -> Result<types::ActivateControllerServicesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.activate_controller_services(id, body).await,
            Self::V2_7_2(api) => api.activate_controller_services(id, body).await,
            Self::V2_8_0(api) => api.activate_controller_services(id, body).await,
        }
    }
    async fn clear_bulletins_1(
        &self,
        id: &str,
        body: types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<types::ClearBulletinsForGroupResultsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_1(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_1(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_1(id, body).await,
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
    async fn get_branches(&self, id: &str) -> Result<types::FlowRegistryBranchesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_branches(id).await,
            Self::V2_7_2(api) => api.get_branches(id).await,
            Self::V2_8_0(api) => api.get_branches(id).await,
        }
    }
    async fn get_breadcrumbs(&self, id: &str) -> Result<types::FlowBreadcrumbEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_breadcrumbs(id).await,
            Self::V2_7_2(api) => api.get_breadcrumbs(id).await,
            Self::V2_8_0(api) => api.get_breadcrumbs(id).await,
        }
    }
    async fn get_buckets(
        &self,
        id: &str,
        branch: Option<&str>,
    ) -> Result<types::FlowRegistryBucketsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_buckets(id, branch).await,
            Self::V2_7_2(api) => api.get_buckets(id, branch).await,
            Self::V2_8_0(api) => api.get_buckets(id, branch).await,
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
    async fn get_connection_statistics(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatisticsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_connection_statistics(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_connection_statistics(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_connection_statistics(id, nodewise, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_connection_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_connection_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_connection_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_connection_status(id, nodewise, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_connection_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_connection_status_history(id).await,
            Self::V2_7_2(api) => api.get_connection_status_history(id).await,
            Self::V2_8_0(api) => api.get_connection_status_history(id).await,
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
    async fn get_controller_services_from_group(
        &self,
        id: &str,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_controller_services_from_group(
                    id,
                    include_ancestor_groups,
                    include_descendant_groups,
                    include_referencing_components,
                    ui_only,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.get_controller_services_from_group(
                    id,
                    include_ancestor_groups,
                    include_descendant_groups,
                    include_referencing_components,
                    ui_only,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.get_controller_services_from_group(
                    id,
                    include_ancestor_groups,
                    include_descendant_groups,
                    include_referencing_components,
                    ui_only,
                )
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
    async fn get_details(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_details(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_details(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_details(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
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
    async fn get_flows(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flows(id, registry_id, bucket_id, branch).await,
            Self::V2_7_2(api) => api.get_flows(id, registry_id, bucket_id, branch).await,
            Self::V2_8_0(api) => api.get_flows(id, registry_id, bucket_id, branch).await,
        }
    }
    async fn get_input_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_input_port_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_input_port_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_input_port_status(id, nodewise, cluster_node_id)
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
    async fn get_output_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_output_port_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_output_port_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_output_port_status(id, nodewise, cluster_node_id)
                    .await
            }
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
    async fn get_process_group_status(
        &self,
        id: &str,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessGroupStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_process_group_status(id, recursive, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_process_group_status(id, recursive, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_process_group_status(id, recursive, nodewise, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_process_group_status_history(id).await,
            Self::V2_7_2(api) => api.get_process_group_status_history(id).await,
            Self::V2_8_0(api) => api.get_process_group_status_history(id).await,
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
    async fn get_processor_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessorStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_processor_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_processor_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_processor_status(id, nodewise, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_processor_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor_status_history(id).await,
            Self::V2_7_2(api) => api.get_processor_status_history(id).await,
            Self::V2_8_0(api) => api.get_processor_status_history(id).await,
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
    async fn get_remote_process_group_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::RemoteProcessGroupStatusEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_remote_process_group_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_remote_process_group_status(id, nodewise, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_remote_process_group_status(id, nodewise, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_remote_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_remote_process_group_status_history(id).await,
            Self::V2_7_2(api) => api.get_remote_process_group_status_history(id).await,
            Self::V2_8_0(api) => api.get_remote_process_group_status_history(id).await,
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
    async fn get_version_differences(
        &self,
        id: &str,
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
        match self {
            Self::V2_6_0(api) => {
                api.get_version_differences(
                    id,
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
                .await
            }
            Self::V2_7_2(api) => {
                api.get_version_differences(
                    id,
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
                .await
            }
            Self::V2_8_0(api) => {
                api.get_version_differences(
                    id,
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
                .await
            }
        }
    }
    async fn get_versions(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_versions(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_versions(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_versions(id, registry_id, bucket_id, flow_id, branch)
                    .await
            }
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
        body: types::ScheduleComponentsEntity,
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

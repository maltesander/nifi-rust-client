// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2FlowApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl FlowApi for V2_7_2FlowApi<'_> {
    async fn activate_controller_services(
        &self,
        id: &str,
        body: types::ActivateControllerServicesEntity,
    ) -> Result<types::ActivateControllerServicesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
            client: self.client,
            id,
        };
        Ok(api
            .activate_controller_services(
                &crate::v2_7_2::types::ActivateControllerServicesEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_bulletins_1(
        &self,
        id: &str,
        body: types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<types::ClearBulletinsForGroupResultsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_bulletins_1(
                &crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        api.download_reporting_task_snapshot(reporting_task_id)
            .await
    }
    async fn generate_client_id(&self) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        api.generate_client_id().await
    }
    async fn get_about_info(&self) -> Result<types::AboutDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_about_info().await?.into())
    }
    async fn get_action(&self, id: &str) -> Result<types::ActionEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_action(id).await?.into())
    }
    async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::AdditionalDetailsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_additional_details(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_all_flow_analysis_results().await?.into())
    }
    async fn get_banners(&self) -> Result<types::BannerDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_banners().await?.into())
    }
    async fn get_branches(&self, id: &str) -> Result<types::FlowRegistryBranchesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBranchesApi {
            client: self.client,
            id,
        };
        Ok(api.get_branches().await?.into())
    }
    async fn get_breadcrumbs(&self, id: &str) -> Result<types::FlowBreadcrumbEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBreadcrumbsApi {
            client: self.client,
            id,
        };
        Ok(api.get_breadcrumbs().await?.into())
    }
    async fn get_buckets(
        &self,
        id: &str,
        branch: Option<&str>,
    ) -> Result<types::FlowRegistryBucketsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBucketsApi {
            client: self.client,
            id,
        };
        Ok(api.get_buckets(branch).await?.into())
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
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_bulletin_board(after, source_name, message, source_id, group_id, limit)
            .await?
            .into())
    }
    async fn get_bulletins(&self) -> Result<types::ControllerBulletinsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_bulletins().await?.into())
    }
    async fn get_cluster_summary(&self) -> Result<types::ClusterSummaryDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_cluster_summary().await?.into())
    }
    async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<types::ComponentHistoryDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_component_history(component_id).await?.into())
    }
    async fn get_connection_statistics(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatisticsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatisticsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_connection_statistics(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_connection_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_connection_status(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_connection_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api.get_connection_status_history().await?.into())
    }
    async fn get_content_viewers(&self) -> Result<types::ContentViewerEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_content_viewers().await?.into())
    }
    async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ControllerServiceDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_controller_service_definition(group, artifact, version, r#type)
            .await?
            .into())
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
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_controller_service_types(
                service_type,
                service_bundle_group,
                service_bundle_artifact,
                service_bundle_version,
                bundle_group_filter,
                bundle_artifact_filter,
                type_filter,
            )
            .await?
            .into())
    }
    async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_controller_services_from_controller(ui_only, include_referencing_components)
            .await?
            .into())
    }
    async fn get_controller_services_from_group(
        &self,
        id: &str,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
            client: self.client,
            id,
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
    async fn get_controller_status(&self) -> Result<types::ControllerStatusDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_controller_status().await?.into())
    }
    async fn get_current_user(&self) -> Result<types::CurrentUserEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_current_user().await?.into())
    }
    async fn get_details(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBucketsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_details(registry_id, bucket_id, flow_id, branch)
            .await?
            .into())
    }
    async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ProcessGroupFlowEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_flow(id, ui_only).await?.into())
    }
    async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_flow_analysis_results(process_group_id)
            .await?
            .into())
    }
    async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowAnalysisRuleDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_flow_analysis_rule_definition(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::FlowAnalysisRuleTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_flow_analysis_rule_types(bundle_group_filter, bundle_artifact_filter, r#type)
            .await?
            .into())
    }
    async fn get_flow_config(&self) -> Result<types::FlowConfigurationDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_flow_config().await?.into())
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
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        api.get_flow_metrics(
            producer,
            included_registries
                .map(crate::v2_7_2::types::IncludedRegistries::try_from)
                .transpose()?,
            sample_name,
            sample_label_value,
            root_field_name,
            flow_metrics_reporting_strategy
                .map(crate::v2_7_2::types::FlowMetricsReportingStrategy::try_from)
                .transpose()?,
        )
        .await
    }
    async fn get_flow_registry_client_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowRegistryClientDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_flow_registry_client_definition(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_flows(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBucketsApi {
            client: self.client,
            id,
        };
        Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
    }
    async fn get_input_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_input_port_status(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_listen_ports(&self) -> Result<types::ListenPortsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_listen_ports().await?.into())
    }
    async fn get_output_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_output_port_status(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_parameter_contexts(&self) -> Result<types::ParameterContextsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_parameter_contexts().await?.into())
    }
    async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ParameterProviderDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_parameter_provider_definition(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ParameterProviderTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_parameter_provider_types(bundle_group_filter, bundle_artifact_filter, r#type)
            .await?
            .into())
    }
    async fn get_parameter_providers(&self) -> Result<types::ParameterProvidersEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_parameter_providers().await?.into())
    }
    async fn get_prioritizers(&self) -> Result<types::PrioritizerTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_prioritizers().await?.into())
    }
    async fn get_process_group_status(
        &self,
        id: &str,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessGroupStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_process_group_status(recursive, nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api.get_process_group_status_history().await?.into())
    }
    async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ProcessorDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_processor_definition(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_processor_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessorStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_processor_status(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_processor_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api.get_processor_status_history().await?.into())
    }
    async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ProcessorTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
            .await?
            .into())
    }
    async fn get_registry_clients(&self) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_registry_clients().await?.into())
    }
    async fn get_remote_process_group_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::RemoteProcessGroupStatusEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_remote_process_group_status(nodewise, cluster_node_id)
            .await?
            .into())
    }
    async fn get_remote_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowStatusApi {
            client: self.client,
            id,
        };
        Ok(api.get_remote_process_group_status_history().await?.into())
    }
    async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ReportingTaskDefinition, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_reporting_task_definition(group, artifact, version, r#type)
            .await?
            .into())
    }
    async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<types::VersionedReportingTaskSnapshot, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_reporting_task_snapshot(reporting_task_id)
            .await?
            .into())
    }
    async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ReportingTaskTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
            .await?
            .into())
    }
    async fn get_reporting_tasks(&self) -> Result<types::ReportingTasksEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_reporting_tasks().await?.into())
    }
    async fn get_runtime_manifest(&self) -> Result<types::RuntimeManifest, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.get_runtime_manifest().await?.into())
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
        let api = crate::v2_7_2::api::flow::FlowBranchesApi {
            client: self.client,
            id,
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
    async fn get_versions(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowBucketsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_versions(registry_id, bucket_id, flow_id, branch)
            .await?
            .into())
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
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .query_history(
                offset,
                count,
                sort_column,
                sort_order,
                start_date,
                end_date,
                user_identity,
                source_id,
            )
            .await?
            .into())
    }
    async fn schedule_components(
        &self,
        id: &str,
        body: types::ScheduleComponentsEntity,
    ) -> Result<types::ScheduleComponentsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api
            .schedule_components(
                id,
                &crate::v2_7_2::types::ScheduleComponentsEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<types::ClusterSearchResultsEntity, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.search_cluster(q).await?.into())
    }
    async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<types::SearchResultsDto, NifiError> {
        let api = crate::v2_7_2::api::flow::FlowApi {
            client: self.client,
        };
        Ok(api.search_flow(q, a).await?.into())
    }
}

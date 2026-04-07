// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct FlowApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowApi<'a> {
    /// Retrieves details about this NiFi to put in the About dialog
    ///
    /// Calls `GET /nifi-api/flow/about`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_about_info(&self) -> Result<crate::v2_6_0::types::AboutDto, NifiError> {
        let e: crate::v2_6_0::types::AboutEntity = self.client.get("/flow/about").await?;
        Ok(e.about)
    }
    /// Retrieves the additional details for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/additional-details/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The processor type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The additional details for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::AdditionalDetailsEntity, NifiError> {
        self.client
            .get(&format!(
                "/flow/additional-details/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the banners for this NiFi
    ///
    /// Calls `GET /nifi-api/flow/banners`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_banners(&self) -> Result<crate::v2_6_0::types::BannerDto, NifiError> {
        let e: crate::v2_6_0::types::BannerEntity = self.client.get("/flow/banners").await?;
        Ok(e.banners)
    }
    /// Gets current bulletins
    ///
    /// Calls `GET /nifi-api/flow/bulletin-board`.
    ///
    /// # Parameters
    /// - `after`: Includes bulletins with an id after this value.
    /// - `source_name`: Includes bulletins originating from this sources whose name match this regular expression.
    /// - `message`: Includes bulletins whose message that match this regular expression.
    /// - `source_id`: Includes bulletins originating from this sources whose id match this regular expression.
    /// - `group_id`: Includes bulletins originating from this sources whose group id match this regular expression.
    /// - `limit`: The number of bulletins to limit the response to.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /flow`
    /// - `Read - /{component-type}/{uuid} - For component specific bulletins`
    pub async fn get_bulletin_board(
        &self,
        after: Option<&str>,
        source_name: Option<&str>,
        message: Option<&str>,
        source_id: Option<&str>,
        group_id: Option<&str>,
        limit: Option<&str>,
    ) -> Result<crate::v2_6_0::types::BulletinBoardDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = after {
            query.push(("after", v.to_string()));
        }
        if let Some(v) = source_name {
            query.push(("sourceName", v.to_string()));
        }
        if let Some(v) = message {
            query.push(("message", v.to_string()));
        }
        if let Some(v) = source_id {
            query.push(("sourceId", v.to_string()));
        }
        if let Some(v) = group_id {
            query.push(("groupId", v.to_string()));
        }
        if let Some(v) = limit {
            query.push(("limit", v.to_string()));
        }
        let e: crate::v2_6_0::types::BulletinBoardEntity = self
            .client
            .get_with_query("/flow/bulletin-board", &query)
            .await?;
        Ok(e.bulletin_board)
    }
    /// Generates a client id.
    ///
    /// Calls `GET /nifi-api/flow/client-id`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn generate_client_id(&self) -> Result<(), NifiError> {
        self.client.get_void("/flow/client-id").await
    }
    /// Searches the cluster for a node with the specified address
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/cluster/search-results`.
    ///
    /// # Parameters
    /// - `q`: Node address to search for.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<crate::v2_6_0::types::ClusterSearchResultsEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("q", q.to_string()));
        self.client
            .get_with_query("/flow/cluster/search-results", &query)
            .await
    }
    /// The cluster summary for this NiFi
    ///
    /// Calls `GET /nifi-api/flow/cluster/summary`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_cluster_summary(
        &self,
    ) -> Result<crate::v2_6_0::types::ClusterSummaryDto, NifiError> {
        let e: crate::v2_6_0::types::ClusterSummaryEntity =
            self.client.get("/flow/cluster/summary").await?;
        Ok(e.cluster_summary)
    }
    /// Retrieves the configuration for this NiFi flow
    ///
    /// Calls `GET /nifi-api/flow/config`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow_config(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowConfigurationDto, NifiError> {
        let e: crate::v2_6_0::types::FlowConfigurationEntity =
            self.client.get("/flow/config").await?;
        Ok(e.flow_configuration)
    }
    /// Retrieves the registered content viewers
    ///
    /// Calls `GET /nifi-api/flow/content-viewers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_content_viewers(
        &self,
    ) -> Result<crate::v2_6_0::types::ContentViewerEntity, NifiError> {
        self.client.get("/flow/content-viewers").await
    }
    /// Retrieves the Controller Service Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/controller-service-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The controller service type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The controller service definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ControllerServiceDefinition, NifiError> {
        self.client
            .get(&format!(
                "/flow/controller-service-definition/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the types of controller services that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/controller-service-types`.
    ///
    /// # Parameters
    /// - `service_type`: If specified, will only return controller services that are compatible with this type of service.
    /// - `service_bundle_group`: If serviceType specified, is the bundle group of the serviceType.
    /// - `service_bundle_artifact`: If serviceType specified, is the bundle artifact of the serviceType.
    /// - `service_bundle_version`: If serviceType specified, is the bundle version of the serviceType.
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type_filter`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_controller_service_types(
        &self,
        service_type: Option<&str>,
        service_bundle_group: Option<&str>,
        service_bundle_artifact: Option<&str>,
        service_bundle_version: Option<&str>,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        type_filter: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ControllerServiceTypesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = service_type {
            query.push(("serviceType", v.to_string()));
        }
        if let Some(v) = service_bundle_group {
            query.push(("serviceBundleGroup", v.to_string()));
        }
        if let Some(v) = service_bundle_artifact {
            query.push(("serviceBundleArtifact", v.to_string()));
        }
        if let Some(v) = service_bundle_version {
            query.push(("serviceBundleVersion", v.to_string()));
        }
        if let Some(v) = bundle_group_filter {
            query.push(("bundleGroupFilter", v.to_string()));
        }
        if let Some(v) = bundle_artifact_filter {
            query.push(("bundleArtifactFilter", v.to_string()));
        }
        if let Some(v) = type_filter {
            query.push(("typeFilter", v.to_string()));
        }
        self.client
            .get_with_query("/flow/controller-service-types", &query)
            .await
    }
    /// Retrieves Controller level bulletins
    ///
    /// Calls `GET /nifi-api/flow/controller/bulletins`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /flow`
    /// - `Read - /controller - For controller bulletins`
    /// - `Read - /controller-services/{uuid} - For controller service bulletins`
    /// - `Read - /reporting-tasks/{uuid} - For reporting task bulletins`
    pub async fn get_bulletins(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerBulletinsEntity, NifiError> {
        self.client.get("/flow/controller/bulletins").await
    }
    /// Gets controller services for reporting tasks
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/controller/controller-services`.
    ///
    /// # Parameters
    /// - `include_referencing_components`: Whether or not to include services' referencing components in the response
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServicesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = ui_only {
            query.push(("uiOnly", v.to_string()));
        }
        if let Some(v) = include_referencing_components {
            query.push(("includeReferencingComponents", v.to_string()));
        }
        self.client
            .get_with_query("/flow/controller/controller-services", &query)
            .await
    }
    /// Retrieves the user identity of the user making the request
    ///
    /// Calls `GET /nifi-api/flow/current-user`.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_current_user(
        &self,
    ) -> Result<crate::v2_6_0::types::CurrentUserEntity, NifiError> {
        self.client.get("/flow/current-user").await
    }
    /// Retrieves the Flow Analysis Rule Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis-rule-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The flow analysis rule type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The flow analysis rule definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisRuleDefinition, NifiError> {
        self.client
            .get(&format!(
                "/flow/flow-analysis-rule-definition/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the types of available Flow Analysis Rules
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis-rule-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisRuleTypesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bundle_group_filter {
            query.push(("bundleGroupFilter", v.to_string()));
        }
        if let Some(v) = bundle_artifact_filter {
            query.push(("bundleArtifactFilter", v.to_string()));
        }
        if let Some(v) = r#type {
            query.push(("type", v.to_string()));
        }
        self.client
            .get_with_query("/flow/flow-analysis-rule-types", &query)
            .await
    }
    /// Returns all flow analysis results currently in effect
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis/results`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisResultEntity, NifiError> {
        self.client.get("/flow/flow-analysis/results").await
    }
    /// Returns flow analysis results produced by the analysis of a given process group
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis/results/{processGroupId}`.
    ///
    /// # Parameters
    /// - `process_group_id`: The id of the process group representing (a part of) the flow to be analyzed.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisResultEntity, NifiError> {
        self.client
            .get(&format!("/flow/flow-analysis/results/{process_group_id}"))
            .await
    }
    /// Gets configuration history
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history`.
    ///
    /// # Parameters
    /// - `offset`: The offset into the result set.
    /// - `count`: The number of actions to return.
    /// - `sort_column`: The field to sort on.
    /// - `sort_order`: The direction to sort.
    /// - `start_date`: Include actions after this date.
    /// - `end_date`: Include actions before this date.
    /// - `user_identity`: Include actions performed by this user.
    /// - `source_id`: Include actions on this component.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn query_history(
        &self,
        offset: &str,
        count: &str,
        sort_column: Option<&str>,
        sort_order: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        user_identity: Option<&str>,
        source_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::HistoryDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("offset", offset.to_string()));
        query.push(("count", count.to_string()));
        if let Some(v) = sort_column {
            query.push(("sortColumn", v.to_string()));
        }
        if let Some(v) = sort_order {
            query.push(("sortOrder", v.to_string()));
        }
        if let Some(v) = start_date {
            query.push(("startDate", v.to_string()));
        }
        if let Some(v) = end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = user_identity {
            query.push(("userIdentity", v.to_string()));
        }
        if let Some(v) = source_id {
            query.push(("sourceId", v.to_string()));
        }
        let e: crate::v2_6_0::types::HistoryEntity =
            self.client.get_with_query("/flow/history", &query).await?;
        Ok(e.history)
    }
    /// Gets configuration history for a component
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history/components/{componentId}`.
    ///
    /// # Parameters
    /// - `component_id`: The component id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /flow`
    /// - `Read underlying component - /{component-type}/{uuid}`
    pub async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<crate::v2_6_0::types::ComponentHistoryDto, NifiError> {
        let e: crate::v2_6_0::types::ComponentHistoryEntity = self
            .client
            .get(&format!("/flow/history/components/{component_id}"))
            .await?;
        Ok(e.component_history)
    }
    /// Gets an action
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history/{id}`.
    ///
    /// # Parameters
    /// - `id`: The action id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_action(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ActionEntity, NifiError> {
        self.client.get(&format!("/flow/history/{id}")).await
    }
    /// Gets all metrics for the flow from a particular node
    ///
    /// Calls `GET /nifi-api/flow/metrics/{producer}`.
    ///
    /// # Parameters
    /// - `producer`: The producer for flow file metrics. Each producer may have its own output format.
    /// - `included_registries`: Set of included metrics registries. Duplicate the parameter to include multiple registries. All registries are included by default.
    /// - `sample_name`: Regular Expression Pattern to be applied against the sample name field
    /// - `sample_label_value`: Regular Expression Pattern to be applied against the sample label value field
    /// - `root_field_name`: Name of the first field of JSON object. Applicable for JSON producer only.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow_metrics(
        &self,
        producer: &str,
        included_registries: Option<crate::v2_6_0::types::IncludedRegistries>,
        sample_name: Option<&str>,
        sample_label_value: Option<&str>,
        root_field_name: Option<&str>,
    ) -> Result<(), NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = included_registries {
            query.push(("includedRegistries", v.to_string()));
        }
        if let Some(v) = sample_name {
            query.push(("sampleName", v.to_string()));
        }
        if let Some(v) = sample_label_value {
            query.push(("sampleLabelValue", v.to_string()));
        }
        if let Some(v) = root_field_name {
            query.push(("rootFieldName", v.to_string()));
        }
        self.client
            .get_void_with_query(&format!("/flow/metrics/{producer}"), &query)
            .await
    }
    /// Gets all Parameter Contexts
    ///
    /// Calls `GET /nifi-api/flow/parameter-contexts`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id} for each Parameter Context`.
    pub async fn get_parameter_contexts(
        &self,
    ) -> Result<crate::v2_6_0::types::ParameterContextsEntity, NifiError> {
        self.client.get("/flow/parameter-contexts").await
    }
    /// Retrieves the Parameter Provider Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/parameter-provider-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The parameter provider type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The reporting task definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ParameterProviderDefinition, NifiError> {
        self.client
            .get(&format!(
                "/flow/parameter-provider-definition/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the types of parameter providers that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/parameter-provider-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ParameterProviderTypesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bundle_group_filter {
            query.push(("bundleGroupFilter", v.to_string()));
        }
        if let Some(v) = bundle_artifact_filter {
            query.push(("bundleArtifactFilter", v.to_string()));
        }
        if let Some(v) = r#type {
            query.push(("type", v.to_string()));
        }
        self.client
            .get_with_query("/flow/parameter-provider-types", &query)
            .await
    }
    /// Gets all parameter providers
    ///
    /// Calls `GET /nifi-api/flow/parameter-providers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_parameter_providers(
        &self,
    ) -> Result<crate::v2_6_0::types::ParameterProvidersEntity, NifiError> {
        self.client.get("/flow/parameter-providers").await
    }
    /// Retrieves the types of prioritizers that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/prioritizers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_prioritizers(
        &self,
    ) -> Result<crate::v2_6_0::types::PrioritizerTypesEntity, NifiError> {
        self.client.get("/flow/prioritizers").await
    }
    /// Gets a process group
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupFlowEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = ui_only {
            query.push(("uiOnly", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/process-groups/{id}"), &query)
            .await
    }
    /// Schedule or unschedule components in the specified Process Group.
    ///
    /// Calls `PUT /nifi-api/flow/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The request to schedule or unschedule. If the components in the request are not specified, all authorized components will be considered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /flow`
    /// - `Write - /{component-type}/{uuid} or /operation/{component-type}/{uuid} - For every component being scheduled/unscheduled`
    pub async fn schedule_components(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ScheduleComponentsEntity,
    ) -> Result<crate::v2_6_0::types::ScheduleComponentsEntity, NifiError> {
        self.client
            .put(&format!("/flow/process-groups/{id}"), body)
            .await
    }
    /// Retrieves the Processor Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/processor-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The processor type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The processor definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ProcessorDefinition, NifiError> {
        self.client
            .get(&format!(
                "/flow/processor-definition/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the types of processors that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/processor-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessorTypesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bundle_group_filter {
            query.push(("bundleGroupFilter", v.to_string()));
        }
        if let Some(v) = bundle_artifact_filter {
            query.push(("bundleArtifactFilter", v.to_string()));
        }
        if let Some(v) = r#type {
            query.push(("type", v.to_string()));
        }
        self.client
            .get_with_query("/flow/processor-types", &query)
            .await
    }
    /// Gets the listing of available flow registry clients
    ///
    /// Calls `GET /nifi-api/flow/registries`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_registry_clients(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowRegistryClientsEntity, NifiError> {
        self.client.get("/flow/registries").await
    }
    /// Retrieves the Reporting Task Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/reporting-task-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The reporting task type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The reporting task definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ReportingTaskDefinition, NifiError> {
        self.client
            .get(&format!(
                "/flow/reporting-task-definition/{group}/{artifact}/{version}/{type}"
            ))
            .await
    }
    /// Retrieves the types of reporting tasks that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/reporting-task-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ReportingTaskTypesEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bundle_group_filter {
            query.push(("bundleGroupFilter", v.to_string()));
        }
        if let Some(v) = bundle_artifact_filter {
            query.push(("bundleArtifactFilter", v.to_string()));
        }
        if let Some(v) = r#type {
            query.push(("type", v.to_string()));
        }
        self.client
            .get_with_query("/flow/reporting-task-types", &query)
            .await
    }
    /// Gets all reporting tasks
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_reporting_tasks(
        &self,
    ) -> Result<crate::v2_6_0::types::ReportingTasksEntity, NifiError> {
        self.client.get("/flow/reporting-tasks").await
    }
    /// Download a snapshot of the given reporting tasks and any controller services they use
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks/download`.
    ///
    /// # Parameters
    /// - `reporting_task_id`: Specifies a reporting task id to export. If not specified, all reporting tasks will be exported.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = reporting_task_id {
            query.push(("reportingTaskId", v.to_string()));
        }
        self.client
            .get_void_with_query("/flow/reporting-tasks/download", &query)
            .await
    }
    /// Get a snapshot of the given reporting tasks and any controller services they use
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks/snapshot`.
    ///
    /// # Parameters
    /// - `reporting_task_id`: Specifies a reporting task id to export. If not specified, all reporting tasks will be exported.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedReportingTaskSnapshot, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = reporting_task_id {
            query.push(("reportingTaskId", v.to_string()));
        }
        self.client
            .get_with_query("/flow/reporting-tasks/snapshot", &query)
            .await
    }
    /// Retrieves the runtime manifest for this NiFi instance.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/runtime-manifest`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_runtime_manifest(
        &self,
    ) -> Result<crate::v2_6_0::types::RuntimeManifest, NifiError> {
        let e: crate::v2_6_0::types::RuntimeManifestEntity =
            self.client.get("/flow/runtime-manifest").await?;
        Ok(e.runtime_manifest)
    }
    /// Performs a search against this NiFi using the specified search term
    ///
    /// Only search results from authorized components will be returned.
    ///
    /// Calls `GET /nifi-api/flow/search-results`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<crate::v2_6_0::types::SearchResultsDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = q {
            query.push(("q", v.to_string()));
        }
        if let Some(v) = a {
            query.push(("a", v.to_string()));
        }
        let e: crate::v2_6_0::types::SearchResultsEntity = self
            .client
            .get_with_query("/flow/search-results", &query)
            .await?;
        Ok(e.search_results_d_t_o)
    }
    /// Gets the current status of this NiFi
    ///
    /// Calls `GET /nifi-api/flow/status`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_controller_status(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerStatusDto, NifiError> {
        let e: crate::v2_6_0::types::ControllerStatusEntity =
            self.client.get("/flow/status").await?;
        Ok(e.controller_status)
    }
    /// Scope operations to the `branches` sub-resource of a specific process group.
    ///
    /// - `id`: The registry id.
    pub fn branches<'b>(&'b self, id: &'b str) -> FlowBranchesApi<'b> {
        FlowBranchesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `breadcrumbs` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn breadcrumbs<'b>(&'b self, id: &'b str) -> FlowBreadcrumbsApi<'b> {
        FlowBreadcrumbsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `buckets` sub-resource of a specific process group.
    ///
    /// - `id`: The registry id.
    pub fn buckets<'b>(&'b self, id: &'b str) -> FlowBucketsApi<'b> {
        FlowBucketsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `controller_services` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn controller_services<'b>(&'b self, id: &'b str) -> FlowControllerServicesApi<'b> {
        FlowControllerServicesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `statistics` sub-resource of a specific process group.
    ///
    /// - `id`: The connection id.
    pub fn statistics<'b>(&'b self, id: &'b str) -> FlowStatisticsApi<'b> {
        FlowStatisticsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `status` sub-resource of a specific process group.
    ///
    /// - `id`: The connection id.
    pub fn status<'b>(&'b self, id: &'b str) -> FlowStatusApi<'b> {
        FlowStatusApi {
            client: self.client,
            id,
        }
    }
}
pub struct FlowBranchesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowBranchesApi<'a> {
    /// Gets the branches from the specified registry for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{id}/branches`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_branches(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowRegistryBranchesEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/registries/{id}/branches"))
            .await
    }
    /// Gets the differences between two versions of the same versioned flow, the basis of the comparison will be the first version
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/branches/{branch-id-a}/buckets/{bucket-id-a}/flows/{flow-id-a}/{version-a}/diff/branches/{branch-id-b}/buckets/{bucket-id-b}/flows/{flow-id-b}/{version-b}`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `branch_id_a`: The branch id for the base version.
    /// - `bucket_id_a`: The bucket id for the base version.
    /// - `flow_id_a`: The flow id for the base version.
    /// - `version_a`: The base version.
    /// - `branch_id_b`: The branch id for the compared version.
    /// - `bucket_id_b`: The bucket id for the compared version.
    /// - `flow_id_b`: The flow id for the compared version.
    /// - `version_b`: The compared version.
    /// - `offset`: Must be a non-negative number. Specifies the starting point of the listing. 0 means start from the beginning.
    /// - `limit`: Limits the number of differences listed. This might lead to partial result. 0 means no limitation is applied.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_version_differences(
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
    ) -> Result<crate::v2_6_0::types::FlowComparisonEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = offset {
            query.push(("offset", v.to_string()));
        }
        if let Some(v) = limit {
            query.push(("limit", v.to_string()));
        }
        self.client
            .get_with_query(
                &format!(
                    "/flow/registries/{registry_id}/branches/{branch_id_a}/buckets/{bucket_id_a}/flows/{flow_id_a}/{version_a}/diff/branches/{branch_id_b}/buckets/{bucket_id_b}/flows/{flow_id_b}/{version_b}"
                ),
                &query,
            )
            .await
    }
}
pub struct FlowBreadcrumbsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowBreadcrumbsApi<'a> {
    /// Gets the breadcrumbs for a process group
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/breadcrumbs`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_breadcrumbs(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowBreadcrumbEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/process-groups/{id}/breadcrumbs"))
            .await
    }
}
pub struct FlowBucketsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowBucketsApi<'a> {
    /// Gets the buckets from the specified registry for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{id}/buckets`.
    ///
    /// # Parameters
    /// - `branch`: The name of a branch to get the buckets from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_buckets(
        &self,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::FlowRegistryBucketsEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = branch {
            query.push(("branch", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/registries/{id}/buckets"), &query)
            .await
    }
    /// Gets the flows from the specified registry and bucket for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `branch`: The name of a branch to get the flows from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_flows(
        &self,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowsEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = branch {
            query.push(("branch", v.to_string()));
        }
        self.client
            .get_with_query(
                &format!("/flow/registries/{registry_id}/buckets/{bucket_id}/flows"),
                &query,
            )
            .await
    }
    /// Gets the details of a flow from the specified registry and bucket for the specified flow for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows/{flow-id}/details`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `flow_id`: The flow id.
    /// - `branch`: The name of a branch to get the flow from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_details(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = branch {
            query.push(("branch", v.to_string()));
        }
        let e: crate::v2_6_0::types::VersionedFlowEntity = self
            .client
            .get_with_query(
                &format!(
                    "/flow/registries/{registry_id}/buckets/{bucket_id}/flows/{flow_id}/details"
                ),
                &query,
            )
            .await?;
        Ok(e.versioned_flow)
    }
    /// Gets the flow versions from the specified registry and bucket for the specified flow for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows/{flow-id}/versions`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `flow_id`: The flow id.
    /// - `branch`: The name of a branch to get the flow versions from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_versions(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = branch {
            query.push(("branch", v.to_string()));
        }
        self.client
            .get_with_query(
                &format!(
                    "/flow/registries/{registry_id}/buckets/{bucket_id}/flows/{flow_id}/versions"
                ),
                &query,
            )
            .await
    }
}
pub struct FlowControllerServicesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowControllerServicesApi<'a> {
    /// Gets all controller services
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/controller-services`.
    ///
    /// # Parameters
    /// - `include_ancestor_groups`: Whether or not to include parent/ancestor process groups
    /// - `include_descendant_groups`: Whether or not to include descendant process groups
    /// - `include_referencing_components`: Whether or not to include services' referencing components in the response
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_controller_services_from_group(
        &self,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServicesEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = include_ancestor_groups {
            query.push(("includeAncestorGroups", v.to_string()));
        }
        if let Some(v) = include_descendant_groups {
            query.push(("includeDescendantGroups", v.to_string()));
        }
        if let Some(v) = include_referencing_components {
            query.push(("includeReferencingComponents", v.to_string()));
        }
        if let Some(v) = ui_only {
            query.push(("uiOnly", v.to_string()));
        }
        self.client
            .get_with_query(
                &format!("/flow/process-groups/{id}/controller-services"),
                &query,
            )
            .await
    }
    /// Enable or disable Controller Services in the specified Process Group.
    ///
    /// Calls `PUT /nifi-api/flow/process-groups/{id}/controller-services`.
    ///
    /// # Parameters
    /// - `body`: The request to schedule or unschedule. If the comopnents in the request are not specified, all authorized components will be considered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /flow`
    /// - `Write - /{component-type}/{uuid} or /operation/{component-type}/{uuid} - For every service being enabled/disabled`
    pub async fn activate_controller_services(
        &self,
        body: &crate::v2_6_0::types::ActivateControllerServicesEntity,
    ) -> Result<crate::v2_6_0::types::ActivateControllerServicesEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/flow/process-groups/{id}/controller-services"),
                body,
            )
            .await
    }
}
pub struct FlowStatisticsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowStatisticsApi<'a> {
    /// Gets statistics for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/statistics`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the statistics.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_connection_statistics(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ConnectionStatisticsEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/connections/{id}/statistics"), &query)
            .await
    }
}
pub struct FlowStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowStatusApi<'a> {
    /// Gets status for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_connection_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ConnectionStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/connections/{id}/status"), &query)
            .await
    }
    /// Gets the status history for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_connection_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/connections/{id}/status/history"))
            .await
    }
    /// Gets status for an input port
    ///
    /// Calls `GET /nifi-api/flow/input-ports/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_input_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::PortStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/input-ports/{id}/status"), &query)
            .await
    }
    /// Gets status for an output port
    ///
    /// Calls `GET /nifi-api/flow/output-ports/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_output_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::PortStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/output-ports/{id}/status"), &query)
            .await
    }
    /// Gets the status for a process group
    ///
    /// The status for a process group includes status for all descendent components. When invoked on the root group with recursive set to true, it will return the current status of every component in the flow.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/status`.
    ///
    /// # Parameters
    /// - `recursive`: Whether all descendant groups and the status of their content will be included. Optional, defaults to false
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_process_group_status(
        &self,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = recursive {
            query.push(("recursive", v.to_string()));
        }
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/process-groups/{id}/status"), &query)
            .await
    }
    /// Gets status history for a remote process group
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_process_group_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/process-groups/{id}/status/history"))
            .await
    }
    /// Gets status for a processor
    ///
    /// Calls `GET /nifi-api/flow/processors/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_processor_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessorStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/processors/{id}/status"), &query)
            .await
    }
    /// Gets status history for a processor
    ///
    /// Calls `GET /nifi-api/flow/processors/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_processor_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/processors/{id}/status/history"))
            .await
    }
    /// Gets status for a remote process group
    ///
    /// Calls `GET /nifi-api/flow/remote-process-groups/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_remote_process_group_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupStatusEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/flow/remote-process-groups/{id}/status"), &query)
            .await
    }
    /// Gets the status history
    ///
    /// Calls `GET /nifi-api/flow/remote-process-groups/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    pub async fn get_remote_process_group_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/flow/remote-process-groups/{id}/status/history"))
            .await
    }
}

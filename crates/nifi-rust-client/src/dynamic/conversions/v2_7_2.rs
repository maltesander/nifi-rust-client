// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(clippy::useless_conversion, clippy::redundant_closure)]

fn enum_to_string(v: &impl serde::Serialize) -> String {
    serde_json::to_value(v)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default()
}

impl From<crate::v2_7_2::types::AboutDto> for super::super::types::AboutDto {
    fn from(v: crate::v2_7_2::types::AboutDto) -> Self {
        Self {
            build_branch: v.build_branch,
            build_revision: v.build_revision,
            build_tag: v.build_tag,
            build_timestamp: v.build_timestamp,
            content_viewer_url: v.content_viewer_url,
            timezone: v.timezone,
            title: v.title,
            uri: v.uri,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::AboutEntity> for super::super::types::AboutEntity {
    fn from(v: crate::v2_7_2::types::AboutEntity) -> Self {
        Self {
            about: Some(v.about.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::AccessPolicyDto> for super::super::types::AccessPolicyDto {
    fn from(v: crate::v2_7_2::types::AccessPolicyDto) -> Self {
        Self {
            action: v.action.map(|v| enum_to_string(&v)),
            component_reference: v.component_reference.map(Into::into),
            configurable: v.configurable,
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            resource: v.resource,
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::AccessPolicyEntity> for super::super::types::AccessPolicyEntity {
    fn from(v: crate::v2_7_2::types::AccessPolicyEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            generated: v.generated,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::AccessPolicySummaryDto>
    for super::super::types::AccessPolicySummaryDto
{
    fn from(v: crate::v2_7_2::types::AccessPolicySummaryDto) -> Self {
        Self {
            action: v.action.map(|v| enum_to_string(&v)),
            component_reference: v.component_reference.map(Into::into),
            configurable: v.configurable,
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            resource: v.resource,
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::AccessPolicySummaryEntity>
    for super::super::types::AccessPolicySummaryEntity
{
    fn from(v: crate::v2_7_2::types::AccessPolicySummaryEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ActionDetailsDto> for super::super::types::ActionDetailsDto {
    fn from(_v: crate::v2_7_2::types::ActionDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::ActionDto> for super::super::types::ActionDto {
    fn from(v: crate::v2_7_2::types::ActionDto) -> Self {
        Self {
            action_details: v.action_details.map(Into::into),
            component_details: v.component_details.map(Into::into),
            id: v.id,
            operation: v.operation,
            source_id: v.source_id,
            source_name: v.source_name,
            source_type: v.source_type,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        }
    }
}

impl From<crate::v2_7_2::types::ActionEntity> for super::super::types::ActionEntity {
    fn from(v: crate::v2_7_2::types::ActionEntity) -> Self {
        Self {
            action: v.action.map(Into::into),
            can_read: v.can_read,
            id: v.id,
            source_id: v.source_id,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::ActivateControllerServicesEntity>
    for super::super::types::ActivateControllerServicesEntity
{
    fn from(v: crate::v2_7_2::types::ActivateControllerServicesEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::AdditionalDetailsEntity>
    for super::super::types::AdditionalDetailsEntity
{
    fn from(v: crate::v2_7_2::types::AdditionalDetailsEntity) -> Self {
        Self {
            additional_details: v.additional_details,
        }
    }
}

impl From<crate::v2_7_2::types::AffectedComponentDto>
    for super::super::types::AffectedComponentDto
{
    fn from(v: crate::v2_7_2::types::AffectedComponentDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| enum_to_string(&v)),
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::AffectedComponentEntity>
    for super::super::types::AffectedComponentEntity
{
    fn from(v: crate::v2_7_2::types::AffectedComponentEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            process_group: v.process_group.map(Into::into),
            reference_type: v.reference_type.map(|v| enum_to_string(&v)),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::AllowableValueDto> for super::super::types::AllowableValueDto {
    fn from(v: crate::v2_7_2::types::AllowableValueDto) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::AllowableValueEntity>
    for super::super::types::AllowableValueEntity
{
    fn from(v: crate::v2_7_2::types::AllowableValueEntity) -> Self {
        Self {
            allowable_value: v.allowable_value.map(Into::into),
            can_read: v.can_read,
        }
    }
}

impl From<crate::v2_7_2::types::AssetDto> for super::super::types::AssetDto {
    fn from(v: crate::v2_7_2::types::AssetDto) -> Self {
        Self {
            digest: v.digest,
            id: v.id,
            missing_content: v.missing_content,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AssetEntity> for super::super::types::AssetEntity {
    fn from(v: crate::v2_7_2::types::AssetEntity) -> Self {
        Self {
            asset: Some(v.asset.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::AssetReferenceDto> for super::super::types::AssetReferenceDto {
    fn from(v: crate::v2_7_2::types::AssetReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AssetsEntity> for super::super::types::AssetsEntity {
    fn from(v: crate::v2_7_2::types::AssetsEntity) -> Self {
        Self {
            assets: v.assets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::Attribute> for super::super::types::Attribute {
    fn from(v: crate::v2_7_2::types::Attribute) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AttributeDto> for super::super::types::AttributeDto {
    fn from(v: crate::v2_7_2::types::AttributeDto) -> Self {
        Self {
            name: v.name,
            previous_value: v.previous_value,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::AuthenticationConfigurationDto>
    for super::super::types::AuthenticationConfigurationDto
{
    fn from(v: crate::v2_7_2::types::AuthenticationConfigurationDto) -> Self {
        Self {
            external_login_required: v.external_login_required,
            login_supported: v.login_supported,
            login_uri: v.login_uri,
            logout_uri: v.logout_uri,
        }
    }
}

impl From<crate::v2_7_2::types::AuthenticationConfigurationEntity>
    for super::super::types::AuthenticationConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::AuthenticationConfigurationEntity) -> Self {
        Self {
            authentication_configuration: Some(
                v.authentication_configuration.unwrap_or_default().into(),
            ),
        }
    }
}

impl From<crate::v2_7_2::types::BannerDto> for super::super::types::BannerDto {
    fn from(v: crate::v2_7_2::types::BannerDto) -> Self {
        Self {
            footer_text: v.footer_text,
            header_text: v.header_text,
        }
    }
}

impl From<crate::v2_7_2::types::BannerEntity> for super::super::types::BannerEntity {
    fn from(v: crate::v2_7_2::types::BannerEntity) -> Self {
        Self {
            banners: Some(v.banners.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::BatchSettingsDto> for super::super::types::BatchSettingsDto {
    fn from(v: crate::v2_7_2::types::BatchSettingsDto) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_7_2::types::BatchSize> for super::super::types::BatchSize {
    fn from(v: crate::v2_7_2::types::BatchSize) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_7_2::types::BuildInfo> for super::super::types::BuildInfo {
    fn from(v: crate::v2_7_2::types::BuildInfo) -> Self {
        Self {
            compiler: v.compiler,
            compiler_flags: v.compiler_flags,
            revision: v.revision,
            target_arch: v.target_arch,
            timestamp: v.timestamp,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinBoardDto> for super::super::types::BulletinBoardDto {
    fn from(v: crate::v2_7_2::types::BulletinBoardDto) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinBoardEntity> for super::super::types::BulletinBoardEntity {
    fn from(v: crate::v2_7_2::types::BulletinBoardEntity) -> Self {
        Self {
            bulletin_board: Some(v.bulletin_board.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::BulletinBoardPatternParameter>
    for super::super::types::BulletinBoardPatternParameter
{
    fn from(v: crate::v2_7_2::types::BulletinBoardPatternParameter) -> Self {
        Self {
            pattern: v.pattern,
            raw_pattern: v.raw_pattern,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinDto> for super::super::types::BulletinDto {
    fn from(v: crate::v2_7_2::types::BulletinDto) -> Self {
        Self {
            category: v.category,
            group_id: v.group_id,
            id: v.id,
            level: v.level,
            message: v.message,
            node_address: v.node_address,
            source_id: v.source_id,
            source_name: v.source_name,
            source_type: v.source_type,
            stack_trace: v.stack_trace,
            timestamp: v.timestamp,
            timestamp_iso: v.timestamp_iso,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinEntity> for super::super::types::BulletinEntity {
    fn from(v: crate::v2_7_2::types::BulletinEntity) -> Self {
        Self {
            bulletin: v.bulletin.map(Into::into),
            can_read: v.can_read,
            group_id: v.group_id,
            id: v.id,
            node_address: v.node_address,
            source_id: v.source_id,
            timestamp: v.timestamp,
            timestamp_iso: v.timestamp_iso,
        }
    }
}

impl From<crate::v2_7_2::types::Bundle> for super::super::types::Bundle {
    fn from(v: crate::v2_7_2::types::Bundle) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::BundleDto> for super::super::types::BundleDto {
    fn from(v: crate::v2_7_2::types::BundleDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity>
    for super::super::types::ClearBulletinsForGroupRequestEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity) -> Self {
        Self {
            components: v.components,
            from_timestamp: Some(v.from_timestamp),
            id: v.id,
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsForGroupResultsEntity>
    for super::super::types::ClearBulletinsForGroupResultsEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsForGroupResultsEntity) -> Self {
        Self {
            bulletins_cleared: v.bulletins_cleared,
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsRequestEntity>
    for super::super::types::ClearBulletinsRequestEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsRequestEntity) -> Self {
        Self {
            from_timestamp: Some(v.from_timestamp),
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsResultEntity>
    for super::super::types::ClearBulletinsResultEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsResultEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            bulletins_cleared: v.bulletins_cleared,
            component_id: v.component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ClientIdParameter> for super::super::types::ClientIdParameter {
    fn from(v: crate::v2_7_2::types::ClientIdParameter) -> Self {
        Self {
            client_id: v.client_id,
        }
    }
}

impl From<crate::v2_7_2::types::ClusterDto> for super::super::types::ClusterDto {
    fn from(v: crate::v2_7_2::types::ClusterDto) -> Self {
        Self {
            generated: v.generated,
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterEntity> for super::super::types::ClusterEntity {
    fn from(v: crate::v2_7_2::types::ClusterEntity) -> Self {
        Self {
            cluster: Some(v.cluster.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterSearchResultsEntity>
    for super::super::types::ClusterSearchResultsEntity
{
    fn from(v: crate::v2_7_2::types::ClusterSearchResultsEntity) -> Self {
        Self {
            node_results: v
                .node_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterSummaryDto> for super::super::types::ClusterSummaryDto {
    fn from(v: crate::v2_7_2::types::ClusterSummaryDto) -> Self {
        Self {
            clustered: v.clustered,
            connected_node_count: v.connected_node_count,
            connected_nodes: v.connected_nodes,
            connected_to_cluster: v.connected_to_cluster,
            total_node_count: v.total_node_count,
        }
    }
}

impl From<crate::v2_7_2::types::ClusterSummaryEntity>
    for super::super::types::ClusterSummaryEntity
{
    fn from(v: crate::v2_7_2::types::ClusterSummaryEntity) -> Self {
        Self {
            cluster_summary: Some(v.cluster_summary.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentDetailsDto> for super::super::types::ComponentDetailsDto {
    fn from(_v: crate::v2_7_2::types::ComponentDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::ComponentDifferenceDto>
    for super::super::types::ComponentDifferenceDto
{
    fn from(v: crate::v2_7_2::types::ComponentDifferenceDto) -> Self {
        Self {
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            differences: v
                .differences
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            process_group_id: v.process_group_id,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentHistoryDto> for super::super::types::ComponentHistoryDto {
    fn from(v: crate::v2_7_2::types::ComponentHistoryDto) -> Self {
        Self {
            component_id: v.component_id,
            property_history: v
                .property_history
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentHistoryEntity>
    for super::super::types::ComponentHistoryEntity
{
    fn from(v: crate::v2_7_2::types::ComponentHistoryEntity) -> Self {
        Self {
            component_history: Some(v.component_history.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentManifest> for super::super::types::ComponentManifest {
    fn from(v: crate::v2_7_2::types::ComponentManifest) -> Self {
        Self {
            apis: v.apis.map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_registry_clients: v
                .flow_registry_clients
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_providers: v
                .parameter_providers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentReferenceDto>
    for super::super::types::ComponentReferenceDto
{
    fn from(v: crate::v2_7_2::types::ComponentReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentReferenceEntity>
    for super::super::types::ComponentReferenceEntity
{
    fn from(v: crate::v2_7_2::types::ComponentReferenceEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            parent_group_id: v.parent_group_id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentRestrictionPermissionDto>
    for super::super::types::ComponentRestrictionPermissionDto
{
    fn from(v: crate::v2_7_2::types::ComponentRestrictionPermissionDto) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentSearchResultDto>
    for super::super::types::ComponentSearchResultDto
{
    fn from(v: crate::v2_7_2::types::ComponentSearchResultDto) -> Self {
        Self {
            group_id: v.group_id,
            id: v.id,
            matches: v.matches,
            name: v.name,
            parent_group: v.parent_group.map(Into::into),
            versioned_group: v.versioned_group.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentStateDto> for super::super::types::ComponentStateDto {
    fn from(v: crate::v2_7_2::types::ComponentStateDto) -> Self {
        Self {
            cluster_state: v.cluster_state.map(Into::into),
            component_id: v.component_id,
            drop_state_key_supported: v.drop_state_key_supported,
            local_state: v.local_state.map(Into::into),
            state_description: v.state_description,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentStateEntity>
    for super::super::types::ComponentStateEntity
{
    fn from(v: crate::v2_7_2::types::ComponentStateEntity) -> Self {
        Self {
            component_state: Some(v.component_state.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentValidationResultDto>
    for super::super::types::ComponentValidationResultDto
{
    fn from(v: crate::v2_7_2::types::ComponentValidationResultDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            currently_valid: v.currently_valid,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| enum_to_string(&v)),
            resultant_validation_errors: v.resultant_validation_errors,
            results_valid: v.results_valid,
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentValidationResultEntity>
    for super::super::types::ComponentValidationResultEntity
{
    fn from(v: crate::v2_7_2::types::ComponentValidationResultEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentValidationResultsEntity>
    for super::super::types::ComponentValidationResultsEntity
{
    fn from(v: crate::v2_7_2::types::ComponentValidationResultsEntity) -> Self {
        Self {
            validation_results: v
                .validation_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ConfigVerificationResultDto>
    for super::super::types::ConfigVerificationResultDto
{
    fn from(v: crate::v2_7_2::types::ConfigVerificationResultDto) -> Self {
        Self {
            explanation: v.explanation,
            outcome: v.outcome.map(|v| enum_to_string(&v)),
            verification_step_name: v.verification_step_name,
        }
    }
}

impl From<crate::v2_7_2::types::ConfigurationAnalysisDto>
    for super::super::types::ConfigurationAnalysisDto
{
    fn from(v: crate::v2_7_2::types::ConfigurationAnalysisDto) -> Self {
        Self {
            component_id: v.component_id,
            properties: v.properties,
            referenced_attributes: v.referenced_attributes,
            supports_verification: v.supports_verification,
        }
    }
}

impl From<crate::v2_7_2::types::ConfigurationAnalysisEntity>
    for super::super::types::ConfigurationAnalysisEntity
{
    fn from(v: crate::v2_7_2::types::ConfigurationAnalysisEntity) -> Self {
        Self {
            configuration_analysis: Some(v.configuration_analysis.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectableComponent>
    for super::super::types::ConnectableComponent
{
    fn from(v: crate::v2_7_2::types::ConnectableComponent) -> Self {
        Self {
            comments: v.comments,
            group_id: v.group_id,
            id: v.id,
            instance_identifier: v.instance_identifier,
            name: v.name,
            r#type: v.r#type.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectableDto> for super::super::types::ConnectableDto {
    fn from(v: crate::v2_7_2::types::ConnectableDto) -> Self {
        Self {
            comments: v.comments,
            exists: v.exists,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            running: v.running,
            transmitting: v.transmitting,
            r#type: enum_to_string(&v.r#type),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionDto> for super::super::types::ConnectionDto {
    fn from(v: crate::v2_7_2::types::ConnectionDto) -> Self {
        Self {
            available_relationships: v.available_relationships,
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            destination: v.destination.map(Into::into),
            flow_file_expiration: v.flow_file_expiration,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            load_balance_compression: v.load_balance_compression,
            load_balance_partition_attribute: v.load_balance_partition_attribute,
            load_balance_status: v.load_balance_status,
            load_balance_strategy: v.load_balance_strategy,
            name: v.name,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            prioritizers: v.prioritizers,
            retried_relationships: v.retried_relationships,
            selected_relationships: v.selected_relationships,
            source: v.source.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionEntity> for super::super::types::ConnectionEntity {
    fn from(v: crate::v2_7_2::types::ConnectionEntity) -> Self {
        Self {
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            destination_group_id: v.destination_group_id,
            destination_id: v.destination_id,
            destination_type: enum_to_string(&v.destination_type),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            source_group_id: v.source_group_id,
            source_id: v.source_id,
            source_type: enum_to_string(&v.source_type),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatisticsDto>
    for super::super::types::ConnectionStatisticsDto
{
    fn from(v: crate::v2_7_2::types::ConnectionStatisticsDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            id: v.id,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            stats_last_refreshed: v.stats_last_refreshed,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatisticsEntity>
    for super::super::types::ConnectionStatisticsEntity
{
    fn from(v: crate::v2_7_2::types::ConnectionStatisticsEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_statistics: v.connection_statistics.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatisticsSnapshotDto>
    for super::super::types::ConnectionStatisticsSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ConnectionStatisticsSnapshotDto) -> Self {
        Self {
            id: v.id,
            predicted_bytes_at_next_interval: v.predicted_bytes_at_next_interval,
            predicted_count_at_next_interval: v.predicted_count_at_next_interval,
            predicted_millis_until_bytes_backpressure: v.predicted_millis_until_bytes_backpressure,
            predicted_millis_until_count_backpressure: v.predicted_millis_until_count_backpressure,
            predicted_percent_bytes: v.predicted_percent_bytes,
            predicted_percent_count: v.predicted_percent_count,
            prediction_interval_millis: v.prediction_interval_millis,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusDto> for super::super::types::ConnectionStatusDto {
    fn from(v: crate::v2_7_2::types::ConnectionStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            destination_id: v.destination_id,
            destination_name: v.destination_name,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            source_id: v.source_id,
            source_name: v.source_name,
            stats_last_refreshed: v.stats_last_refreshed,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusEntity>
    for super::super::types::ConnectionStatusEntity
{
    fn from(v: crate::v2_7_2::types::ConnectionStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status: v.connection_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusPredictionsSnapshotDto>
    for super::super::types::ConnectionStatusPredictionsSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ConnectionStatusPredictionsSnapshotDto) -> Self {
        Self {
            predicted_bytes_at_next_interval: v.predicted_bytes_at_next_interval,
            predicted_count_at_next_interval: v.predicted_count_at_next_interval,
            predicted_millis_until_bytes_backpressure: v.predicted_millis_until_bytes_backpressure,
            predicted_millis_until_count_backpressure: v.predicted_millis_until_count_backpressure,
            predicted_percent_bytes: v.predicted_percent_bytes,
            predicted_percent_count: v.predicted_percent_count,
            prediction_interval_seconds: v.prediction_interval_seconds,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusSnapshotDto>
    for super::super::types::ConnectionStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ConnectionStatusSnapshotDto) -> Self {
        Self {
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_queued: v.bytes_queued,
            destination_id: v.destination_id,
            destination_name: v.destination_name,
            flow_file_availability: v.flow_file_availability,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            flow_files_queued: v.flow_files_queued,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            load_balance_status: v.load_balance_status.map(|v| enum_to_string(&v)),
            name: v.name,
            output: v.output,
            percent_use_bytes: v.percent_use_bytes,
            percent_use_count: v.percent_use_count,
            predictions: v.predictions.map(Into::into),
            queued: v.queued,
            queued_count: v.queued_count,
            queued_size: v.queued_size,
            source_id: v.source_id,
            source_name: v.source_name,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusSnapshotEntity>
    for super::super::types::ConnectionStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::ConnectionStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status_snapshot: v.connection_status_snapshot.map(Into::into),
            id: v.id,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionsEntity> for super::super::types::ConnectionsEntity {
    fn from(v: crate::v2_7_2::types::ConnectionsEntity) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ContentViewerDto> for super::super::types::ContentViewerDto {
    fn from(v: crate::v2_7_2::types::ContentViewerDto) -> Self {
        Self {
            display_name: v.display_name,
            supported_mime_types: v
                .supported_mime_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ContentViewerEntity> for super::super::types::ContentViewerEntity {
    fn from(v: crate::v2_7_2::types::ContentViewerEntity) -> Self {
        Self {
            content_viewers: v
                .content_viewers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerBulletinsEntity>
    for super::super::types::ControllerBulletinsEntity
{
    fn from(v: crate::v2_7_2::types::ControllerBulletinsEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_service_bulletins: v
                .controller_service_bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_analysis_rule_bulletins: v
                .flow_analysis_rule_bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_registry_client_bulletins: v
                .flow_registry_client_bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_provider_bulletins: v
                .parameter_provider_bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reporting_task_bulletins: v
                .reporting_task_bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerConfigurationDto>
    for super::super::types::ControllerConfigurationDto
{
    fn from(v: crate::v2_7_2::types::ControllerConfigurationDto) -> Self {
        Self {
            max_timer_driven_thread_count: v.max_timer_driven_thread_count,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerConfigurationEntity>
    for super::super::types::ControllerConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::ControllerConfigurationEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            permissions: v.permissions.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerDto> for super::super::types::ControllerDto {
    fn from(v: crate::v2_7_2::types::ControllerDto) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            disabled_count: v.disabled_count,
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            instance_id: v.instance_id,
            invalid_count: v.invalid_count,
            name: v.name,
            output_port_count: v.output_port_count,
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_site_http_listening_port: v.remote_site_http_listening_port,
            remote_site_listening_port: v.remote_site_listening_port,
            running_count: v.running_count,
            site_to_site_secure: v.site_to_site_secure,
            stopped_count: v.stopped_count,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerEntity> for super::super::types::ControllerEntity {
    fn from(v: crate::v2_7_2::types::ControllerEntity) -> Self {
        Self {
            controller: Some(v.controller.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceAPI>
    for super::super::types::ControllerServiceAPI
{
    fn from(v: crate::v2_7_2::types::ControllerServiceAPI) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceApiDto>
    for super::super::types::ControllerServiceApiDto
{
    fn from(v: crate::v2_7_2::types::ControllerServiceApiDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceDefinition>
    for super::super::types::ControllerServiceDefinition
{
    fn from(v: crate::v2_7_2::types::ControllerServiceDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(Into::into),
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceDto>
    for super::super::types::ControllerServiceDto
{
    fn from(v: crate::v2_7_2::types::ControllerServiceDto) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            controller_service_apis: v
                .controller_service_apis
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            custom_ui_url: v.custom_ui_url,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(Into::into),
            properties: v.properties,
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v.state.map(|v| enum_to_string(&v)),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceEntity>
    for super::super::types::ControllerServiceEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            parent_group_id: v.parent_group_id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceReferencingComponentDto>
    for super::super::types::ControllerServiceReferencingComponentDto
{
    fn from(v: crate::v2_7_2::types::ControllerServiceReferencingComponentDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            properties: v.properties,
            reference_cycle: v.reference_cycle,
            reference_type: v.reference_type.map(|v| enum_to_string(&v)),
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            state: v.state,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceReferencingComponentEntity>
    for super::super::types::ControllerServiceReferencingComponentEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceReferencingComponentEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceReferencingComponentsEntity>
    for super::super::types::ControllerServiceReferencingComponentsEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceReferencingComponentsEntity) -> Self {
        Self {
            controller_service_referencing_components: v
                .controller_service_referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceRunStatusEntity>
    for super::super::types::ControllerServiceRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceStatusDto>
    for super::super::types::ControllerServiceStatusDto
{
    fn from(v: crate::v2_7_2::types::ControllerServiceStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceTypesEntity>
    for super::super::types::ControllerServiceTypesEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceTypesEntity) -> Self {
        Self {
            controller_service_types: v
                .controller_service_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServicesEntity>
    for super::super::types::ControllerServicesEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServicesEntity) -> Self {
        Self {
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            current_time: v.current_time,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerStatusDto> for super::super::types::ControllerStatusDto {
    fn from(v: crate::v2_7_2::types::ControllerStatusDto) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            active_thread_count: v.active_thread_count,
            bytes_queued: v.bytes_queued,
            disabled_count: v.disabled_count,
            flow_files_queued: v.flow_files_queued,
            inactive_remote_port_count: v.inactive_remote_port_count,
            invalid_count: v.invalid_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            queued: v.queued,
            running_count: v.running_count,
            stale_count: v.stale_count,
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            terminated_thread_count: v.terminated_thread_count,
            up_to_date_count: v.up_to_date_count,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerStatusEntity>
    for super::super::types::ControllerStatusEntity
{
    fn from(v: crate::v2_7_2::types::ControllerStatusEntity) -> Self {
        Self {
            controller_status: Some(v.controller_status.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::CopyRequestEntity> for super::super::types::CopyRequestEntity {
    fn from(v: crate::v2_7_2::types::CopyRequestEntity) -> Self {
        Self {
            connections: v.connections,
            funnels: v.funnels,
            input_ports: v.input_ports,
            labels: v.labels,
            output_ports: v.output_ports,
            process_groups: v.process_groups,
            processors: v.processors,
            remote_process_groups: v.remote_process_groups,
        }
    }
}

impl From<crate::v2_7_2::types::CopyResponseEntity> for super::super::types::CopyResponseEntity {
    fn from(v: crate::v2_7_2::types::CopyResponseEntity) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            external_controller_service_references: v
                .external_controller_service_references
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            id: v.id,
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_contexts: v
                .parameter_contexts
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            parameter_providers: v
                .parameter_providers
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::CopySnippetRequestEntity>
    for super::super::types::CopySnippetRequestEntity
{
    fn from(v: crate::v2_7_2::types::CopySnippetRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            origin_x: v.origin_x,
            origin_y: v.origin_y,
            snippet_id: v.snippet_id,
        }
    }
}

impl From<crate::v2_7_2::types::CounterDto> for super::super::types::CounterDto {
    fn from(v: crate::v2_7_2::types::CounterDto) -> Self {
        Self {
            context: v.context,
            id: v.id,
            name: v.name,
            value: v.value,
            value_count: v.value_count,
        }
    }
}

impl From<crate::v2_7_2::types::CounterEntity> for super::super::types::CounterEntity {
    fn from(v: crate::v2_7_2::types::CounterEntity) -> Self {
        Self {
            counter: Some(v.counter.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersDto> for super::super::types::CountersDto {
    fn from(v: crate::v2_7_2::types::CountersDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersEntity> for super::super::types::CountersEntity {
    fn from(v: crate::v2_7_2::types::CountersEntity) -> Self {
        Self {
            counters: Some(v.counters.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersSnapshotDto> for super::super::types::CountersSnapshotDto {
    fn from(v: crate::v2_7_2::types::CountersSnapshotDto) -> Self {
        Self {
            counters: v
                .counters
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
        }
    }
}

impl From<crate::v2_7_2::types::CreateActiveRequestEntity>
    for super::super::types::CreateActiveRequestEntity
{
    fn from(v: crate::v2_7_2::types::CreateActiveRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_id: v.process_group_id,
        }
    }
}

impl From<crate::v2_7_2::types::CurrentUserEntity> for super::super::types::CurrentUserEntity {
    fn from(v: crate::v2_7_2::types::CurrentUserEntity) -> Self {
        Self {
            anonymous: v.anonymous,
            can_version_flows: v.can_version_flows,
            component_restriction_permissions: v
                .component_restriction_permissions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_permissions: v.controller_permissions.map(Into::into),
            counters_permissions: v.counters_permissions.map(Into::into),
            identity: v.identity,
            logout_supported: v.logout_supported,
            parameter_context_permissions: v.parameter_context_permissions.map(Into::into),
            policies_permissions: v.policies_permissions.map(Into::into),
            provenance_permissions: v.provenance_permissions.map(Into::into),
            restricted_components_permissions: v.restricted_components_permissions.map(Into::into),
            system_permissions: v.system_permissions.map(Into::into),
            tenants_permissions: v.tenants_permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::DateTimeParameter> for super::super::types::DateTimeParameter {
    fn from(v: crate::v2_7_2::types::DateTimeParameter) -> Self {
        Self {
            date_time: v.date_time,
        }
    }
}

impl From<crate::v2_7_2::types::DefinedType> for super::super::types::DefinedType {
    fn from(v: crate::v2_7_2::types::DefinedType) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::DiagnosticLevel> for super::super::types::DiagnosticLevel {
    fn from(v: crate::v2_7_2::types::DiagnosticLevel) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::DifferenceDto> for super::super::types::DifferenceDto {
    fn from(v: crate::v2_7_2::types::DifferenceDto) -> Self {
        Self {
            difference: v.difference,
            difference_type: v.difference_type,
        }
    }
}

impl From<crate::v2_7_2::types::DimensionsDto> for super::super::types::DimensionsDto {
    fn from(v: crate::v2_7_2::types::DimensionsDto) -> Self {
        Self {
            height: v.height,
            width: v.width,
        }
    }
}

impl From<crate::v2_7_2::types::DocumentedTypeDto> for super::super::types::DocumentedTypeDto {
    fn from(v: crate::v2_7_2::types::DocumentedTypeDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            controller_service_apis: v
                .controller_service_apis
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            deprecation_reason: v.deprecation_reason,
            description: v.description,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            tags: v.tags,
            r#type: v.r#type,
            usage_restriction: v.usage_restriction,
        }
    }
}

impl From<crate::v2_7_2::types::DropRequestDto> for super::super::types::DropRequestDto {
    fn from(v: crate::v2_7_2::types::DropRequestDto) -> Self {
        Self {
            current: v.current,
            current_count: v.current_count,
            current_size: v.current_size,
            dropped: v.dropped,
            dropped_count: v.dropped_count,
            dropped_size: v.dropped_size,
            failure_reason: v.failure_reason,
            finished: v.finished,
            id: v.id,
            last_updated: v.last_updated,
            original: v.original,
            original_count: v.original_count,
            original_size: v.original_size,
            percent_completed: v.percent_completed,
            state: v.state,
            submission_time: v.submission_time,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::DropRequestEntity> for super::super::types::DropRequestEntity {
    fn from(v: crate::v2_7_2::types::DropRequestEntity) -> Self {
        Self {
            drop_request: Some(v.drop_request.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::DynamicProperty> for super::super::types::DynamicProperty {
    fn from(v: crate::v2_7_2::types::DynamicProperty) -> Self {
        Self {
            description: v.description,
            expression_language_scope: v.expression_language_scope.map(|v| enum_to_string(&v)),
            name: v.name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::DynamicRelationship> for super::super::types::DynamicRelationship {
    fn from(v: crate::v2_7_2::types::DynamicRelationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ExplicitRestrictionDto>
    for super::super::types::ExplicitRestrictionDto
{
    fn from(v: crate::v2_7_2::types::ExplicitRestrictionDto) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ExternalControllerServiceReference>
    for super::super::types::ExternalControllerServiceReference
{
    fn from(v: crate::v2_7_2::types::ExternalControllerServiceReference) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisResultEntity>
    for super::super::types::FlowAnalysisResultEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisResultEntity) -> Self {
        Self {
            flow_analysis_pending: v.flow_analysis_pending,
            rule_violations: v
                .rule_violations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            rules: v.rules.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleDefinition>
    for super::super::types::FlowAnalysisRuleDefinition
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(Into::into),
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleDto> for super::super::types::FlowAnalysisRuleDto {
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            enforcement_policy: v.enforcement_policy,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(Into::into),
            properties: v.properties,
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v.state.map(|v| enum_to_string(&v)),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleEntity>
    for super::super::types::FlowAnalysisRuleEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity>
    for super::super::types::FlowAnalysisRuleRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleStatusDto>
    for super::super::types::FlowAnalysisRuleStatusDto
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleTypesEntity>
    for super::super::types::FlowAnalysisRuleTypesEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleTypesEntity) -> Self {
        Self {
            flow_analysis_rule_types: v
                .flow_analysis_rule_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleViolationDto>
    for super::super::types::FlowAnalysisRuleViolationDto
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleViolationDto) -> Self {
        Self {
            enabled: v.enabled,
            enforcement_policy: v.enforcement_policy,
            group_id: v.group_id,
            issue_id: v.issue_id,
            rule_id: v.rule_id,
            scope: v.scope,
            subject_component_type: v.subject_component_type,
            subject_display_name: v.subject_display_name,
            subject_id: v.subject_id,
            subject_permission_dto: v.subject_permission_dto.map(Into::into),
            violation_message: v.violation_message,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRulesEntity>
    for super::super::types::FlowAnalysisRulesEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRulesEntity) -> Self {
        Self {
            current_time: v.current_time,
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowBreadcrumbDto> for super::super::types::FlowBreadcrumbDto {
    fn from(v: crate::v2_7_2::types::FlowBreadcrumbDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::FlowBreadcrumbEntity>
    for super::super::types::FlowBreadcrumbEntity
{
    fn from(v: crate::v2_7_2::types::FlowBreadcrumbEntity) -> Self {
        Self {
            breadcrumb: v.breadcrumb.map(Into::into),
            id: v.id,
            parent_breadcrumb: v.parent_breadcrumb.map(|v| Box::new((*v).into())),
            permissions: v.permissions.map(Into::into),
            versioned_flow_state: v.versioned_flow_state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::FlowComparisonEntity>
    for super::super::types::FlowComparisonEntity
{
    fn from(v: crate::v2_7_2::types::FlowComparisonEntity) -> Self {
        Self {
            component_differences: v
                .component_differences
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowConfigurationDto>
    for super::super::types::FlowConfigurationDto
{
    fn from(v: crate::v2_7_2::types::FlowConfigurationDto) -> Self {
        Self {
            current_time: v.current_time,
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            supports_configurable_authorizer: v.supports_configurable_authorizer,
            supports_configurable_users_and_groups: v.supports_configurable_users_and_groups,
            supports_managed_authorizer: v.supports_managed_authorizer,
            time_offset: v.time_offset,
        }
    }
}

impl From<crate::v2_7_2::types::FlowConfigurationEntity>
    for super::super::types::FlowConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::FlowConfigurationEntity) -> Self {
        Self {
            flow_configuration: Some(v.flow_configuration.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowDto> for super::super::types::FlowDto {
    fn from(v: crate::v2_7_2::types::FlowDto) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowEntity> for super::super::types::FlowEntity {
    fn from(v: crate::v2_7_2::types::FlowEntity) -> Self {
        Self {
            flow: Some(v.flow.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowFileDto> for super::super::types::FlowFileDto {
    fn from(v: crate::v2_7_2::types::FlowFileDto) -> Self {
        Self {
            attributes: v.attributes,
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            content_claim_container: v.content_claim_container,
            content_claim_file_size: v.content_claim_file_size,
            content_claim_file_size_bytes: v.content_claim_file_size_bytes,
            content_claim_identifier: v.content_claim_identifier,
            content_claim_offset: v.content_claim_offset,
            content_claim_section: v.content_claim_section,
            filename: v.filename,
            lineage_duration: v.lineage_duration,
            mime_type: v.mime_type,
            penalized: v.penalized,
            penalty_expires_in: v.penalty_expires_in,
            position: v.position,
            queued_duration: v.queued_duration,
            size: v.size,
            uri: v.uri,
            uuid: v.uuid,
        }
    }
}

impl From<crate::v2_7_2::types::FlowFileEntity> for super::super::types::FlowFileEntity {
    fn from(v: crate::v2_7_2::types::FlowFileEntity) -> Self {
        Self {
            flow_file: Some(v.flow_file.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowFileSummaryDto> for super::super::types::FlowFileSummaryDto {
    fn from(v: crate::v2_7_2::types::FlowFileSummaryDto) -> Self {
        Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            filename: v.filename,
            lineage_duration: v.lineage_duration,
            mime_type: v.mime_type,
            penalized: v.penalized,
            penalty_expires_in: v.penalty_expires_in,
            position: v.position,
            queued_duration: v.queued_duration,
            size: v.size,
            uri: v.uri,
            uuid: v.uuid,
        }
    }
}

impl From<crate::v2_7_2::types::FlowMetricsReportingStrategy>
    for super::super::types::FlowMetricsReportingStrategy
{
    fn from(v: crate::v2_7_2::types::FlowMetricsReportingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchDto>
    for super::super::types::FlowRegistryBranchDto
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchDto) -> Self {
        Self { name: v.name }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchEntity>
    for super::super::types::FlowRegistryBranchEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchEntity) -> Self {
        Self {
            branch: Some(v.branch.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchesEntity>
    for super::super::types::FlowRegistryBranchesEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchesEntity) -> Self {
        Self {
            branches: v
                .branches
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBucket> for super::super::types::FlowRegistryBucket {
    fn from(v: crate::v2_7_2::types::FlowRegistryBucket) -> Self {
        Self {
            created_timestamp: v.created_timestamp,
            description: v.description,
            identifier: v.identifier,
            name: v.name,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBucketDto>
    for super::super::types::FlowRegistryBucketDto
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBucketDto) -> Self {
        Self {
            created: v.created,
            description: v.description,
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBucketEntity>
    for super::super::types::FlowRegistryBucketEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBucketEntity) -> Self {
        Self {
            bucket: v.bucket.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBucketsEntity>
    for super::super::types::FlowRegistryBucketsEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBucketsEntity) -> Self {
        Self {
            buckets: v.buckets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientDefinition>
    for super::super::types::FlowRegistryClientDefinition
{
    fn from(v: crate::v2_7_2::types::FlowRegistryClientDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(Into::into),
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientDto>
    for super::super::types::FlowRegistryClientDto
{
    fn from(v: crate::v2_7_2::types::FlowRegistryClientDto) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            deprecated: v.deprecated,
            description: v.description,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            properties: v.properties,
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            supports_branching: v.supports_branching,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientEntity>
    for super::super::types::FlowRegistryClientEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryClientEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientTypesEntity>
    for super::super::types::FlowRegistryClientTypesEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryClientTypesEntity) -> Self {
        Self {
            flow_registry_client_types: v
                .flow_registry_client_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientsEntity>
    for super::super::types::FlowRegistryClientsEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryClientsEntity) -> Self {
        Self {
            current_time: v.current_time,
            registries: v
                .registries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryPermissions>
    for super::super::types::FlowRegistryPermissions
{
    fn from(v: crate::v2_7_2::types::FlowRegistryPermissions) -> Self {
        Self {
            can_delete: v.can_delete,
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_7_2::types::FlowSnippetDto> for super::super::types::FlowSnippetDto {
    fn from(v: crate::v2_7_2::types::FlowSnippetDto) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FunnelDto> for super::super::types::FunnelDto {
    fn from(v: crate::v2_7_2::types::FunnelDto) -> Self {
        Self {
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::FunnelEntity> for super::super::types::FunnelEntity {
    fn from(v: crate::v2_7_2::types::FunnelEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::FunnelsEntity> for super::super::types::FunnelsEntity {
    fn from(v: crate::v2_7_2::types::FunnelsEntity) -> Self {
        Self {
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::GarbageCollectionDto>
    for super::super::types::GarbageCollectionDto
{
    fn from(v: crate::v2_7_2::types::GarbageCollectionDto) -> Self {
        Self {
            collection_count: v.collection_count,
            collection_millis: v.collection_millis,
            collection_time: v.collection_time,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::HistoryDto> for super::super::types::HistoryDto {
    fn from(v: crate::v2_7_2::types::HistoryDto) -> Self {
        Self {
            actions: v.actions.map(|v| v.into_iter().map(|v| v.into()).collect()),
            last_refreshed: v.last_refreshed,
            total: v.total,
        }
    }
}

impl From<crate::v2_7_2::types::HistoryEntity> for super::super::types::HistoryEntity {
    fn from(v: crate::v2_7_2::types::HistoryEntity) -> Self {
        Self {
            history: Some(v.history.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::IncludedRegistries> for super::super::types::IncludedRegistries {
    fn from(v: crate::v2_7_2::types::IncludedRegistries) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::InputPortsEntity> for super::super::types::InputPortsEntity {
    fn from(v: crate::v2_7_2::types::InputPortsEntity) -> Self {
        Self {
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::IntegerParameter> for super::super::types::IntegerParameter {
    fn from(v: crate::v2_7_2::types::IntegerParameter) -> Self {
        Self { integer: v.integer }
    }
}

impl From<crate::v2_7_2::types::JmxMetricsResultDto> for super::super::types::JmxMetricsResultDto {
    fn from(v: crate::v2_7_2::types::JmxMetricsResultDto) -> Self {
        Self {
            attribute_name: v.attribute_name,
            attribute_value: v.attribute_value,
            bean_name: v.bean_name,
        }
    }
}

impl From<crate::v2_7_2::types::JmxMetricsResultsEntity>
    for super::super::types::JmxMetricsResultsEntity
{
    fn from(v: crate::v2_7_2::types::JmxMetricsResultsEntity) -> Self {
        Self {
            jmx_metrics_results: v
                .jmx_metrics_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::LabelDto> for super::super::types::LabelDto {
    fn from(v: crate::v2_7_2::types::LabelDto) -> Self {
        Self {
            getz_index: v.getz_index,
            height: v.height,
            id: v.id,
            label: v.label,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            style: v.style,
            versioned_component_id: v.versioned_component_id,
            width: v.width,
        }
    }
}

impl From<crate::v2_7_2::types::LabelEntity> for super::super::types::LabelEntity {
    fn from(v: crate::v2_7_2::types::LabelEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            dimensions: v.dimensions.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::LabelsEntity> for super::super::types::LabelsEntity {
    fn from(v: crate::v2_7_2::types::LabelsEntity) -> Self {
        Self {
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::LatestProvenanceEventsDto>
    for super::super::types::LatestProvenanceEventsDto
{
    fn from(v: crate::v2_7_2::types::LatestProvenanceEventsDto) -> Self {
        Self {
            component_id: v.component_id,
            provenance_events: v
                .provenance_events
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::LatestProvenanceEventsEntity>
    for super::super::types::LatestProvenanceEventsEntity
{
    fn from(v: crate::v2_7_2::types::LatestProvenanceEventsEntity) -> Self {
        Self {
            latest_provenance_events: Some(v.latest_provenance_events.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::LineageDto> for super::super::types::LineageDto {
    fn from(v: crate::v2_7_2::types::LineageDto) -> Self {
        Self {
            expiration: v.expiration,
            finished: v.finished,
            id: v.id,
            percent_completed: v.percent_completed,
            request: v.request.map(Into::into),
            results: v.results.map(Into::into),
            submission_time: v.submission_time,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::LineageEntity> for super::super::types::LineageEntity {
    fn from(v: crate::v2_7_2::types::LineageEntity) -> Self {
        Self {
            lineage: Some(v.lineage.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::LineageRequestDto> for super::super::types::LineageRequestDto {
    fn from(v: crate::v2_7_2::types::LineageRequestDto) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
            lineage_request_type: v.lineage_request_type.map(|v| enum_to_string(&v)),
            uuid: v.uuid,
        }
    }
}

impl From<crate::v2_7_2::types::LineageResultsDto> for super::super::types::LineageResultsDto {
    fn from(v: crate::v2_7_2::types::LineageResultsDto) -> Self {
        Self {
            errors: v.errors,
            links: v.links.map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ListenPortDto> for super::super::types::ListenPortDto {
    fn from(v: crate::v2_7_2::types::ListenPortDto) -> Self {
        Self {
            application_protocols: v.application_protocols,
            component_class: v.component_class,
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            parent_group_id: v.parent_group_id,
            parent_group_name: v.parent_group_name,
            port_name: v.port_name,
            port_number: v.port_number,
            transport_protocol: v.transport_protocol,
        }
    }
}

impl From<crate::v2_7_2::types::ListenPortsEntity> for super::super::types::ListenPortsEntity {
    fn from(v: crate::v2_7_2::types::ListenPortsEntity) -> Self {
        Self {
            listen_ports: v
                .listen_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ListingRequestDto> for super::super::types::ListingRequestDto {
    fn from(v: crate::v2_7_2::types::ListingRequestDto) -> Self {
        Self {
            destination_running: v.destination_running,
            failure_reason: v.failure_reason,
            finished: v.finished,
            flow_file_summaries: v
                .flow_file_summaries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            id: v.id,
            last_updated: v.last_updated,
            max_results: v.max_results,
            percent_completed: v.percent_completed,
            queue_size: v.queue_size.map(Into::into),
            source_running: v.source_running,
            state: v.state,
            submission_time: v.submission_time,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ListingRequestEntity>
    for super::super::types::ListingRequestEntity
{
    fn from(v: crate::v2_7_2::types::ListingRequestEntity) -> Self {
        Self {
            listing_request: Some(v.listing_request.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::LongParameter> for super::super::types::LongParameter {
    fn from(v: crate::v2_7_2::types::LongParameter) -> Self {
        Self { long: v.long }
    }
}

impl From<crate::v2_7_2::types::MultiProcessorUseCase>
    for super::super::types::MultiProcessorUseCase
{
    fn from(v: crate::v2_7_2::types::MultiProcessorUseCase) -> Self {
        Self {
            configurations: v
                .configurations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            description: v.description,
            keywords: v.keywords,
            notes: v.notes,
        }
    }
}

impl From<crate::v2_7_2::types::NarCoordinateDto> for super::super::types::NarCoordinateDto {
    fn from(v: crate::v2_7_2::types::NarCoordinateDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::NarDetailsEntity> for super::super::types::NarDetailsEntity {
    fn from(v: crate::v2_7_2::types::NarDetailsEntity) -> Self {
        Self {
            controller_service_types: v
                .controller_service_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            dependent_coordinates: v
                .dependent_coordinates
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_analysis_rule_types: v
                .flow_analysis_rule_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_registry_client_types: v
                .flow_registry_client_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            nar_summary: v.nar_summary.map(Into::into),
            parameter_provider_types: v
                .parameter_provider_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processor_types: v
                .processor_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reporting_task_types: v
                .reporting_task_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::NarSummariesEntity> for super::super::types::NarSummariesEntity {
    fn from(v: crate::v2_7_2::types::NarSummariesEntity) -> Self {
        Self {
            current_time: v.current_time,
            nar_summaries: v
                .nar_summaries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::NarSummaryDto> for super::super::types::NarSummaryDto {
    fn from(v: crate::v2_7_2::types::NarSummaryDto) -> Self {
        Self {
            build_time: v.build_time,
            coordinate: v.coordinate.map(Into::into),
            created_by: v.created_by,
            dependency_coordinate: v.dependency_coordinate.map(Into::into),
            digest: v.digest,
            extension_count: v.extension_count,
            failure_message: v.failure_message,
            identifier: v.identifier,
            install_complete: v.install_complete,
            source_identifier: v.source_identifier,
            source_type: v.source_type,
            state: v.state,
        }
    }
}

impl From<crate::v2_7_2::types::NarSummaryEntity> for super::super::types::NarSummaryEntity {
    fn from(v: crate::v2_7_2::types::NarSummaryEntity) -> Self {
        Self {
            nar_summary: Some(v.nar_summary.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::NodeConnectionStatisticsSnapshotDto>
    for super::super::types::NodeConnectionStatisticsSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeConnectionStatisticsSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            statistics_snapshot: v.statistics_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeConnectionStatusSnapshotDto>
    for super::super::types::NodeConnectionStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeConnectionStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeCountersSnapshotDto>
    for super::super::types::NodeCountersSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeCountersSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeDto> for super::super::types::NodeDto {
    fn from(v: crate::v2_7_2::types::NodeDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            address: v.address,
            api_port: v.api_port,
            bytes_queued: v.bytes_queued,
            connection_requested: v.connection_requested,
            events: v.events.map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_file_bytes: v.flow_file_bytes,
            flow_files_queued: v.flow_files_queued,
            heartbeat: v.heartbeat,
            node_id: v.node_id,
            node_start_time: v.node_start_time,
            queued: v.queued,
            roles: v.roles,
            status: v.status,
        }
    }
}

impl From<crate::v2_7_2::types::NodeEntity> for super::super::types::NodeEntity {
    fn from(v: crate::v2_7_2::types::NodeEntity) -> Self {
        Self {
            node: Some(v.node.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::NodeEventDto> for super::super::types::NodeEventDto {
    fn from(v: crate::v2_7_2::types::NodeEventDto) -> Self {
        Self {
            category: v.category,
            message: v.message,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::NodePortStatusSnapshotDto>
    for super::super::types::NodePortStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodePortStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeProcessGroupStatusSnapshotDto>
    for super::super::types::NodeProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeProcessGroupStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeProcessorStatusSnapshotDto>
    for super::super::types::NodeProcessorStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeProcessorStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeRemoteProcessGroupStatusSnapshotDto>
    for super::super::types::NodeRemoteProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeRemoteProcessGroupStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeReplayLastEventSnapshotDto>
    for super::super::types::NodeReplayLastEventSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeReplayLastEventSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeSearchResultDto> for super::super::types::NodeSearchResultDto {
    fn from(v: crate::v2_7_2::types::NodeSearchResultDto) -> Self {
        Self {
            address: v.address,
            id: v.id,
        }
    }
}

impl From<crate::v2_7_2::types::NodeStatusSnapshotsDto>
    for super::super::types::NodeStatusSnapshotsDto
{
    fn from(v: crate::v2_7_2::types::NodeStatusSnapshotsDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshots: v
                .status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::NodeSystemDiagnosticsSnapshotDto>
    for super::super::types::NodeSystemDiagnosticsSnapshotDto
{
    fn from(v: crate::v2_7_2::types::NodeSystemDiagnosticsSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::OutputPortsEntity> for super::super::types::OutputPortsEntity {
    fn from(v: crate::v2_7_2::types::OutputPortsEntity) -> Self {
        Self {
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextDto> for super::super::types::ParameterContextDto {
    fn from(v: crate::v2_7_2::types::ParameterContextDto) -> Self {
        Self {
            bound_process_groups: v
                .bound_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            description: v.description,
            id: v.id,
            inherited_parameter_contexts: v
                .inherited_parameter_contexts
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            name: v.name,
            parameter_provider_configuration: v.parameter_provider_configuration.map(Into::into),
            parameters: v
                .parameters
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextEntity>
    for super::super::types::ParameterContextEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextHandlingStrategy>
    for super::super::types::ParameterContextHandlingStrategy
{
    fn from(v: crate::v2_7_2::types::ParameterContextHandlingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::ParameterContextReferenceDto>
    for super::super::types::ParameterContextReferenceDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextReferenceEntity>
    for super::super::types::ParameterContextReferenceEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextReferenceEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextUpdateEntity>
    for super::super::types::ParameterContextUpdateEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextUpdateEntity) -> Self {
        Self {
            parameter_context: v.parameter_context.map(Into::into),
            parameter_context_revision: v.parameter_context_revision.map(Into::into),
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextUpdateRequestDto>
    for super::super::types::ParameterContextUpdateRequestDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextUpdateRequestDto) -> Self {
        Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context: v.parameter_context.map(Into::into),
            percent_completed: v.percent_completed,
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextUpdateRequestEntity>
    for super::super::types::ParameterContextUpdateRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextUpdateRequestEntity) -> Self {
        Self {
            parameter_context_revision: v.parameter_context_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextUpdateStepDto>
    for super::super::types::ParameterContextUpdateStepDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextValidationRequestDto>
    for super::super::types::ParameterContextValidationRequestDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextValidationRequestDto) -> Self {
        Self {
            complete: v.complete,
            component_validation_results: v.component_validation_results.map(Into::into),
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context: v.parameter_context.map(Into::into),
            percent_completed: v.percent_completed,
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextValidationRequestEntity>
    for super::super::types::ParameterContextValidationRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextValidationRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextValidationStepDto>
    for super::super::types::ParameterContextValidationStepDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextValidationStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextsEntity>
    for super::super::types::ParameterContextsEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextsEntity) -> Self {
        Self {
            current_time: v.current_time,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterDto> for super::super::types::ParameterDto {
    fn from(v: crate::v2_7_2::types::ParameterDto) -> Self {
        Self {
            description: v.description,
            inherited: v.inherited,
            name: v.name,
            parameter_context: v.parameter_context.map(Into::into),
            provided: v.provided,
            referenced_assets: v
                .referenced_assets
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            sensitive: v.sensitive,
            value: v.value,
            value_removed: v.value_removed,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterEntity> for super::super::types::ParameterEntity {
    fn from(v: crate::v2_7_2::types::ParameterEntity) -> Self {
        Self {
            can_write: v.can_write,
            parameter: v.parameter.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterGroupConfigurationEntity>
    for super::super::types::ParameterGroupConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::ParameterGroupConfigurationEntity) -> Self {
        Self {
            group_name: v.group_name,
            parameter_context_name: v.parameter_context_name,
            parameter_sensitivities: v.parameter_sensitivities.map(|m| {
                m.into_iter()
                    .map(|(k, v)| (k, v.map(|v| enum_to_string(&v))))
                    .collect()
            }),
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderApplyParametersRequestDto>
    for super::super::types::ParameterProviderApplyParametersRequestDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderApplyParametersRequestDto) -> Self {
        Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context_updates: v
                .parameter_context_updates
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_provider: v.parameter_provider.map(Into::into),
            percent_completed: v.percent_completed,
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderApplyParametersRequestEntity>
    for super::super::types::ParameterProviderApplyParametersRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderApplyParametersRequestEntity) -> Self {
        Self {
            request: Some(v.request.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderApplyParametersUpdateStepDto>
    for super::super::types::ParameterProviderApplyParametersUpdateStepDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderApplyParametersUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderConfigurationDto>
    for super::super::types::ParameterProviderConfigurationDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderConfigurationDto) -> Self {
        Self {
            parameter_group_name: v.parameter_group_name,
            parameter_provider_id: v.parameter_provider_id,
            parameter_provider_name: v.parameter_provider_name,
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderConfigurationEntity>
    for super::super::types::ParameterProviderConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderConfigurationEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderDefinition>
    for super::super::types::ParameterProviderDefinition
{
    fn from(v: crate::v2_7_2::types::ParameterProviderDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(Into::into),
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderDto>
    for super::super::types::ParameterProviderDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderDto) -> Self {
        Self {
            affected_components: v
                .affected_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            custom_ui_url: v.custom_ui_url,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parameter_group_configurations: v
                .parameter_group_configurations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_status: v
                .parameter_status
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(Into::into),
            properties: v.properties,
            referencing_parameter_contexts: v
                .referencing_parameter_contexts
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderEntity>
    for super::super::types::ParameterProviderEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderParameterApplicationEntity>
    for super::super::types::ParameterProviderParameterApplicationEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderParameterApplicationEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            parameter_group_configurations: v
                .parameter_group_configurations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderParameterFetchEntity>
    for super::super::types::ParameterProviderParameterFetchEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderParameterFetchEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderReference>
    for super::super::types::ParameterProviderReference
{
    fn from(v: crate::v2_7_2::types::ParameterProviderReference) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            identifier: v.identifier,
            name: v.name,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderReferencingComponentDto>
    for super::super::types::ParameterProviderReferencingComponentDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderReferencingComponentDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderReferencingComponentEntity>
    for super::super::types::ParameterProviderReferencingComponentEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderReferencingComponentEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderReferencingComponentsEntity>
    for super::super::types::ParameterProviderReferencingComponentsEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderReferencingComponentsEntity) -> Self {
        Self {
            parameter_provider_referencing_components: v
                .parameter_provider_referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderTypesEntity>
    for super::super::types::ParameterProviderTypesEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderTypesEntity) -> Self {
        Self {
            parameter_provider_types: v
                .parameter_provider_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProvidersEntity>
    for super::super::types::ParameterProvidersEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProvidersEntity) -> Self {
        Self {
            current_time: v.current_time,
            parameter_providers: v
                .parameter_providers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterStatusDto> for super::super::types::ParameterStatusDto {
    fn from(v: crate::v2_7_2::types::ParameterStatusDto) -> Self {
        Self {
            parameter: v.parameter.map(Into::into),
            status: v.status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::PasteRequestEntity> for super::super::types::PasteRequestEntity {
    fn from(v: crate::v2_7_2::types::PasteRequestEntity) -> Self {
        Self {
            copy_response: v.copy_response.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PasteResponseEntity> for super::super::types::PasteResponseEntity {
    fn from(v: crate::v2_7_2::types::PasteResponseEntity) -> Self {
        Self {
            flow: v.flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PeerDto> for super::super::types::PeerDto {
    fn from(v: crate::v2_7_2::types::PeerDto) -> Self {
        Self {
            flow_file_count: v.flow_file_count,
            hostname: v.hostname,
            port: v.port,
            secure: v.secure,
        }
    }
}

impl From<crate::v2_7_2::types::PeersEntity> for super::super::types::PeersEntity {
    fn from(v: crate::v2_7_2::types::PeersEntity) -> Self {
        Self {
            peers: v.peers.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PermissionsDto> for super::super::types::PermissionsDto {
    fn from(v: crate::v2_7_2::types::PermissionsDto) -> Self {
        Self {
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_7_2::types::PortDto> for super::super::types::PortDto {
    fn from(v: crate::v2_7_2::types::PortDto) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            port_function: v.port_function.map(|v| enum_to_string(&v)),
            position: v.position.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
            transmitting: v.transmitting,
            r#type: v.r#type.map(|v| enum_to_string(&v)),
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::PortEntity> for super::super::types::PortEntity {
    fn from(v: crate::v2_7_2::types::PortEntity) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            port_type: v.port_type,
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::PortRunStatusEntity> for super::super::types::PortRunStatusEntity {
    fn from(v: crate::v2_7_2::types::PortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusDto> for super::super::types::PortStatusDto {
    fn from(v: crate::v2_7_2::types::PortStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            stats_last_refreshed: v.stats_last_refreshed,
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusEntity> for super::super::types::PortStatusEntity {
    fn from(v: crate::v2_7_2::types::PortStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            port_status: v.port_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusSnapshotDto>
    for super::super::types::PortStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::PortStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusSnapshotEntity>
    for super::super::types::PortStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::PortStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            port_status_snapshot: v.port_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::Position> for super::super::types::Position {
    fn from(v: crate::v2_7_2::types::Position) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_7_2::types::PositionDto> for super::super::types::PositionDto {
    fn from(v: crate::v2_7_2::types::PositionDto) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_7_2::types::PreviousValueDto> for super::super::types::PreviousValueDto {
    fn from(v: crate::v2_7_2::types::PreviousValueDto) -> Self {
        Self {
            previous_value: v.previous_value,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        }
    }
}

impl From<crate::v2_7_2::types::PrioritizerTypesEntity>
    for super::super::types::PrioritizerTypesEntity
{
    fn from(v: crate::v2_7_2::types::PrioritizerTypesEntity) -> Self {
        Self {
            prioritizer_types: v
                .prioritizer_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupDto> for super::super::types::ProcessGroupDto {
    fn from(v: crate::v2_7_2::types::ProcessGroupDto) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            contents: v.contents.map(Into::into),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            disabled_count: v.disabled_count,
            execution_engine: v.execution_engine.map(|v| enum_to_string(&v)),
            flowfile_concurrency: v.flowfile_concurrency.map(|v| enum_to_string(&v)),
            flowfile_outbound_policy: v.flowfile_outbound_policy.map(|v| enum_to_string(&v)),
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            invalid_count: v.invalid_count,
            local_input_port_count: v.local_input_port_count,
            local_output_port_count: v.local_output_port_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            log_file_suffix: v.log_file_suffix,
            max_concurrent_tasks: v.max_concurrent_tasks,
            name: v.name,
            output_port_count: v.output_port_count,
            parameter_context: v.parameter_context.map(Into::into),
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            public_input_port_count: v.public_input_port_count,
            public_output_port_count: v.public_output_port_count,
            running_count: v.running_count,
            stale_count: v.stale_count,
            stateless_flow_timeout: v.stateless_flow_timeout,
            stateless_group_scheduled_state: v
                .stateless_group_scheduled_state
                .map(|v| enum_to_string(&v)),
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            version_control_information: v.version_control_information.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupEntity> for super::super::types::ProcessGroupEntity {
    fn from(v: crate::v2_7_2::types::ProcessGroupEntity) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disabled_count: v.disabled_count,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            invalid_count: v.invalid_count,
            local_input_port_count: v.local_input_port_count,
            local_output_port_count: v.local_output_port_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            output_port_count: v.output_port_count,
            parameter_context: v.parameter_context.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            process_group_update_strategy: v
                .process_group_update_strategy
                .map(|v| enum_to_string(&v)),
            public_input_port_count: v.public_input_port_count,
            public_output_port_count: v.public_output_port_count,
            revision: v.revision.map(Into::into),
            running_count: v.running_count,
            stale_count: v.stale_count,
            status: v.status.map(Into::into),
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            uri: v.uri,
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
            versioned_flow_state: v.versioned_flow_state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupFlowDto> for super::super::types::ProcessGroupFlowDto {
    fn from(v: crate::v2_7_2::types::ProcessGroupFlowDto) -> Self {
        Self {
            breadcrumb: v.breadcrumb.map(Into::into),
            flow: v.flow.map(Into::into),
            id: v.id,
            last_refreshed: v.last_refreshed,
            parameter_context: v.parameter_context.map(Into::into),
            parent_group_id: v.parent_group_id,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupFlowEntity>
    for super::super::types::ProcessGroupFlowEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupFlowEntity) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            process_group_flow: v.process_group_flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupImportEntity>
    for super::super::types::ProcessGroupImportEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupImportEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupNameDto> for super::super::types::ProcessGroupNameDto {
    fn from(v: crate::v2_7_2::types::ProcessGroupNameDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupReplaceRequestDto>
    for super::super::types::ProcessGroupReplaceRequestDto
{
    fn from(v: crate::v2_7_2::types::ProcessGroupReplaceRequestDto) -> Self {
        Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            process_group_id: v.process_group_id,
            request_id: v.request_id,
            state: v.state,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupReplaceRequestEntity>
    for super::super::types::ProcessGroupReplaceRequestEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupReplaceRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusDto>
    for super::super::types::ProcessGroupStatusDto
{
    fn from(v: crate::v2_7_2::types::ProcessGroupStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            stats_last_refreshed: v.stats_last_refreshed,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusEntity>
    for super::super::types::ProcessGroupStatusEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            process_group_status: v.process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusSnapshotDto>
    for super::super::types::ProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ProcessGroupStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_queued: v.bytes_queued,
            bytes_read: v.bytes_read,
            bytes_received: v.bytes_received,
            bytes_sent: v.bytes_sent,
            bytes_transferred: v.bytes_transferred,
            bytes_written: v.bytes_written,
            connection_status_snapshots: v
                .connection_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            flow_files_queued: v.flow_files_queued,
            flow_files_received: v.flow_files_received,
            flow_files_sent: v.flow_files_sent,
            flow_files_transferred: v.flow_files_transferred,
            id: v.id,
            input: v.input,
            input_port_status_snapshots: v
                .input_port_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            name: v.name,
            output: v.output,
            output_port_status_snapshots: v
                .output_port_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            process_group_status_snapshots: v
                .process_group_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processing_nanos: v.processing_nanos,
            processing_performance_status: v.processing_performance_status.map(Into::into),
            processor_status_snapshots: v
                .processor_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            queued: v.queued,
            queued_count: v.queued_count,
            queued_size: v.queued_size,
            read: v.read,
            received: v.received,
            remote_process_group_status_snapshots: v
                .remote_process_group_status_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            sent: v.sent,
            stateless_active_thread_count: v.stateless_active_thread_count,
            terminated_thread_count: v.terminated_thread_count,
            transferred: v.transferred,
            versioned_flow_state: v.versioned_flow_state.map(|v| enum_to_string(&v)),
            written: v.written,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusSnapshotEntity>
    for super::super::types::ProcessGroupStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            process_group_status_snapshot: v.process_group_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupUploadEntity>
    for super::super::types::ProcessGroupUploadEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupUploadEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            flow_snapshot: v.flow_snapshot.map(Into::into),
            group_id: v.group_id,
            group_name: v.group_name,
            position_d_t_o: v.position_d_t_o.map(Into::into),
            revision_d_t_o: v.revision_d_t_o.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupsEntity> for super::super::types::ProcessGroupsEntity {
    fn from(v: crate::v2_7_2::types::ProcessGroupsEntity) -> Self {
        Self {
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessingPerformanceStatusDto>
    for super::super::types::ProcessingPerformanceStatusDto
{
    fn from(v: crate::v2_7_2::types::ProcessingPerformanceStatusDto) -> Self {
        Self {
            content_read_duration: v.content_read_duration,
            content_write_duration: v.content_write_duration,
            cpu_duration: v.cpu_duration,
            garbage_collection_duration: v.garbage_collection_duration,
            identifier: v.identifier,
            session_commit_duration: v.session_commit_duration,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorConfigDto> for super::super::types::ProcessorConfigDto {
    fn from(v: crate::v2_7_2::types::ProcessorConfigDto) -> Self {
        Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            custom_ui_url: v.custom_ui_url,
            default_concurrent_tasks: v.default_concurrent_tasks,
            default_scheduling_period: v.default_scheduling_period,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            execution_node: v.execution_node,
            loss_tolerant: v.loss_tolerant,
            max_backoff_period: v.max_backoff_period,
            penalty_duration: v.penalty_duration,
            properties: v.properties,
            retried_relationships: v.retried_relationships,
            retry_count: v.retry_count,
            run_duration_millis: v.run_duration_millis,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorConfiguration>
    for super::super::types::ProcessorConfiguration
{
    fn from(v: crate::v2_7_2::types::ProcessorConfiguration) -> Self {
        Self {
            configuration: v.configuration,
            processor_class_name: v.processor_class_name,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorDefinition> for super::super::types::ProcessorDefinition {
    fn from(v: crate::v2_7_2::types::ProcessorDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            default_bulletin_level: v.default_bulletin_level,
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_penalty_duration: v.default_penalty_duration,
            default_scheduling_period_by_scheduling_strategy: v
                .default_scheduling_period_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy,
            default_yield_duration: v.default_yield_duration,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            dynamic_relationship: v.dynamic_relationship.map(Into::into),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            input_requirement: v.input_requirement.map(|v| enum_to_string(&v)),
            multi_processor_use_cases: v
                .multi_processor_use_cases
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            primary_node_only: v.primary_node_only,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reads_attributes: v
                .reads_attributes
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            side_effect_free: v.side_effect_free,
            stateful: v.stateful.map(Into::into),
            supported_relationships: v
                .supported_relationships
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            supported_scheduling_strategies: v.supported_scheduling_strategies,
            supports_batching: v.supports_batching,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_dynamic_relationships: v.supports_dynamic_relationships,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            trigger_serially: v.trigger_serially,
            trigger_when_any_destination_available: v.trigger_when_any_destination_available,
            trigger_when_empty: v.trigger_when_empty,
            r#type: v.r#type,
            type_description: v.type_description,
            use_cases: v
                .use_cases
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            version: v.version,
            writes_attributes: v
                .writes_attributes
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorDto> for super::super::types::ProcessorDto {
    fn from(v: crate::v2_7_2::types::ProcessorDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            config: v.config.map(Into::into),
            deprecated: v.deprecated,
            description: v.description,
            execution_node_restricted: v.execution_node_restricted,
            extension_missing: v.extension_missing,
            id: v.id,
            input_requirement: v.input_requirement,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            physical_state: v.physical_state.map(|v| enum_to_string(&v)),
            position: v.position.map(Into::into),
            relationships: v
                .relationships
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            state: v.state.map(|v| enum_to_string(&v)),
            style: v.style,
            supports_batching: v.supports_batching,
            supports_parallel_processing: v.supports_parallel_processing,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorEntity> for super::super::types::ProcessorEntity {
    fn from(v: crate::v2_7_2::types::ProcessorEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            input_requirement: v.input_requirement,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            physical_state: v.physical_state.map(|v| enum_to_string(&v)),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorRunStatusDetailsDto>
    for super::super::types::ProcessorRunStatusDetailsDto
{
    fn from(v: crate::v2_7_2::types::ProcessorRunStatusDetailsDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorRunStatusDetailsEntity>
    for super::super::types::ProcessorRunStatusDetailsEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorRunStatusDetailsEntity) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            revision: v.revision.map(Into::into),
            run_status_details: v.run_status_details.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorRunStatusEntity>
    for super::super::types::ProcessorRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusDto> for super::super::types::ProcessorStatusDto {
    fn from(v: crate::v2_7_2::types::ProcessorStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            stats_last_refreshed: v.stats_last_refreshed,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusEntity>
    for super::super::types::ProcessorStatusEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            processor_status: v.processor_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusSnapshotDto>
    for super::super::types::ProcessorStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ProcessorStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_read: v.bytes_read,
            bytes_written: v.bytes_written,
            execution_node: v.execution_node.map(|v| enum_to_string(&v)),
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            processing_performance_status: v.processing_performance_status.map(Into::into),
            read: v.read,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            task_count: v.task_count,
            tasks: v.tasks,
            tasks_duration: v.tasks_duration,
            tasks_duration_nanos: v.tasks_duration_nanos,
            terminated_thread_count: v.terminated_thread_count,
            r#type: v.r#type,
            written: v.written,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusSnapshotEntity>
    for super::super::types::ProcessorStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            processor_status_snapshot: v.processor_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorTypesEntity>
    for super::super::types::ProcessorTypesEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorTypesEntity) -> Self {
        Self {
            processor_types: v
                .processor_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorsEntity> for super::super::types::ProcessorsEntity {
    fn from(v: crate::v2_7_2::types::ProcessorsEntity) -> Self {
        Self {
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity>
    for super::super::types::ProcessorsRunStatusDetailsEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity) -> Self {
        Self {
            run_status_details: v
                .run_status_details
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyAllowableValue>
    for super::super::types::PropertyAllowableValue
{
    fn from(v: crate::v2_7_2::types::PropertyAllowableValue) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDependency> for super::super::types::PropertyDependency {
    fn from(v: crate::v2_7_2::types::PropertyDependency) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_display_name: v.property_display_name,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDependencyDto>
    for super::super::types::PropertyDependencyDto
{
    fn from(v: crate::v2_7_2::types::PropertyDependencyDto) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDescriptor> for super::super::types::PropertyDescriptor {
    fn from(v: crate::v2_7_2::types::PropertyDescriptor) -> Self {
        Self {
            allowable_values: v
                .allowable_values
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            default_value: v.default_value,
            dependencies: v
                .dependencies
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            description: v.description,
            display_name: v.display_name,
            dynamic: v.dynamic,
            expression_language_scope: v.expression_language_scope.map(|v| enum_to_string(&v)),
            expression_language_scope_description: v.expression_language_scope_description,
            listen_port_definition: v.listen_port_definition.map(Into::into),
            name: v.name,
            required: v.required,
            resource_definition: v.resource_definition.map(Into::into),
            sensitive: v.sensitive,
            type_provided_by_value: v.type_provided_by_value.map(Into::into),
            valid_regex: v.valid_regex,
            validator: v.validator,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDescriptorDto>
    for super::super::types::PropertyDescriptorDto
{
    fn from(v: crate::v2_7_2::types::PropertyDescriptorDto) -> Self {
        Self {
            allowable_values: v
                .allowable_values
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            default_value: v.default_value,
            dependencies: v
                .dependencies
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            description: v.description,
            display_name: v.display_name,
            dynamic: v.dynamic,
            expression_language_scope: v.expression_language_scope,
            identifies_controller_service: v.identifies_controller_service,
            identifies_controller_service_bundle: v
                .identifies_controller_service_bundle
                .map(Into::into),
            name: v.name,
            required: v.required,
            sensitive: v.sensitive,
            supports_el: v.supports_el,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDescriptorEntity>
    for super::super::types::PropertyDescriptorEntity
{
    fn from(v: crate::v2_7_2::types::PropertyDescriptorEntity) -> Self {
        Self {
            property_descriptor: Some(v.property_descriptor.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyHistoryDto> for super::super::types::PropertyHistoryDto {
    fn from(v: crate::v2_7_2::types::PropertyHistoryDto) -> Self {
        Self {
            previous_values: v
                .previous_values
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyListenPortDefinition>
    for super::super::types::PropertyListenPortDefinition
{
    fn from(v: crate::v2_7_2::types::PropertyListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyResourceDefinition>
    for super::super::types::PropertyResourceDefinition
{
    fn from(v: crate::v2_7_2::types::PropertyResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| enum_to_string(&v)),
            resource_types: v
                .resource_types
                .map(|v| v.into_iter().map(|v| enum_to_string(&v)).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceDto> for super::super::types::ProvenanceDto {
    fn from(v: crate::v2_7_2::types::ProvenanceDto) -> Self {
        Self {
            expiration: v.expiration,
            finished: v.finished,
            id: v.id,
            percent_completed: v.percent_completed,
            request: v.request.map(Into::into),
            results: v.results.map(Into::into),
            submission_time: v.submission_time,
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceEntity> for super::super::types::ProvenanceEntity {
    fn from(v: crate::v2_7_2::types::ProvenanceEntity) -> Self {
        Self {
            provenance: Some(v.provenance.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceEventDto> for super::super::types::ProvenanceEventDto {
    fn from(v: crate::v2_7_2::types::ProvenanceEventDto) -> Self {
        Self {
            alternate_identifier_uri: v.alternate_identifier_uri,
            attributes: v
                .attributes
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            child_uuids: v.child_uuids,
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            content_equal: v.content_equal,
            details: v.details,
            event_duration: v.event_duration,
            event_id: v.event_id,
            event_time: v.event_time,
            event_timestamp: None,
            event_type: v.event_type,
            file_size: v.file_size,
            file_size_bytes: v.file_size_bytes,
            flow_file_uuid: v.flow_file_uuid,
            group_id: v.group_id,
            id: v.id,
            input_content_available: v.input_content_available,
            input_content_claim_container: v.input_content_claim_container,
            input_content_claim_file_size: v.input_content_claim_file_size,
            input_content_claim_file_size_bytes: v.input_content_claim_file_size_bytes,
            input_content_claim_identifier: v.input_content_claim_identifier,
            input_content_claim_offset: v.input_content_claim_offset,
            input_content_claim_section: v.input_content_claim_section,
            lineage_duration: v.lineage_duration,
            output_content_available: v.output_content_available,
            output_content_claim_container: v.output_content_claim_container,
            output_content_claim_file_size: v.output_content_claim_file_size,
            output_content_claim_file_size_bytes: v.output_content_claim_file_size_bytes,
            output_content_claim_identifier: v.output_content_claim_identifier,
            output_content_claim_offset: v.output_content_claim_offset,
            output_content_claim_section: v.output_content_claim_section,
            parent_uuids: v.parent_uuids,
            relationship: v.relationship,
            replay_available: v.replay_available,
            replay_explanation: v.replay_explanation,
            source_connection_identifier: v.source_connection_identifier,
            source_system_flow_file_id: v.source_system_flow_file_id,
            transit_uri: v.transit_uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceEventEntity>
    for super::super::types::ProvenanceEventEntity
{
    fn from(v: crate::v2_7_2::types::ProvenanceEventEntity) -> Self {
        Self {
            provenance_event: Some(v.provenance_event.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceLinkDto> for super::super::types::ProvenanceLinkDto {
    fn from(v: crate::v2_7_2::types::ProvenanceLinkDto) -> Self {
        Self {
            flow_file_uuid: v.flow_file_uuid,
            millis: v.millis,
            source_id: v.source_id,
            target_id: v.target_id,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceNodeDto> for super::super::types::ProvenanceNodeDto {
    fn from(v: crate::v2_7_2::types::ProvenanceNodeDto) -> Self {
        Self {
            child_uuids: v.child_uuids,
            cluster_node_identifier: v.cluster_node_identifier,
            component_type: None,
            event_type: v.event_type,
            flow_file_uuid: v.flow_file_uuid,
            id: v.id,
            millis: v.millis,
            parent_uuids: v.parent_uuids,
            timestamp: v.timestamp,
            r#type: v.r#type.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceOptionsDto>
    for super::super::types::ProvenanceOptionsDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceOptionsDto) -> Self {
        Self {
            searchable_fields: v
                .searchable_fields
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceOptionsEntity>
    for super::super::types::ProvenanceOptionsEntity
{
    fn from(v: crate::v2_7_2::types::ProvenanceOptionsEntity) -> Self {
        Self {
            provenance_options: Some(v.provenance_options.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceRequestDto>
    for super::super::types::ProvenanceRequestDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceRequestDto) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            end_date: v.end_date,
            incremental_results: v.incremental_results,
            max_results: v.max_results,
            maximum_file_size: v.maximum_file_size,
            minimum_file_size: v.minimum_file_size,
            search_terms: v
                .search_terms
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            start_date: v.start_date,
            summarize: v.summarize,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceResultsDto>
    for super::super::types::ProvenanceResultsDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceResultsDto) -> Self {
        Self {
            errors: v.errors,
            generated: v.generated,
            oldest_event: v.oldest_event,
            provenance_events: v
                .provenance_events
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            time_offset: v.time_offset,
            total: v.total,
            total_count: v.total_count,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceSearchValueDto>
    for super::super::types::ProvenanceSearchValueDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceSearchValueDto) -> Self {
        Self {
            inverse: v.inverse,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceSearchableFieldDto>
    for super::super::types::ProvenanceSearchableFieldDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceSearchableFieldDto) -> Self {
        Self {
            field: v.field,
            id: v.id,
            label: v.label,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::QueueSizeDto> for super::super::types::QueueSizeDto {
    fn from(v: crate::v2_7_2::types::QueueSizeDto) -> Self {
        Self {
            byte_count: v.byte_count,
            object_count: v.object_count,
        }
    }
}

impl From<crate::v2_7_2::types::RegisteredFlow> for super::super::types::RegisteredFlow {
    fn from(v: crate::v2_7_2::types::RegisteredFlow) -> Self {
        Self {
            branch: v.branch,
            bucket_identifier: v.bucket_identifier,
            bucket_name: v.bucket_name,
            created_timestamp: v.created_timestamp,
            description: v.description,
            identifier: v.identifier,
            last_modified_timestamp: v.last_modified_timestamp,
            name: v.name,
            permissions: v.permissions.map(Into::into),
            version_count: v.version_count,
            version_info: v.version_info.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::RegisteredFlowSnapshot>
    for super::super::types::RegisteredFlowSnapshot
{
    fn from(v: crate::v2_7_2::types::RegisteredFlowSnapshot) -> Self {
        Self {
            bucket: v.bucket.map(Into::into),
            external_controller_services: v
                .external_controller_services
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            flow: v.flow.map(Into::into),
            flow_contents: v.flow_contents.map(Into::into),
            flow_encoding_version: v.flow_encoding_version,
            latest: v.latest,
            parameter_contexts: v
                .parameter_contexts
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            parameter_providers: v
                .parameter_providers
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            snapshot_metadata: v.snapshot_metadata.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::RegisteredFlowSnapshotMetadata>
    for super::super::types::RegisteredFlowSnapshotMetadata
{
    fn from(v: crate::v2_7_2::types::RegisteredFlowSnapshotMetadata) -> Self {
        Self {
            author: v.author,
            branch: v.branch,
            bucket_identifier: v.bucket_identifier,
            comments: v.comments,
            flow_identifier: v.flow_identifier,
            flow_name: v.flow_name,
            registry_identifier: v.registry_identifier,
            registry_name: v.registry_name,
            timestamp: v.timestamp,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::RegisteredFlowVersionInfo>
    for super::super::types::RegisteredFlowVersionInfo
{
    fn from(v: crate::v2_7_2::types::RegisteredFlowVersionInfo) -> Self {
        Self { version: v.version }
    }
}

impl From<crate::v2_7_2::types::Relationship> for super::super::types::Relationship {
    fn from(v: crate::v2_7_2::types::Relationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::RelationshipDto> for super::super::types::RelationshipDto {
    fn from(v: crate::v2_7_2::types::RelationshipDto) -> Self {
        Self {
            auto_terminate: v.auto_terminate,
            description: v.description,
            name: v.name,
            retry: v.retry,
        }
    }
}

impl From<crate::v2_7_2::types::RemotePortRunStatusEntity>
    for super::super::types::RemotePortRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::RemotePortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupContentsDto>
    for super::super::types::RemoteProcessGroupContentsDto
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupContentsDto) -> Self {
        Self {
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupDto>
    for super::super::types::RemoteProcessGroupDto
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupDto) -> Self {
        Self {
            active_remote_input_port_count: v.active_remote_input_port_count,
            active_remote_output_port_count: v.active_remote_output_port_count,
            authorization_issues: v.authorization_issues,
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            contents: v.contents.map(Into::into),
            flow_refreshed: v.flow_refreshed,
            id: v.id,
            inactive_remote_input_port_count: v.inactive_remote_input_port_count,
            inactive_remote_output_port_count: v.inactive_remote_output_port_count,
            input_port_count: v.input_port_count,
            local_network_interface: v.local_network_interface,
            name: v.name,
            output_port_count: v.output_port_count,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            proxy_host: v.proxy_host,
            proxy_password: v.proxy_password,
            proxy_port: v.proxy_port,
            proxy_user: v.proxy_user,
            target_secure: v.target_secure,
            target_uri: v.target_uri,
            target_uris: v.target_uris,
            transmitting: v.transmitting,
            transport_protocol: v.transport_protocol,
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupEntity>
    for super::super::types::RemoteProcessGroupEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            input_port_count: v.input_port_count,
            operate_permissions: v.operate_permissions.map(Into::into),
            output_port_count: v.output_port_count,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupPortDto>
    for super::super::types::RemoteProcessGroupPortDto
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupPortDto) -> Self {
        Self {
            batch_settings: v.batch_settings.map(Into::into),
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            connected: v.connected,
            exists: v.exists,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            target_id: v.target_id,
            target_running: v.target_running,
            transmitting: v.transmitting,
            use_compression: v.use_compression,
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupPortEntity>
    for super::super::types::RemoteProcessGroupPortEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupPortEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            remote_process_group_port: v.remote_process_group_port.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusDto>
    for super::super::types::RemoteProcessGroupStatusDto
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            stats_last_refreshed: v.stats_last_refreshed,
            target_uri: v.target_uri,
            transmission_status: v.transmission_status,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusEntity>
    for super::super::types::RemoteProcessGroupStatusEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            remote_process_group_status: v.remote_process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotDto>
    for super::super::types::RemoteProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_received: v.bytes_received,
            bytes_sent: v.bytes_sent,
            flow_files_received: v.flow_files_received,
            flow_files_sent: v.flow_files_sent,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            received: v.received,
            sent: v.sent,
            target_uri: v.target_uri,
            transmission_status: v.transmission_status,
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotEntity>
    for super::super::types::RemoteProcessGroupStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            remote_process_group_status_snapshot: v
                .remote_process_group_status_snapshot
                .map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupsEntity>
    for super::super::types::RemoteProcessGroupsEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupsEntity) -> Self {
        Self {
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ReplayLastEventRequestEntity>
    for super::super::types::ReplayLastEventRequestEntity
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventRequestEntity) -> Self {
        Self {
            component_id: v.component_id,
            nodes: v.nodes.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ReplayLastEventResponseEntity>
    for super::super::types::ReplayLastEventResponseEntity
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventResponseEntity) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            component_id: v.component_id,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ReplayLastEventSnapshotDto>
    for super::super::types::ReplayLastEventSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventSnapshotDto) -> Self {
        Self {
            event_available: v.event_available,
            events_replayed: v.events_replayed,
            failure_explanation: v.failure_explanation,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskDefinition>
    for super::super::types::ReportingTaskDefinition
{
    fn from(v: crate::v2_7_2::types::ReportingTaskDefinition) -> Self {
        Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(Into::into),
            default_scheduling_period_by_scheduling_strategy: v
                .default_scheduling_period_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(Into::into),
            supported_scheduling_strategies: v.supported_scheduling_strategies,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskDto> for super::super::types::ReportingTaskDto {
    fn from(v: crate::v2_7_2::types::ReportingTaskDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            custom_ui_url: v.custom_ui_url,
            default_scheduling_period: v.default_scheduling_period,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(Into::into),
            properties: v.properties,
            restricted: v.restricted,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v.state.map(|v| enum_to_string(&v)),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskEntity> for super::super::types::ReportingTaskEntity {
    fn from(v: crate::v2_7_2::types::ReportingTaskEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(Into::into),
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskRunStatusEntity>
    for super::super::types::ReportingTaskRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ReportingTaskRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskStatusDto>
    for super::super::types::ReportingTaskStatusDto
{
    fn from(v: crate::v2_7_2::types::ReportingTaskStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| enum_to_string(&v)),
            validation_status: v.validation_status.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskTypesEntity>
    for super::super::types::ReportingTaskTypesEntity
{
    fn from(v: crate::v2_7_2::types::ReportingTaskTypesEntity) -> Self {
        Self {
            reporting_task_types: v
                .reporting_task_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTasksEntity>
    for super::super::types::ReportingTasksEntity
{
    fn from(v: crate::v2_7_2::types::ReportingTasksEntity) -> Self {
        Self {
            current_time: v.current_time,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::RequiredPermissionDto>
    for super::super::types::RequiredPermissionDto
{
    fn from(v: crate::v2_7_2::types::RequiredPermissionDto) -> Self {
        Self {
            id: v.id,
            label: v.label,
        }
    }
}

impl From<crate::v2_7_2::types::ResourceClaimDetailsDto>
    for super::super::types::ResourceClaimDetailsDto
{
    fn from(v: crate::v2_7_2::types::ResourceClaimDetailsDto) -> Self {
        Self {
            awaiting_destruction: v.awaiting_destruction,
            claimant_count: v.claimant_count,
            container: v.container,
            identifier: v.identifier,
            in_use: v.in_use,
            section: v.section,
            writable: v.writable,
        }
    }
}

impl From<crate::v2_7_2::types::ResourceDto> for super::super::types::ResourceDto {
    fn from(v: crate::v2_7_2::types::ResourceDto) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ResourcesEntity> for super::super::types::ResourcesEntity {
    fn from(v: crate::v2_7_2::types::ResourcesEntity) -> Self {
        Self {
            resources: v
                .resources
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::Restriction> for super::super::types::Restriction {
    fn from(v: crate::v2_7_2::types::Restriction) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission,
        }
    }
}

impl From<crate::v2_7_2::types::RevisionDto> for super::super::types::RevisionDto {
    fn from(v: crate::v2_7_2::types::RevisionDto) -> Self {
        Self {
            client_id: v.client_id,
            last_modifier: v.last_modifier,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::RunStatusDetailsRequestEntity>
    for super::super::types::RunStatusDetailsRequestEntity
{
    fn from(v: crate::v2_7_2::types::RunStatusDetailsRequestEntity) -> Self {
        Self {
            processor_ids: v.processor_ids,
        }
    }
}

impl From<crate::v2_7_2::types::RuntimeManifest> for super::super::types::RuntimeManifest {
    fn from(v: crate::v2_7_2::types::RuntimeManifest) -> Self {
        Self {
            agent_type: v.agent_type,
            build_info: v.build_info.map(Into::into),
            bundles: v.bundles.map(|v| v.into_iter().map(|v| v.into()).collect()),
            identifier: v.identifier,
            scheduling_defaults: v.scheduling_defaults.map(Into::into),
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::RuntimeManifestEntity>
    for super::super::types::RuntimeManifestEntity
{
    fn from(v: crate::v2_7_2::types::RuntimeManifestEntity) -> Self {
        Self {
            runtime_manifest: Some(v.runtime_manifest.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::ScheduleComponentsEntity>
    for super::super::types::ScheduleComponentsEntity
{
    fn from(v: crate::v2_7_2::types::ScheduleComponentsEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::SchedulingDefaults> for super::super::types::SchedulingDefaults {
    fn from(v: crate::v2_7_2::types::SchedulingDefaults) -> Self {
        Self {
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_max_concurrent_tasks: v.default_max_concurrent_tasks,
            default_run_duration_nanos: v.default_run_duration_nanos,
            default_scheduling_period_millis: v.default_scheduling_period_millis,
            default_scheduling_periods_by_scheduling_strategy: v
                .default_scheduling_periods_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy.map(|v| enum_to_string(&v)),
            penalization_period_millis: v.penalization_period_millis,
            yield_duration_millis: v.yield_duration_millis,
        }
    }
}

impl From<crate::v2_7_2::types::SearchResultGroupDto>
    for super::super::types::SearchResultGroupDto
{
    fn from(v: crate::v2_7_2::types::SearchResultGroupDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::SearchResultsDto> for super::super::types::SearchResultsDto {
    fn from(v: crate::v2_7_2::types::SearchResultsDto) -> Self {
        Self {
            connection_results: v
                .connection_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_service_node_results: v
                .controller_service_node_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            funnel_results: v
                .funnel_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            input_port_results: v
                .input_port_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            label_results: v
                .label_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            output_port_results: v
                .output_port_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_context_results: v
                .parameter_context_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_provider_node_results: v
                .parameter_provider_node_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_results: v
                .parameter_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            process_group_results: v
                .process_group_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            processor_results: v
                .processor_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_process_group_results: v
                .remote_process_group_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::SearchResultsEntity> for super::super::types::SearchResultsEntity {
    fn from(v: crate::v2_7_2::types::SearchResultsEntity) -> Self {
        Self {
            search_results_d_t_o: Some(v.search_results_d_t_o.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::SnippetDto> for super::super::types::SnippetDto {
    fn from(v: crate::v2_7_2::types::SnippetDto) -> Self {
        Self {
            connections: v
                .connections
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            funnels: v
                .funnels
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            id: v.id,
            input_ports: v
                .input_ports
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            labels: v
                .labels
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            output_ports: v
                .output_ports
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            parent_group_id: v.parent_group_id,
            process_groups: v
                .process_groups
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            processors: v
                .processors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            remote_process_groups: v
                .remote_process_groups
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::SnippetEntity> for super::super::types::SnippetEntity {
    fn from(v: crate::v2_7_2::types::SnippetEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            snippet: v.snippet.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StartVersionControlRequestEntity>
    for super::super::types::StartVersionControlRequestEntity
{
    fn from(v: crate::v2_7_2::types::StartVersionControlRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow: v.versioned_flow.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StateEntryDto> for super::super::types::StateEntryDto {
    fn from(v: crate::v2_7_2::types::StateEntryDto) -> Self {
        Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            key: v.key,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::StateMapDto> for super::super::types::StateMapDto {
    fn from(v: crate::v2_7_2::types::StateMapDto) -> Self {
        Self {
            scope: v.scope,
            state: v.state.map(|v| v.into_iter().map(|v| v.into()).collect()),
            total_entry_count: v.total_entry_count,
        }
    }
}

impl From<crate::v2_7_2::types::Stateful> for super::super::types::Stateful {
    fn from(v: crate::v2_7_2::types::Stateful) -> Self {
        Self {
            description: v.description,
            scopes: v
                .scopes
                .map(|v| v.into_iter().map(|v| enum_to_string(&v)).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::StatusDescriptorDto> for super::super::types::StatusDescriptorDto {
    fn from(v: crate::v2_7_2::types::StatusDescriptorDto) -> Self {
        Self {
            description: v.description,
            field: v.field,
            formatter: v.formatter,
            label: v.label,
        }
    }
}

impl From<crate::v2_7_2::types::StatusHistoryDto> for super::super::types::StatusHistoryDto {
    fn from(v: crate::v2_7_2::types::StatusHistoryDto) -> Self {
        Self {
            aggregate_snapshots: v
                .aggregate_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component_details: v.component_details,
            field_descriptors: v
                .field_descriptors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::StatusHistoryEntity> for super::super::types::StatusHistoryEntity {
    fn from(v: crate::v2_7_2::types::StatusHistoryEntity) -> Self {
        Self {
            can_read: v.can_read,
            status_history: v.status_history.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StatusSnapshotDto> for super::super::types::StatusSnapshotDto {
    fn from(v: crate::v2_7_2::types::StatusSnapshotDto) -> Self {
        Self {
            status_metrics: v.status_metrics,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::StorageUsageDto> for super::super::types::StorageUsageDto {
    fn from(v: crate::v2_7_2::types::StorageUsageDto) -> Self {
        Self {
            free_space: v.free_space,
            free_space_bytes: v.free_space_bytes,
            identifier: v.identifier,
            total_space: v.total_space,
            total_space_bytes: v.total_space_bytes,
            used_space: v.used_space,
            used_space_bytes: v.used_space_bytes,
            utilization: v.utilization,
        }
    }
}

impl From<crate::v2_7_2::types::StreamingOutput> for super::super::types::StreamingOutput {
    fn from(_v: crate::v2_7_2::types::StreamingOutput) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::SubmitReplayRequestEntity>
    for super::super::types::SubmitReplayRequestEntity
{
    fn from(v: crate::v2_7_2::types::SubmitReplayRequestEntity) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
        }
    }
}

impl From<crate::v2_7_2::types::SupportedMimeTypesDto>
    for super::super::types::SupportedMimeTypesDto
{
    fn from(v: crate::v2_7_2::types::SupportedMimeTypesDto) -> Self {
        Self {
            display_name: v.display_name,
            mime_types: v.mime_types,
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsDto>
    for super::super::types::SystemDiagnosticsDto
{
    fn from(v: crate::v2_7_2::types::SystemDiagnosticsDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsEntity>
    for super::super::types::SystemDiagnosticsEntity
{
    fn from(v: crate::v2_7_2::types::SystemDiagnosticsEntity) -> Self {
        Self {
            system_diagnostics: Some(v.system_diagnostics.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsSnapshotDto>
    for super::super::types::SystemDiagnosticsSnapshotDto
{
    fn from(v: crate::v2_7_2::types::SystemDiagnosticsSnapshotDto) -> Self {
        Self {
            available_processors: v.available_processors,
            content_repository_storage_usage: v
                .content_repository_storage_usage
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            daemon_threads: v.daemon_threads,
            flow_file_repository_storage_usage: v
                .flow_file_repository_storage_usage
                .map(Into::into),
            free_heap: v.free_heap,
            free_heap_bytes: v.free_heap_bytes,
            free_non_heap: v.free_non_heap,
            free_non_heap_bytes: v.free_non_heap_bytes,
            garbage_collection: v
                .garbage_collection
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            heap_utilization: v.heap_utilization,
            max_heap: v.max_heap,
            max_heap_bytes: v.max_heap_bytes,
            max_non_heap: v.max_non_heap,
            max_non_heap_bytes: v.max_non_heap_bytes,
            non_heap_utilization: v.non_heap_utilization,
            processor_load_average: v.processor_load_average,
            provenance_repository_storage_usage: v
                .provenance_repository_storage_usage
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            resource_claim_details: v
                .resource_claim_details
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            stats_last_refreshed: v.stats_last_refreshed,
            total_heap: v.total_heap,
            total_heap_bytes: v.total_heap_bytes,
            total_non_heap: v.total_non_heap,
            total_non_heap_bytes: v.total_non_heap_bytes,
            total_threads: v.total_threads,
            uptime: v.uptime,
            used_heap: v.used_heap,
            used_heap_bytes: v.used_heap_bytes,
            used_non_heap: v.used_non_heap,
            used_non_heap_bytes: v.used_non_heap_bytes,
            version_info: v.version_info.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::SystemResourceConsideration>
    for super::super::types::SystemResourceConsideration
{
    fn from(v: crate::v2_7_2::types::SystemResourceConsideration) -> Self {
        Self {
            description: v.description,
            resource: v.resource,
        }
    }
}

impl From<crate::v2_7_2::types::TenantDto> for super::super::types::TenantDto {
    fn from(v: crate::v2_7_2::types::TenantDto) -> Self {
        Self {
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::TenantEntity> for super::super::types::TenantEntity {
    fn from(v: crate::v2_7_2::types::TenantEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::TenantsEntity> for super::super::types::TenantsEntity {
    fn from(v: crate::v2_7_2::types::TenantsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::TransactionResultEntity>
    for super::super::types::TransactionResultEntity
{
    fn from(v: crate::v2_7_2::types::TransactionResultEntity) -> Self {
        Self {
            flow_file_sent: v.flow_file_sent,
            message: v.message,
            response_code: v.response_code,
        }
    }
}

impl From<crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity>
    for super::super::types::UpdateControllerServiceReferenceRequestEntity
{
    fn from(v: crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            referencing_component_revisions: v
                .referencing_component_revisions
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            state: v.state.map(|v| enum_to_string(&v)),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_7_2::types::UseCase> for super::super::types::UseCase {
    fn from(v: crate::v2_7_2::types::UseCase) -> Self {
        Self {
            configuration: v.configuration,
            description: v.description,
            input_requirement: v.input_requirement.map(|v| enum_to_string(&v)),
            keywords: v.keywords,
            notes: v.notes,
        }
    }
}

impl From<crate::v2_7_2::types::UserDto> for super::super::types::UserDto {
    fn from(v: crate::v2_7_2::types::UserDto) -> Self {
        Self {
            access_policies: v
                .access_policies
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::UserEntity> for super::super::types::UserEntity {
    fn from(v: crate::v2_7_2::types::UserEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::UserGroupDto> for super::super::types::UserGroupDto {
    fn from(v: crate::v2_7_2::types::UserGroupDto) -> Self {
        Self {
            access_policies: v
                .access_policies
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::UserGroupEntity> for super::super::types::UserGroupEntity {
    fn from(v: crate::v2_7_2::types::UserGroupEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::UserGroupsEntity> for super::super::types::UserGroupsEntity {
    fn from(v: crate::v2_7_2::types::UserGroupsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::UsersEntity> for super::super::types::UsersEntity {
    fn from(v: crate::v2_7_2::types::UsersEntity) -> Self {
        Self {
            generated: v.generated,
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VerifyConfigRequestDto>
    for super::super::types::VerifyConfigRequestDto
{
    fn from(v: crate::v2_7_2::types::VerifyConfigRequestDto) -> Self {
        Self {
            attributes: v.attributes,
            complete: v.complete,
            component_id: v.component_id,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            properties: v.properties,
            request_id: v.request_id,
            results: v.results.map(|v| v.into_iter().map(|v| v.into()).collect()),
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::VerifyConfigRequestEntity>
    for super::super::types::VerifyConfigRequestEntity
{
    fn from(v: crate::v2_7_2::types::VerifyConfigRequestEntity) -> Self {
        Self {
            request: Some(v.request.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::VerifyConfigUpdateStepDto>
    for super::super::types::VerifyConfigUpdateStepDto
{
    fn from(v: crate::v2_7_2::types::VerifyConfigUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_7_2::types::VersionControlComponentMappingEntity>
    for super::super::types::VersionControlComponentMappingEntity
{
    fn from(v: crate::v2_7_2::types::VersionControlComponentMappingEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            version_control_component_mapping: v.version_control_component_mapping,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionControlInformationDto>
    for super::super::types::VersionControlInformationDto
{
    fn from(v: crate::v2_7_2::types::VersionControlInformationDto) -> Self {
        Self {
            branch: v.branch,
            bucket_id: v.bucket_id,
            bucket_name: v.bucket_name,
            flow_description: v.flow_description,
            flow_id: v.flow_id,
            flow_name: v.flow_name,
            group_id: v.group_id,
            registry_id: v.registry_id,
            registry_name: v.registry_name,
            state: v.state.map(|v| enum_to_string(&v)),
            state_explanation: v.state_explanation,
            storage_location: v.storage_location,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::VersionControlInformationEntity>
    for super::super::types::VersionControlInformationEntity
{
    fn from(v: crate::v2_7_2::types::VersionControlInformationEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionInfoDto> for super::super::types::VersionInfoDto {
    fn from(v: crate::v2_7_2::types::VersionInfoDto) -> Self {
        Self {
            build_branch: v.build_branch,
            build_revision: v.build_revision,
            build_tag: v.build_tag,
            build_timestamp: v.build_timestamp,
            java_vendor: v.java_vendor,
            java_version: v.java_version,
            ni_fi_version: v.ni_fi_version,
            os_architecture: v.os_architecture,
            os_name: v.os_name,
            os_version: v.os_version,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedAsset> for super::super::types::VersionedAsset {
    fn from(v: crate::v2_7_2::types::VersionedAsset) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedConnection> for super::super::types::VersionedConnection {
    fn from(v: crate::v2_7_2::types::VersionedConnection) -> Self {
        Self {
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            destination: v.destination.map(Into::into),
            flow_file_expiration: v.flow_file_expiration,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            label_index: v.label_index,
            load_balance_compression: v.load_balance_compression,
            load_balance_strategy: v.load_balance_strategy,
            name: v.name,
            partitioning_attribute: v.partitioning_attribute,
            position: v.position.map(Into::into),
            prioritizers: v.prioritizers,
            selected_relationships: v.selected_relationships,
            source: v.source.map(Into::into),
            z_index: v.z_index,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedControllerService>
    for super::super::types::VersionedControllerService
{
    fn from(v: crate::v2_7_2::types::VersionedControllerService) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            controller_service_apis: v
                .controller_service_apis
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowCoordinates>
    for super::super::types::VersionedFlowCoordinates
{
    fn from(v: crate::v2_7_2::types::VersionedFlowCoordinates) -> Self {
        Self {
            branch: v.branch,
            bucket_id: v.bucket_id,
            flow_id: v.flow_id,
            latest: v.latest,
            registry_id: v.registry_id,
            storage_location: v.storage_location,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowDto> for super::super::types::VersionedFlowDto {
    fn from(v: crate::v2_7_2::types::VersionedFlowDto) -> Self {
        Self {
            action: v.action.map(|v| enum_to_string(&v)),
            branch: v.branch,
            bucket_id: v.bucket_id,
            comments: v.comments,
            description: v.description,
            flow_id: v.flow_id,
            flow_name: v.flow_name,
            registry_id: v.registry_id,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowEntity> for super::super::types::VersionedFlowEntity {
    fn from(v: crate::v2_7_2::types::VersionedFlowEntity) -> Self {
        Self {
            versioned_flow: Some(v.versioned_flow.unwrap_or_default().into()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowSnapshotEntity>
    for super::super::types::VersionedFlowSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowSnapshotEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            registry_id: v.registry_id,
            update_descendant_versioned_flows: v.update_descendant_versioned_flows,
            versioned_flow: v.versioned_flow.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowSnapshotMetadataEntity>
    for super::super::types::VersionedFlowSnapshotMetadataEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowSnapshotMetadataEntity) -> Self {
        Self {
            registry_id: v.registry_id,
            versioned_flow_snapshot_metadata: v.versioned_flow_snapshot_metadata.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowSnapshotMetadataSetEntity>
    for super::super::types::VersionedFlowSnapshotMetadataSetEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowSnapshotMetadataSetEntity) -> Self {
        Self {
            versioned_flow_snapshot_metadata_set: v
                .versioned_flow_snapshot_metadata_set
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowUpdateRequestDto>
    for super::super::types::VersionedFlowUpdateRequestDto
{
    fn from(v: crate::v2_7_2::types::VersionedFlowUpdateRequestDto) -> Self {
        Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            process_group_id: v.process_group_id,
            request_id: v.request_id,
            state: v.state,
            uri: v.uri,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowUpdateRequestEntity>
    for super::super::types::VersionedFlowUpdateRequestEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowUpdateRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowsEntity>
    for super::super::types::VersionedFlowsEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowsEntity) -> Self {
        Self {
            versioned_flows: v
                .versioned_flows
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFunnel> for super::super::types::VersionedFunnel {
    fn from(v: crate::v2_7_2::types::VersionedFunnel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedLabel> for super::super::types::VersionedLabel {
    fn from(v: crate::v2_7_2::types::VersionedLabel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            group_identifier: v.group_identifier,
            height: v.height,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            label: v.label,
            name: v.name,
            position: v.position.map(Into::into),
            style: v.style,
            width: v.width,
            z_index: v.z_index,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedListenPortDefinition>
    for super::super::types::VersionedListenPortDefinition
{
    fn from(v: crate::v2_7_2::types::VersionedListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedParameter> for super::super::types::VersionedParameter {
    fn from(v: crate::v2_7_2::types::VersionedParameter) -> Self {
        Self {
            description: v.description,
            name: v.name,
            provided: v.provided,
            referenced_assets: v
                .referenced_assets
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            sensitive: v.sensitive,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedParameterContext>
    for super::super::types::VersionedParameterContext
{
    fn from(v: crate::v2_7_2::types::VersionedParameterContext) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            description: v.description,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            inherited_parameter_contexts: v.inherited_parameter_contexts,
            instance_identifier: v.instance_identifier,
            name: v.name,
            parameter_group_name: v.parameter_group_name,
            parameter_provider: v.parameter_provider,
            parameters: v
                .parameters
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            position: v.position.map(Into::into),
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedPort> for super::super::types::VersionedPort {
    fn from(v: crate::v2_7_2::types::VersionedPort) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            port_function: v.port_function.map(|v| enum_to_string(&v)),
            position: v.position.map(Into::into),
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            r#type: v.r#type.map(|v| enum_to_string(&v)),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedProcessGroup>
    for super::super::types::VersionedProcessGroup
{
    fn from(v: crate::v2_7_2::types::VersionedProcessGroup) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            execution_engine: v.execution_engine.map(|v| enum_to_string(&v)),
            flow_file_concurrency: v.flow_file_concurrency,
            flow_file_outbound_policy: v.flow_file_outbound_policy,
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            instance_identifier: v.instance_identifier,
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
            log_file_suffix: v.log_file_suffix,
            max_concurrent_tasks: v.max_concurrent_tasks,
            name: v.name,
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            parameter_context_name: v.parameter_context_name,
            position: v.position.map(Into::into),
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| Box::new((*v).into())).collect()),
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            stateless_flow_timeout: v.stateless_flow_timeout,
            versioned_flow_coordinates: v.versioned_flow_coordinates.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedProcessor> for super::super::types::VersionedProcessor {
    fn from(v: crate::v2_7_2::types::VersionedProcessor) -> Self {
        Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            execution_node: v.execution_node,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            max_backoff_period: v.max_backoff_period,
            name: v.name,
            penalty_duration: v.penalty_duration,
            position: v.position.map(Into::into),
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            retried_relationships: v.retried_relationships,
            retry_count: v.retry_count,
            run_duration_millis: v.run_duration_millis,
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            style: v.style,
            r#type: v.r#type,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedPropertyDescriptor>
    for super::super::types::VersionedPropertyDescriptor
{
    fn from(v: crate::v2_7_2::types::VersionedPropertyDescriptor) -> Self {
        Self {
            display_name: v.display_name,
            dynamic: v.dynamic,
            identifies_controller_service: v.identifies_controller_service,
            listen_port_definition: v.listen_port_definition.map(Into::into),
            name: v.name,
            resource_definition: v.resource_definition.map(Into::into),
            sensitive: v.sensitive,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedRemoteGroupPort>
    for super::super::types::VersionedRemoteGroupPort
{
    fn from(v: crate::v2_7_2::types::VersionedRemoteGroupPort) -> Self {
        Self {
            batch_size: v.batch_size.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            remote_group_id: v.remote_group_id,
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            target_id: v.target_id,
            use_compression: v.use_compression,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedRemoteProcessGroup>
    for super::super::types::VersionedRemoteProcessGroup
{
    fn from(v: crate::v2_7_2::types::VersionedRemoteProcessGroup) -> Self {
        Self {
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            instance_identifier: v.instance_identifier,
            local_network_interface: v.local_network_interface,
            name: v.name,
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            position: v.position.map(Into::into),
            proxy_host: v.proxy_host,
            proxy_password: v.proxy_password,
            proxy_port: v.proxy_port,
            proxy_user: v.proxy_user,
            target_uris: v.target_uris,
            transport_protocol: v.transport_protocol,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTask>
    for super::super::types::VersionedReportingTask
{
    fn from(v: crate::v2_7_2::types::VersionedReportingTask) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| enum_to_string(&v)),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            scheduled_state: v.scheduled_state.map(|v| enum_to_string(&v)),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity>
    for super::super::types::VersionedReportingTaskImportRequestEntity
{
    fn from(v: crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            reporting_task_snapshot: v.reporting_task_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTaskImportResponseEntity>
    for super::super::types::VersionedReportingTaskImportResponseEntity
{
    fn from(v: crate::v2_7_2::types::VersionedReportingTaskImportResponseEntity) -> Self {
        Self {
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTaskSnapshot>
    for super::super::types::VersionedReportingTaskSnapshot
{
    fn from(v: crate::v2_7_2::types::VersionedReportingTaskSnapshot) -> Self {
        Self {
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedResourceDefinition>
    for super::super::types::VersionedResourceDefinition
{
    fn from(v: crate::v2_7_2::types::VersionedResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| enum_to_string(&v)),
            resource_types: v
                .resource_types
                .map(|v| v.into_iter().map(|v| enum_to_string(&v)).collect()),
        }
    }
}

impl TryFrom<super::super::types::IncludedRegistries> for crate::v2_7_2::types::IncludedRegistries {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::IncludedRegistries) -> Result<Self, Self::Error> {
        #[allow(unreachable_patterns)]
        match v {
            super::super::types::IncludedRegistries::Bulletin => {
                Ok(crate::v2_7_2::types::IncludedRegistries::Bulletin)
            }
            super::super::types::IncludedRegistries::Cluster => {
                Ok(crate::v2_7_2::types::IncludedRegistries::Cluster)
            }
            super::super::types::IncludedRegistries::Connection => {
                Ok(crate::v2_7_2::types::IncludedRegistries::Connection)
            }
            super::super::types::IncludedRegistries::Jvm => {
                Ok(crate::v2_7_2::types::IncludedRegistries::Jvm)
            }
            super::super::types::IncludedRegistries::Nifi => {
                Ok(crate::v2_7_2::types::IncludedRegistries::Nifi)
            }
            _ => Err(crate::NifiError::UnsupportedEnumVariant {
                variant: format!("{v:?}"),
                type_name: "IncludedRegistries".to_string(),
                version: "2.7.2".to_string(),
            }),
        }
    }
}

impl TryFrom<super::super::types::FlowMetricsReportingStrategy>
    for crate::v2_7_2::types::FlowMetricsReportingStrategy
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowMetricsReportingStrategy) -> Result<Self, Self::Error> {
        #[allow(unreachable_patterns)]
        match v {
            super::super::types::FlowMetricsReportingStrategy::AllComponents => {
                Ok(crate::v2_7_2::types::FlowMetricsReportingStrategy::AllComponents)
            }
            super::super::types::FlowMetricsReportingStrategy::AllProcessGroups => {
                Ok(crate::v2_7_2::types::FlowMetricsReportingStrategy::AllProcessGroups)
            }
            _ => Err(crate::NifiError::UnsupportedEnumVariant {
                variant: format!("{v:?}"),
                type_name: "FlowMetricsReportingStrategy".to_string(),
                version: "2.7.2".to_string(),
            }),
        }
    }
}

impl TryFrom<super::super::types::ParameterContextHandlingStrategy>
    for crate::v2_7_2::types::ParameterContextHandlingStrategy
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextHandlingStrategy,
    ) -> Result<Self, Self::Error> {
        #[allow(unreachable_patterns)]
        match v {
            super::super::types::ParameterContextHandlingStrategy::KeepExisting => {
                Ok(crate::v2_7_2::types::ParameterContextHandlingStrategy::KeepExisting)
            }
            super::super::types::ParameterContextHandlingStrategy::Replace => {
                Ok(crate::v2_7_2::types::ParameterContextHandlingStrategy::Replace)
            }
            _ => Err(crate::NifiError::UnsupportedEnumVariant {
                variant: format!("{v:?}"),
                type_name: "ParameterContextHandlingStrategy".to_string(),
                version: "2.7.2".to_string(),
            }),
        }
    }
}

impl TryFrom<super::super::types::DiagnosticLevel> for crate::v2_7_2::types::DiagnosticLevel {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DiagnosticLevel) -> Result<Self, Self::Error> {
        #[allow(unreachable_patterns)]
        match v {
            super::super::types::DiagnosticLevel::Basic => {
                Ok(crate::v2_7_2::types::DiagnosticLevel::Basic)
            }
            super::super::types::DiagnosticLevel::Verbose => {
                Ok(crate::v2_7_2::types::DiagnosticLevel::Verbose)
            }
            _ => Err(crate::NifiError::UnsupportedEnumVariant {
                variant: format!("{v:?}"),
                type_name: "DiagnosticLevel".to_string(),
                version: "2.7.2".to_string(),
            }),
        }
    }
}

impl TryFrom<super::super::types::AboutDto> for crate::v2_7_2::types::AboutDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AboutDto) -> Result<Self, Self::Error> {
        Ok(Self {
            build_branch: v.build_branch,
            build_revision: v.build_revision,
            build_tag: v.build_tag,
            build_timestamp: v.build_timestamp,
            content_viewer_url: v.content_viewer_url,
            timezone: v.timezone,
            title: v.title,
            uri: v.uri,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::AboutEntity> for crate::v2_7_2::types::AboutEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AboutEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            about: v
                .about
                .map(crate::v2_7_2::types::AboutDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::AccessPolicyDto> for crate::v2_7_2::types::AccessPolicyDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AccessPolicyDto) -> Result<Self, Self::Error> {
        Ok(Self {
            action: v
                .action
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            component_reference: v.component_reference.map(|v| v.try_into()).transpose()?,
            configurable: v.configurable,
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            resource: v.resource,
            user_groups: v
                .user_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            users: v
                .users
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::AccessPolicyEntity> for crate::v2_7_2::types::AccessPolicyEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AccessPolicyEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            generated: v.generated,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::AccessPolicySummaryDto>
    for crate::v2_7_2::types::AccessPolicySummaryDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AccessPolicySummaryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            action: v
                .action
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            component_reference: v.component_reference.map(|v| v.try_into()).transpose()?,
            configurable: v.configurable,
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            resource: v.resource,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::AccessPolicySummaryEntity>
    for crate::v2_7_2::types::AccessPolicySummaryEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AccessPolicySummaryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ActionDetailsDto> for crate::v2_7_2::types::ActionDetailsDto {
    type Error = crate::NifiError;
    fn try_from(_v: super::super::types::ActionDetailsDto) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<super::super::types::ActionDto> for crate::v2_7_2::types::ActionDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ActionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            action_details: v.action_details.map(|v| v.try_into()).transpose()?,
            component_details: v.component_details.map(|v| v.try_into()).transpose()?,
            id: v.id,
            operation: v.operation,
            source_id: v.source_id,
            source_name: v.source_name,
            source_type: v.source_type,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        })
    }
}

impl TryFrom<super::super::types::ActionEntity> for crate::v2_7_2::types::ActionEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ActionEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            action: v.action.map(|v| v.try_into()).transpose()?,
            can_read: v.can_read,
            id: v.id,
            source_id: v.source_id,
            timestamp: v.timestamp,
        })
    }
}

impl TryFrom<super::super::types::ActivateControllerServicesEntity>
    for crate::v2_7_2::types::ActivateControllerServicesEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ActivateControllerServicesEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            components: v
                .components
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::AdditionalDetailsEntity>
    for crate::v2_7_2::types::AdditionalDetailsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AdditionalDetailsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
        })
    }
}

impl TryFrom<super::super::types::AffectedComponentDto>
    for crate::v2_7_2::types::AffectedComponentDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AffectedComponentDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v
                .reference_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            state: v.state,
            validation_errors: v.validation_errors,
        })
    }
}

impl TryFrom<super::super::types::AffectedComponentEntity>
    for crate::v2_7_2::types::AffectedComponentEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AffectedComponentEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            process_group: v.process_group.map(|v| v.try_into()).transpose()?,
            reference_type: v
                .reference_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::AllowableValueDto> for crate::v2_7_2::types::AllowableValueDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AllowableValueDto) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::AllowableValueEntity>
    for crate::v2_7_2::types::AllowableValueEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AllowableValueEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            allowable_value: v.allowable_value.map(|v| v.try_into()).transpose()?,
            can_read: v.can_read,
        })
    }
}

impl TryFrom<super::super::types::AssetDto> for crate::v2_7_2::types::AssetDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AssetDto) -> Result<Self, Self::Error> {
        Ok(Self {
            digest: v.digest,
            id: v.id,
            missing_content: v.missing_content,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::AssetEntity> for crate::v2_7_2::types::AssetEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AssetEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            asset: v
                .asset
                .map(crate::v2_7_2::types::AssetDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::AssetReferenceDto> for crate::v2_7_2::types::AssetReferenceDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AssetReferenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::AssetsEntity> for crate::v2_7_2::types::AssetsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AssetsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            assets: v
                .assets
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::Attribute> for crate::v2_7_2::types::Attribute {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Attribute) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::AttributeDto> for crate::v2_7_2::types::AttributeDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::AttributeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: v.name,
            previous_value: v.previous_value,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::AuthenticationConfigurationDto>
    for crate::v2_7_2::types::AuthenticationConfigurationDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::AuthenticationConfigurationDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            external_login_required: v.external_login_required,
            login_supported: v.login_supported,
            login_uri: v.login_uri,
            logout_uri: v.logout_uri,
        })
    }
}

impl TryFrom<super::super::types::AuthenticationConfigurationEntity>
    for crate::v2_7_2::types::AuthenticationConfigurationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::AuthenticationConfigurationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            authentication_configuration: v
                .authentication_configuration
                .map(crate::v2_7_2::types::AuthenticationConfigurationDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::BannerDto> for crate::v2_7_2::types::BannerDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BannerDto) -> Result<Self, Self::Error> {
        Ok(Self {
            footer_text: v.footer_text,
            header_text: v.header_text,
        })
    }
}

impl TryFrom<super::super::types::BannerEntity> for crate::v2_7_2::types::BannerEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BannerEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            banners: v
                .banners
                .map(crate::v2_7_2::types::BannerDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::BatchSettingsDto> for crate::v2_7_2::types::BatchSettingsDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BatchSettingsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        })
    }
}

impl TryFrom<super::super::types::BatchSize> for crate::v2_7_2::types::BatchSize {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BatchSize) -> Result<Self, Self::Error> {
        Ok(Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        })
    }
}

impl TryFrom<super::super::types::BuildInfo> for crate::v2_7_2::types::BuildInfo {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BuildInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            compiler: v.compiler,
            compiler_flags: v.compiler_flags,
            revision: v.revision,
            target_arch: v.target_arch,
            timestamp: v.timestamp,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::BulletinBoardDto> for crate::v2_7_2::types::BulletinBoardDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BulletinBoardDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            generated: v.generated,
        })
    }
}

impl TryFrom<super::super::types::BulletinBoardEntity>
    for crate::v2_7_2::types::BulletinBoardEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BulletinBoardEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletin_board: v
                .bulletin_board
                .map(crate::v2_7_2::types::BulletinBoardDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::BulletinBoardPatternParameter>
    for crate::v2_7_2::types::BulletinBoardPatternParameter
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::BulletinBoardPatternParameter,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            pattern: v.pattern,
            raw_pattern: v.raw_pattern,
        })
    }
}

impl TryFrom<super::super::types::BulletinDto> for crate::v2_7_2::types::BulletinDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BulletinDto) -> Result<Self, Self::Error> {
        Ok(Self {
            category: v.category,
            group_id: v.group_id,
            id: v.id,
            level: v.level,
            message: v.message,
            node_address: v.node_address,
            source_id: v.source_id,
            source_name: v.source_name,
            source_type: v.source_type,
            stack_trace: v.stack_trace,
            timestamp: v.timestamp,
            timestamp_iso: v.timestamp_iso,
        })
    }
}

impl TryFrom<super::super::types::BulletinEntity> for crate::v2_7_2::types::BulletinEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BulletinEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletin: v.bulletin.map(|v| v.try_into()).transpose()?,
            can_read: v.can_read,
            group_id: v.group_id,
            id: v.id,
            node_address: v.node_address,
            source_id: v.source_id,
            timestamp: v.timestamp,
            timestamp_iso: v.timestamp_iso,
        })
    }
}

impl TryFrom<super::super::types::Bundle> for crate::v2_7_2::types::Bundle {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Bundle) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::BundleDto> for crate::v2_7_2::types::BundleDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::BundleDto) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::ClearBulletinsForGroupRequestEntity>
    for crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            components: v.components,
            from_timestamp: v.from_timestamp.ok_or_else(|| {
                crate::NifiError::MissingRequiredField {
                    field: "from_timestamp".to_string(),
                    type_name: "ClearBulletinsForGroupRequestEntity".to_string(),
                    version: "2.7.2".to_string(),
                }
            })?,
            id: v.id,
        })
    }
}

impl TryFrom<super::super::types::ClearBulletinsForGroupResultsEntity>
    for crate::v2_7_2::types::ClearBulletinsForGroupResultsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ClearBulletinsForGroupResultsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins_cleared: v.bulletins_cleared,
        })
    }
}

impl TryFrom<super::super::types::ClearBulletinsRequestEntity>
    for crate::v2_7_2::types::ClearBulletinsRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClearBulletinsRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            from_timestamp: v.from_timestamp.ok_or_else(|| {
                crate::NifiError::MissingRequiredField {
                    field: "from_timestamp".to_string(),
                    type_name: "ClearBulletinsRequestEntity".to_string(),
                    version: "2.7.2".to_string(),
                }
            })?,
        })
    }
}

impl TryFrom<super::super::types::ClearBulletinsResultEntity>
    for crate::v2_7_2::types::ClearBulletinsResultEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClearBulletinsResultEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            bulletins_cleared: v.bulletins_cleared,
            component_id: v.component_id,
        })
    }
}

impl TryFrom<super::super::types::ClientIdParameter> for crate::v2_7_2::types::ClientIdParameter {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClientIdParameter) -> Result<Self, Self::Error> {
        Ok(Self {
            client_id: v.client_id,
        })
    }
}

impl TryFrom<super::super::types::ClusterDto> for crate::v2_7_2::types::ClusterDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClusterDto) -> Result<Self, Self::Error> {
        Ok(Self {
            generated: v.generated,
            nodes: v
                .nodes
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ClusterEntity> for crate::v2_7_2::types::ClusterEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClusterEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster: v
                .cluster
                .map(crate::v2_7_2::types::ClusterDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ClusterSearchResultsEntity>
    for crate::v2_7_2::types::ClusterSearchResultsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClusterSearchResultsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            node_results: v
                .node_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ClusterSummaryDto> for crate::v2_7_2::types::ClusterSummaryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClusterSummaryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            clustered: v.clustered,
            connected_node_count: v.connected_node_count,
            connected_nodes: v.connected_nodes,
            connected_to_cluster: v.connected_to_cluster,
            total_node_count: v.total_node_count,
        })
    }
}

impl TryFrom<super::super::types::ClusterSummaryEntity>
    for crate::v2_7_2::types::ClusterSummaryEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ClusterSummaryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_summary: v
                .cluster_summary
                .map(crate::v2_7_2::types::ClusterSummaryDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentDetailsDto>
    for crate::v2_7_2::types::ComponentDetailsDto
{
    type Error = crate::NifiError;
    fn try_from(_v: super::super::types::ComponentDetailsDto) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<super::super::types::ComponentDifferenceDto>
    for crate::v2_7_2::types::ComponentDifferenceDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentDifferenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            differences: v
                .differences
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            process_group_id: v.process_group_id,
        })
    }
}

impl TryFrom<super::super::types::ComponentHistoryDto>
    for crate::v2_7_2::types::ComponentHistoryDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentHistoryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            component_id: v.component_id,
            property_history: v
                .property_history
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentHistoryEntity>
    for crate::v2_7_2::types::ComponentHistoryEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentHistoryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            component_history: v
                .component_history
                .map(crate::v2_7_2::types::ComponentHistoryDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentManifest> for crate::v2_7_2::types::ComponentManifest {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentManifest) -> Result<Self, Self::Error> {
        Ok(Self {
            apis: v
                .apis
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_registry_clients: v
                .flow_registry_clients
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_providers: v
                .parameter_providers
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentReferenceDto>
    for crate::v2_7_2::types::ComponentReferenceDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentReferenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ComponentReferenceEntity>
    for crate::v2_7_2::types::ComponentReferenceEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentReferenceEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            parent_group_id: v.parent_group_id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ComponentRestrictionPermissionDto>
    for crate::v2_7_2::types::ComponentRestrictionPermissionDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ComponentRestrictionPermissionDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            required_permission: v.required_permission.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentSearchResultDto>
    for crate::v2_7_2::types::ComponentSearchResultDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentSearchResultDto) -> Result<Self, Self::Error> {
        Ok(Self {
            group_id: v.group_id,
            id: v.id,
            matches: v.matches,
            name: v.name,
            parent_group: v.parent_group.map(|v| v.try_into()).transpose()?,
            versioned_group: v.versioned_group.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentStateDto> for crate::v2_7_2::types::ComponentStateDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentStateDto) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_state: v.cluster_state.map(|v| v.try_into()).transpose()?,
            component_id: v.component_id,
            drop_state_key_supported: v.drop_state_key_supported,
            local_state: v.local_state.map(|v| v.try_into()).transpose()?,
            state_description: v.state_description,
        })
    }
}

impl TryFrom<super::super::types::ComponentStateEntity>
    for crate::v2_7_2::types::ComponentStateEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentStateEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            component_state: v
                .component_state
                .map(crate::v2_7_2::types::ComponentStateDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ComponentValidationResultDto>
    for crate::v2_7_2::types::ComponentValidationResultDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ComponentValidationResultDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            currently_valid: v.currently_valid,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v
                .reference_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            resultant_validation_errors: v.resultant_validation_errors,
            results_valid: v.results_valid,
            state: v.state,
            validation_errors: v.validation_errors,
        })
    }
}

impl TryFrom<super::super::types::ComponentValidationResultEntity>
    for crate::v2_7_2::types::ComponentValidationResultEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ComponentValidationResultEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ComponentValidationResultsEntity>
    for crate::v2_7_2::types::ComponentValidationResultsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ComponentValidationResultsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            validation_results: v
                .validation_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ConfigVerificationResultDto>
    for crate::v2_7_2::types::ConfigVerificationResultDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConfigVerificationResultDto) -> Result<Self, Self::Error> {
        Ok(Self {
            explanation: v.explanation,
            outcome: v
                .outcome
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            verification_step_name: v.verification_step_name,
        })
    }
}

impl TryFrom<super::super::types::ConfigurationAnalysisDto>
    for crate::v2_7_2::types::ConfigurationAnalysisDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConfigurationAnalysisDto) -> Result<Self, Self::Error> {
        Ok(Self {
            component_id: v.component_id,
            properties: v.properties,
            referenced_attributes: v.referenced_attributes,
            supports_verification: v.supports_verification,
        })
    }
}

impl TryFrom<super::super::types::ConfigurationAnalysisEntity>
    for crate::v2_7_2::types::ConfigurationAnalysisEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConfigurationAnalysisEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            configuration_analysis: v
                .configuration_analysis
                .map(crate::v2_7_2::types::ConfigurationAnalysisDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ConnectableComponent>
    for crate::v2_7_2::types::ConnectableComponent
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectableComponent) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            group_id: v.group_id,
            id: v.id,
            instance_identifier: v.instance_identifier,
            name: v.name,
            r#type: v
                .r#type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ConnectableDto> for crate::v2_7_2::types::ConnectableDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectableDto) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            exists: v.exists,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            running: v.running,
            transmitting: v.transmitting,
            r#type: serde_json::from_value(serde_json::Value::String(v.r#type)).map_err(|e| {
                crate::NifiError::Api {
                    status: 0,
                    message: format!("enum parse: {}", e),
                }
            })?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ConnectionDto> for crate::v2_7_2::types::ConnectionDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            available_relationships: v.available_relationships,
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v
                .bends
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            destination: v.destination.map(|v| v.try_into()).transpose()?,
            flow_file_expiration: v.flow_file_expiration,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            load_balance_compression: v.load_balance_compression,
            load_balance_partition_attribute: v.load_balance_partition_attribute,
            load_balance_status: v.load_balance_status,
            load_balance_strategy: v.load_balance_strategy,
            name: v.name,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            prioritizers: v.prioritizers,
            retried_relationships: v.retried_relationships,
            selected_relationships: v.selected_relationships,
            source: v.source.map(|v| v.try_into()).transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ConnectionEntity> for crate::v2_7_2::types::ConnectionEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bends: v
                .bends
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            destination_group_id: v.destination_group_id,
            destination_id: v.destination_id,
            destination_type: serde_json::from_value(serde_json::Value::String(v.destination_type))
                .map_err(|e| crate::NifiError::Api {
                    status: 0,
                    message: format!("enum parse: {}", e),
                })?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            source_group_id: v.source_group_id,
            source_id: v.source_id,
            source_type: serde_json::from_value(serde_json::Value::String(v.source_type)).map_err(
                |e| crate::NifiError::Api {
                    status: 0,
                    message: format!("enum parse: {}", e),
                },
            )?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatisticsDto>
    for crate::v2_7_2::types::ConnectionStatisticsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionStatisticsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            id: v.id,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatisticsEntity>
    for crate::v2_7_2::types::ConnectionStatisticsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionStatisticsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            connection_statistics: v.connection_statistics.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatisticsSnapshotDto>
    for crate::v2_7_2::types::ConnectionStatisticsSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ConnectionStatisticsSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            predicted_bytes_at_next_interval: v.predicted_bytes_at_next_interval,
            predicted_count_at_next_interval: v.predicted_count_at_next_interval,
            predicted_millis_until_bytes_backpressure: v.predicted_millis_until_bytes_backpressure,
            predicted_millis_until_count_backpressure: v.predicted_millis_until_count_backpressure,
            predicted_percent_bytes: v.predicted_percent_bytes,
            predicted_percent_count: v.predicted_percent_count,
            prediction_interval_millis: v.prediction_interval_millis,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatusDto>
    for crate::v2_7_2::types::ConnectionStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            destination_id: v.destination_id,
            destination_name: v.destination_name,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            source_id: v.source_id,
            source_name: v.source_name,
            stats_last_refreshed: v.stats_last_refreshed,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatusEntity>
    for crate::v2_7_2::types::ConnectionStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            connection_status: v.connection_status.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatusPredictionsSnapshotDto>
    for crate::v2_7_2::types::ConnectionStatusPredictionsSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ConnectionStatusPredictionsSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            predicted_bytes_at_next_interval: v.predicted_bytes_at_next_interval,
            predicted_count_at_next_interval: v.predicted_count_at_next_interval,
            predicted_millis_until_bytes_backpressure: v.predicted_millis_until_bytes_backpressure,
            predicted_millis_until_count_backpressure: v.predicted_millis_until_count_backpressure,
            predicted_percent_bytes: v.predicted_percent_bytes,
            predicted_percent_count: v.predicted_percent_count,
            prediction_interval_seconds: v.prediction_interval_seconds,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatusSnapshotDto>
    for crate::v2_7_2::types::ConnectionStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionStatusSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_queued: v.bytes_queued,
            destination_id: v.destination_id,
            destination_name: v.destination_name,
            flow_file_availability: v.flow_file_availability,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            flow_files_queued: v.flow_files_queued,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            load_balance_status: v
                .load_balance_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            name: v.name,
            output: v.output,
            percent_use_bytes: v.percent_use_bytes,
            percent_use_count: v.percent_use_count,
            predictions: v.predictions.map(|v| v.try_into()).transpose()?,
            queued: v.queued,
            queued_count: v.queued_count,
            queued_size: v.queued_size,
            source_id: v.source_id,
            source_name: v.source_name,
        })
    }
}

impl TryFrom<super::super::types::ConnectionStatusSnapshotEntity>
    for crate::v2_7_2::types::ConnectionStatusSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ConnectionStatusSnapshotEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            connection_status_snapshot: v
                .connection_status_snapshot
                .map(|v| v.try_into())
                .transpose()?,
            id: v.id,
        })
    }
}

impl TryFrom<super::super::types::ConnectionsEntity> for crate::v2_7_2::types::ConnectionsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ConnectionsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ContentViewerDto> for crate::v2_7_2::types::ContentViewerDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ContentViewerDto) -> Result<Self, Self::Error> {
        Ok(Self {
            display_name: v.display_name,
            supported_mime_types: v
                .supported_mime_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ContentViewerEntity>
    for crate::v2_7_2::types::ContentViewerEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ContentViewerEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            content_viewers: v
                .content_viewers
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerBulletinsEntity>
    for crate::v2_7_2::types::ControllerBulletinsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerBulletinsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_service_bulletins: v
                .controller_service_bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_analysis_rule_bulletins: v
                .flow_analysis_rule_bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_registry_client_bulletins: v
                .flow_registry_client_bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_provider_bulletins: v
                .parameter_provider_bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reporting_task_bulletins: v
                .reporting_task_bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerConfigurationDto>
    for crate::v2_7_2::types::ControllerConfigurationDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerConfigurationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            max_timer_driven_thread_count: v.max_timer_driven_thread_count,
        })
    }
}

impl TryFrom<super::super::types::ControllerConfigurationEntity>
    for crate::v2_7_2::types::ControllerConfigurationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ControllerConfigurationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerDto> for crate::v2_7_2::types::ControllerDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            disabled_count: v.disabled_count,
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            instance_id: v.instance_id,
            invalid_count: v.invalid_count,
            name: v.name,
            output_port_count: v.output_port_count,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_site_http_listening_port: v.remote_site_http_listening_port,
            remote_site_listening_port: v.remote_site_listening_port,
            running_count: v.running_count,
            site_to_site_secure: v.site_to_site_secure,
            stopped_count: v.stopped_count,
        })
    }
}

impl TryFrom<super::super::types::ControllerEntity> for crate::v2_7_2::types::ControllerEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            controller: v
                .controller
                .map(crate::v2_7_2::types::ControllerDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceAPI>
    for crate::v2_7_2::types::ControllerServiceAPI
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceAPI) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceApiDto>
    for crate::v2_7_2::types::ControllerServiceApiDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceApiDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceDefinition>
    for crate::v2_7_2::types::ControllerServiceDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceDto>
    for crate::v2_7_2::types::ControllerServiceDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            controller_service_apis: v
                .controller_service_apis
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            custom_ui_url: v.custom_ui_url,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceEntity>
    for crate::v2_7_2::types::ControllerServiceEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            parent_group_id: v.parent_group_id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceReferencingComponentDto>
    for crate::v2_7_2::types::ControllerServiceReferencingComponentDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ControllerServiceReferencingComponentDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            properties: v.properties,
            reference_cycle: v.reference_cycle,
            reference_type: v
                .reference_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            state: v.state,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceReferencingComponentEntity>
    for crate::v2_7_2::types::ControllerServiceReferencingComponentEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ControllerServiceReferencingComponentEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceReferencingComponentsEntity>
    for crate::v2_7_2::types::ControllerServiceReferencingComponentsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ControllerServiceReferencingComponentsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_service_referencing_components: v
                .controller_service_referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceRunStatusEntity>
    for crate::v2_7_2::types::ControllerServiceRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ControllerServiceRunStatusEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            ui_only: v.ui_only,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceStatusDto>
    for crate::v2_7_2::types::ControllerServiceStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerServiceTypesEntity>
    for crate::v2_7_2::types::ControllerServiceTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServiceTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_service_types: v
                .controller_service_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ControllerServicesEntity>
    for crate::v2_7_2::types::ControllerServicesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerServicesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            current_time: v.current_time,
        })
    }
}

impl TryFrom<super::super::types::ControllerStatusDto>
    for crate::v2_7_2::types::ControllerStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_remote_port_count: v.active_remote_port_count,
            active_thread_count: v.active_thread_count,
            bytes_queued: v.bytes_queued,
            disabled_count: v.disabled_count,
            flow_files_queued: v.flow_files_queued,
            inactive_remote_port_count: v.inactive_remote_port_count,
            invalid_count: v.invalid_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            queued: v.queued,
            running_count: v.running_count,
            stale_count: v.stale_count,
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            terminated_thread_count: v.terminated_thread_count,
            up_to_date_count: v.up_to_date_count,
        })
    }
}

impl TryFrom<super::super::types::ControllerStatusEntity>
    for crate::v2_7_2::types::ControllerStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ControllerStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_status: v
                .controller_status
                .map(crate::v2_7_2::types::ControllerStatusDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::CopyRequestEntity> for crate::v2_7_2::types::CopyRequestEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CopyRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v.connections,
            funnels: v.funnels,
            input_ports: v.input_ports,
            labels: v.labels,
            output_ports: v.output_ports,
            process_groups: v.process_groups,
            processors: v.processors,
            remote_process_groups: v.remote_process_groups,
        })
    }
}

impl TryFrom<super::super::types::CopyResponseEntity> for crate::v2_7_2::types::CopyResponseEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CopyResponseEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            external_controller_service_references: v
                .external_controller_service_references
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            id: v.id,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            parameter_providers: v
                .parameter_providers
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::CopySnippetRequestEntity>
    for crate::v2_7_2::types::CopySnippetRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CopySnippetRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            origin_x: v.origin_x,
            origin_y: v.origin_y,
            snippet_id: v.snippet_id,
        })
    }
}

impl TryFrom<super::super::types::CounterDto> for crate::v2_7_2::types::CounterDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CounterDto) -> Result<Self, Self::Error> {
        Ok(Self {
            context: v.context,
            id: v.id,
            name: v.name,
            value: v.value,
            value_count: v.value_count,
        })
    }
}

impl TryFrom<super::super::types::CounterEntity> for crate::v2_7_2::types::CounterEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CounterEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            counter: v
                .counter
                .map(crate::v2_7_2::types::CounterDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::CountersDto> for crate::v2_7_2::types::CountersDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CountersDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::CountersEntity> for crate::v2_7_2::types::CountersEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CountersEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            counters: v
                .counters
                .map(crate::v2_7_2::types::CountersDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::CountersSnapshotDto>
    for crate::v2_7_2::types::CountersSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CountersSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            counters: v
                .counters
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            generated: v.generated,
        })
    }
}

impl TryFrom<super::super::types::CreateActiveRequestEntity>
    for crate::v2_7_2::types::CreateActiveRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CreateActiveRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_id: v.process_group_id,
        })
    }
}

impl TryFrom<super::super::types::CurrentUserEntity> for crate::v2_7_2::types::CurrentUserEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::CurrentUserEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            anonymous: v.anonymous,
            can_version_flows: v.can_version_flows,
            component_restriction_permissions: v
                .component_restriction_permissions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_permissions: v.controller_permissions.map(|v| v.try_into()).transpose()?,
            counters_permissions: v.counters_permissions.map(|v| v.try_into()).transpose()?,
            identity: v.identity,
            logout_supported: v.logout_supported,
            parameter_context_permissions: v
                .parameter_context_permissions
                .map(|v| v.try_into())
                .transpose()?,
            policies_permissions: v.policies_permissions.map(|v| v.try_into()).transpose()?,
            provenance_permissions: v.provenance_permissions.map(|v| v.try_into()).transpose()?,
            restricted_components_permissions: v
                .restricted_components_permissions
                .map(|v| v.try_into())
                .transpose()?,
            system_permissions: v.system_permissions.map(|v| v.try_into()).transpose()?,
            tenants_permissions: v.tenants_permissions.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::DateTimeParameter> for crate::v2_7_2::types::DateTimeParameter {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DateTimeParameter) -> Result<Self, Self::Error> {
        Ok(Self {
            date_time: v.date_time,
        })
    }
}

impl TryFrom<super::super::types::DefinedType> for crate::v2_7_2::types::DefinedType {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DefinedType) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact: v.artifact,
            group: v.group,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::DifferenceDto> for crate::v2_7_2::types::DifferenceDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DifferenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            difference: v.difference,
            difference_type: v.difference_type,
        })
    }
}

impl TryFrom<super::super::types::DimensionsDto> for crate::v2_7_2::types::DimensionsDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DimensionsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            height: v.height,
            width: v.width,
        })
    }
}

impl TryFrom<super::super::types::DocumentedTypeDto> for crate::v2_7_2::types::DocumentedTypeDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DocumentedTypeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            controller_service_apis: v
                .controller_service_apis
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            deprecation_reason: v.deprecation_reason,
            description: v.description,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            tags: v.tags,
            r#type: v.r#type,
            usage_restriction: v.usage_restriction,
        })
    }
}

impl TryFrom<super::super::types::DropRequestDto> for crate::v2_7_2::types::DropRequestDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DropRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            current: v.current,
            current_count: v.current_count,
            current_size: v.current_size,
            dropped: v.dropped,
            dropped_count: v.dropped_count,
            dropped_size: v.dropped_size,
            failure_reason: v.failure_reason,
            finished: v.finished,
            id: v.id,
            last_updated: v.last_updated,
            original: v.original,
            original_count: v.original_count,
            original_size: v.original_size,
            percent_completed: v.percent_completed,
            state: v.state,
            submission_time: v.submission_time,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::DropRequestEntity> for crate::v2_7_2::types::DropRequestEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DropRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            drop_request: v
                .drop_request
                .map(crate::v2_7_2::types::DropRequestDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::DynamicProperty> for crate::v2_7_2::types::DynamicProperty {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DynamicProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            expression_language_scope: v
                .expression_language_scope
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            name: v.name,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::DynamicRelationship>
    for crate::v2_7_2::types::DynamicRelationship
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::DynamicRelationship) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::ExplicitRestrictionDto>
    for crate::v2_7_2::types::ExplicitRestrictionDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ExplicitRestrictionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            explanation: v.explanation,
            required_permission: v.required_permission.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ExternalControllerServiceReference>
    for crate::v2_7_2::types::ExternalControllerServiceReference
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ExternalControllerServiceReference,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: v.identifier,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisResultEntity>
    for crate::v2_7_2::types::FlowAnalysisResultEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisResultEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_analysis_pending: v.flow_analysis_pending,
            rule_violations: v
                .rule_violations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            rules: v
                .rules
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleDefinition>
    for crate::v2_7_2::types::FlowAnalysisRuleDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleDto>
    for crate::v2_7_2::types::FlowAnalysisRuleDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            enforcement_policy: v.enforcement_policy,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleEntity>
    for crate::v2_7_2::types::FlowAnalysisRuleEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleRunStatusEntity>
    for crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleStatusDto>
    for crate::v2_7_2::types::FlowAnalysisRuleStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleTypesEntity>
    for crate::v2_7_2::types::FlowAnalysisRuleTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_analysis_rule_types: v
                .flow_analysis_rule_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRuleViolationDto>
    for crate::v2_7_2::types::FlowAnalysisRuleViolationDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRuleViolationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: v.enabled,
            enforcement_policy: v.enforcement_policy,
            group_id: v.group_id,
            issue_id: v.issue_id,
            rule_id: v.rule_id,
            scope: v.scope,
            subject_component_type: v.subject_component_type,
            subject_display_name: v.subject_display_name,
            subject_id: v.subject_id,
            subject_permission_dto: v.subject_permission_dto.map(|v| v.try_into()).transpose()?,
            violation_message: v.violation_message,
        })
    }
}

impl TryFrom<super::super::types::FlowAnalysisRulesEntity>
    for crate::v2_7_2::types::FlowAnalysisRulesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowAnalysisRulesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowBreadcrumbDto> for crate::v2_7_2::types::FlowBreadcrumbDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowBreadcrumbDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
            version_control_information: v
                .version_control_information
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowBreadcrumbEntity>
    for crate::v2_7_2::types::FlowBreadcrumbEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowBreadcrumbEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            breadcrumb: v.breadcrumb.map(|v| v.try_into()).transpose()?,
            id: v.id,
            parent_breadcrumb: v
                .parent_breadcrumb
                .map(|v| (*v).try_into().map(Box::new))
                .transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            versioned_flow_state: v
                .versioned_flow_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowComparisonEntity>
    for crate::v2_7_2::types::FlowComparisonEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowComparisonEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            component_differences: v
                .component_differences
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowConfigurationDto>
    for crate::v2_7_2::types::FlowConfigurationDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowConfigurationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            supports_configurable_authorizer: v.supports_configurable_authorizer,
            supports_configurable_users_and_groups: v.supports_configurable_users_and_groups,
            supports_managed_authorizer: v.supports_managed_authorizer,
            time_offset: v.time_offset,
        })
    }
}

impl TryFrom<super::super::types::FlowConfigurationEntity>
    for crate::v2_7_2::types::FlowConfigurationEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowConfigurationEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_configuration: v
                .flow_configuration
                .map(crate::v2_7_2::types::FlowConfigurationDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowDto> for crate::v2_7_2::types::FlowDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowDto) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowEntity> for crate::v2_7_2::types::FlowEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow: v
                .flow
                .map(crate::v2_7_2::types::FlowDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowFileDto> for crate::v2_7_2::types::FlowFileDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowFileDto) -> Result<Self, Self::Error> {
        Ok(Self {
            attributes: v.attributes,
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            content_claim_container: v.content_claim_container,
            content_claim_file_size: v.content_claim_file_size,
            content_claim_file_size_bytes: v.content_claim_file_size_bytes,
            content_claim_identifier: v.content_claim_identifier,
            content_claim_offset: v.content_claim_offset,
            content_claim_section: v.content_claim_section,
            filename: v.filename,
            lineage_duration: v.lineage_duration,
            mime_type: v.mime_type,
            penalized: v.penalized,
            penalty_expires_in: v.penalty_expires_in,
            position: v.position,
            queued_duration: v.queued_duration,
            size: v.size,
            uri: v.uri,
            uuid: v.uuid,
        })
    }
}

impl TryFrom<super::super::types::FlowFileEntity> for crate::v2_7_2::types::FlowFileEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowFileEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_file: v
                .flow_file
                .map(crate::v2_7_2::types::FlowFileDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowFileSummaryDto> for crate::v2_7_2::types::FlowFileSummaryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowFileSummaryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            filename: v.filename,
            lineage_duration: v.lineage_duration,
            mime_type: v.mime_type,
            penalized: v.penalized,
            penalty_expires_in: v.penalty_expires_in,
            position: v.position,
            queued_duration: v.queued_duration,
            size: v.size,
            uri: v.uri,
            uuid: v.uuid,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBranchDto>
    for crate::v2_7_2::types::FlowRegistryBranchDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBranchDto) -> Result<Self, Self::Error> {
        Ok(Self { name: v.name })
    }
}

impl TryFrom<super::super::types::FlowRegistryBranchEntity>
    for crate::v2_7_2::types::FlowRegistryBranchEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBranchEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            branch: v
                .branch
                .map(crate::v2_7_2::types::FlowRegistryBranchDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBranchesEntity>
    for crate::v2_7_2::types::FlowRegistryBranchesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBranchesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            branches: v
                .branches
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBucket> for crate::v2_7_2::types::FlowRegistryBucket {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBucket) -> Result<Self, Self::Error> {
        Ok(Self {
            created_timestamp: v.created_timestamp,
            description: v.description,
            identifier: v.identifier,
            name: v.name,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBucketDto>
    for crate::v2_7_2::types::FlowRegistryBucketDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBucketDto) -> Result<Self, Self::Error> {
        Ok(Self {
            created: v.created,
            description: v.description,
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBucketEntity>
    for crate::v2_7_2::types::FlowRegistryBucketEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBucketEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bucket: v.bucket.map(|v| v.try_into()).transpose()?,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryBucketsEntity>
    for crate::v2_7_2::types::FlowRegistryBucketsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryBucketsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            buckets: v
                .buckets
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryClientDefinition>
    for crate::v2_7_2::types::FlowRegistryClientDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryClientDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryClientDto>
    for crate::v2_7_2::types::FlowRegistryClientDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryClientDto) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            description: v.description,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            properties: v.properties,
            restricted: v.restricted,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            supports_branching: v.supports_branching,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryClientEntity>
    for crate::v2_7_2::types::FlowRegistryClientEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryClientEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryClientTypesEntity>
    for crate::v2_7_2::types::FlowRegistryClientTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::FlowRegistryClientTypesEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_registry_client_types: v
                .flow_registry_client_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryClientsEntity>
    for crate::v2_7_2::types::FlowRegistryClientsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryClientsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            registries: v
                .registries
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FlowRegistryPermissions>
    for crate::v2_7_2::types::FlowRegistryPermissions
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowRegistryPermissions) -> Result<Self, Self::Error> {
        Ok(Self {
            can_delete: v.can_delete,
            can_read: v.can_read,
            can_write: v.can_write,
        })
    }
}

impl TryFrom<super::super::types::FlowSnippetDto> for crate::v2_7_2::types::FlowSnippetDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FlowSnippetDto) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::FunnelDto> for crate::v2_7_2::types::FunnelDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FunnelDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::FunnelEntity> for crate::v2_7_2::types::FunnelEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FunnelEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::FunnelsEntity> for crate::v2_7_2::types::FunnelsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::FunnelsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::GarbageCollectionDto>
    for crate::v2_7_2::types::GarbageCollectionDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::GarbageCollectionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            collection_count: v.collection_count,
            collection_millis: v.collection_millis,
            collection_time: v.collection_time,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::HistoryDto> for crate::v2_7_2::types::HistoryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::HistoryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            actions: v
                .actions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            last_refreshed: v.last_refreshed,
            total: v.total,
        })
    }
}

impl TryFrom<super::super::types::HistoryEntity> for crate::v2_7_2::types::HistoryEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::HistoryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            history: v
                .history
                .map(crate::v2_7_2::types::HistoryDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::InputPortsEntity> for crate::v2_7_2::types::InputPortsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::InputPortsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::IntegerParameter> for crate::v2_7_2::types::IntegerParameter {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::IntegerParameter) -> Result<Self, Self::Error> {
        Ok(Self { integer: v.integer })
    }
}

impl TryFrom<super::super::types::JmxMetricsResultDto>
    for crate::v2_7_2::types::JmxMetricsResultDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::JmxMetricsResultDto) -> Result<Self, Self::Error> {
        Ok(Self {
            attribute_name: v.attribute_name,
            attribute_value: v.attribute_value,
            bean_name: v.bean_name,
        })
    }
}

impl TryFrom<super::super::types::JmxMetricsResultsEntity>
    for crate::v2_7_2::types::JmxMetricsResultsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::JmxMetricsResultsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            jmx_metrics_results: v
                .jmx_metrics_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LabelDto> for crate::v2_7_2::types::LabelDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LabelDto) -> Result<Self, Self::Error> {
        Ok(Self {
            getz_index: v.getz_index,
            height: v.height,
            id: v.id,
            label: v.label,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            style: v.style,
            versioned_component_id: v.versioned_component_id,
            width: v.width,
        })
    }
}

impl TryFrom<super::super::types::LabelEntity> for crate::v2_7_2::types::LabelEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LabelEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            dimensions: v.dimensions.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::LabelsEntity> for crate::v2_7_2::types::LabelsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LabelsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LatestProvenanceEventsDto>
    for crate::v2_7_2::types::LatestProvenanceEventsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LatestProvenanceEventsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            component_id: v.component_id,
            provenance_events: v
                .provenance_events
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LatestProvenanceEventsEntity>
    for crate::v2_7_2::types::LatestProvenanceEventsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LatestProvenanceEventsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            latest_provenance_events: v
                .latest_provenance_events
                .map(crate::v2_7_2::types::LatestProvenanceEventsDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LineageDto> for crate::v2_7_2::types::LineageDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LineageDto) -> Result<Self, Self::Error> {
        Ok(Self {
            expiration: v.expiration,
            finished: v.finished,
            id: v.id,
            percent_completed: v.percent_completed,
            request: v.request.map(|v| v.try_into()).transpose()?,
            results: v.results.map(|v| v.try_into()).transpose()?,
            submission_time: v.submission_time,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::LineageEntity> for crate::v2_7_2::types::LineageEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LineageEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            lineage: v
                .lineage
                .map(crate::v2_7_2::types::LineageDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LineageRequestDto> for crate::v2_7_2::types::LineageRequestDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LineageRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
            lineage_request_type: v
                .lineage_request_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            uuid: v.uuid,
        })
    }
}

impl TryFrom<super::super::types::LineageResultsDto> for crate::v2_7_2::types::LineageResultsDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LineageResultsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            errors: v.errors,
            links: v
                .links
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            nodes: v
                .nodes
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ListenPortDto> for crate::v2_7_2::types::ListenPortDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ListenPortDto) -> Result<Self, Self::Error> {
        Ok(Self {
            application_protocols: v.application_protocols,
            component_class: v.component_class,
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            parent_group_id: v.parent_group_id,
            parent_group_name: v.parent_group_name,
            port_name: v.port_name,
            port_number: v.port_number,
            transport_protocol: v.transport_protocol,
        })
    }
}

impl TryFrom<super::super::types::ListenPortsEntity> for crate::v2_7_2::types::ListenPortsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ListenPortsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            listen_ports: v
                .listen_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ListingRequestDto> for crate::v2_7_2::types::ListingRequestDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ListingRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            destination_running: v.destination_running,
            failure_reason: v.failure_reason,
            finished: v.finished,
            flow_file_summaries: v
                .flow_file_summaries
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            id: v.id,
            last_updated: v.last_updated,
            max_results: v.max_results,
            percent_completed: v.percent_completed,
            queue_size: v.queue_size.map(|v| v.try_into()).transpose()?,
            source_running: v.source_running,
            state: v.state,
            submission_time: v.submission_time,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ListingRequestEntity>
    for crate::v2_7_2::types::ListingRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ListingRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            listing_request: v
                .listing_request
                .map(crate::v2_7_2::types::ListingRequestDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::LongParameter> for crate::v2_7_2::types::LongParameter {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::LongParameter) -> Result<Self, Self::Error> {
        Ok(Self { long: v.long })
    }
}

impl TryFrom<super::super::types::MultiProcessorUseCase>
    for crate::v2_7_2::types::MultiProcessorUseCase
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::MultiProcessorUseCase) -> Result<Self, Self::Error> {
        Ok(Self {
            configurations: v
                .configurations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            description: v.description,
            keywords: v.keywords,
            notes: v.notes,
        })
    }
}

impl TryFrom<super::super::types::NarCoordinateDto> for crate::v2_7_2::types::NarCoordinateDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NarCoordinateDto) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::NarDetailsEntity> for crate::v2_7_2::types::NarDetailsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NarDetailsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_service_types: v
                .controller_service_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            dependent_coordinates: v
                .dependent_coordinates
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_analysis_rule_types: v
                .flow_analysis_rule_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_registry_client_types: v
                .flow_registry_client_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            nar_summary: v.nar_summary.map(|v| v.try_into()).transpose()?,
            parameter_provider_types: v
                .parameter_provider_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processor_types: v
                .processor_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reporting_task_types: v
                .reporting_task_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NarSummariesEntity> for crate::v2_7_2::types::NarSummariesEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NarSummariesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            nar_summaries: v
                .nar_summaries
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NarSummaryDto> for crate::v2_7_2::types::NarSummaryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NarSummaryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            build_time: v.build_time,
            coordinate: v.coordinate.map(|v| v.try_into()).transpose()?,
            created_by: v.created_by,
            dependency_coordinate: v.dependency_coordinate.map(|v| v.try_into()).transpose()?,
            digest: v.digest,
            extension_count: v.extension_count,
            failure_message: v.failure_message,
            identifier: v.identifier,
            install_complete: v.install_complete,
            source_identifier: v.source_identifier,
            source_type: v.source_type,
            state: v.state,
        })
    }
}

impl TryFrom<super::super::types::NarSummaryEntity> for crate::v2_7_2::types::NarSummaryEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NarSummaryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            nar_summary: v
                .nar_summary
                .map(crate::v2_7_2::types::NarSummaryDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeConnectionStatisticsSnapshotDto>
    for crate::v2_7_2::types::NodeConnectionStatisticsSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeConnectionStatisticsSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            statistics_snapshot: v.statistics_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeConnectionStatusSnapshotDto>
    for crate::v2_7_2::types::NodeConnectionStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeConnectionStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeCountersSnapshotDto>
    for crate::v2_7_2::types::NodeCountersSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeCountersSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeDto> for crate::v2_7_2::types::NodeDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            address: v.address,
            api_port: v.api_port,
            bytes_queued: v.bytes_queued,
            connection_requested: v.connection_requested,
            events: v
                .events
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_file_bytes: v.flow_file_bytes,
            flow_files_queued: v.flow_files_queued,
            heartbeat: v.heartbeat,
            node_id: v.node_id,
            node_start_time: v.node_start_time,
            queued: v.queued,
            roles: v.roles,
            status: v.status,
        })
    }
}

impl TryFrom<super::super::types::NodeEntity> for crate::v2_7_2::types::NodeEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            node: v
                .node
                .map(crate::v2_7_2::types::NodeDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeEventDto> for crate::v2_7_2::types::NodeEventDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeEventDto) -> Result<Self, Self::Error> {
        Ok(Self {
            category: v.category,
            message: v.message,
            timestamp: v.timestamp,
        })
    }
}

impl TryFrom<super::super::types::NodePortStatusSnapshotDto>
    for crate::v2_7_2::types::NodePortStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodePortStatusSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeProcessGroupStatusSnapshotDto>
    for crate::v2_7_2::types::NodeProcessGroupStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeProcessGroupStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeProcessorStatusSnapshotDto>
    for crate::v2_7_2::types::NodeProcessorStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeProcessorStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeRemoteProcessGroupStatusSnapshotDto>
    for crate::v2_7_2::types::NodeRemoteProcessGroupStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeRemoteProcessGroupStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeReplayLastEventSnapshotDto>
    for crate::v2_7_2::types::NodeReplayLastEventSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeReplayLastEventSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeSearchResultDto>
    for crate::v2_7_2::types::NodeSearchResultDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeSearchResultDto) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            id: v.id,
        })
    }
}

impl TryFrom<super::super::types::NodeStatusSnapshotsDto>
    for crate::v2_7_2::types::NodeStatusSnapshotsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::NodeStatusSnapshotsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshots: v
                .status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::NodeSystemDiagnosticsSnapshotDto>
    for crate::v2_7_2::types::NodeSystemDiagnosticsSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::NodeSystemDiagnosticsSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::OutputPortsEntity> for crate::v2_7_2::types::OutputPortsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::OutputPortsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextDto>
    for crate::v2_7_2::types::ParameterContextDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterContextDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bound_process_groups: v
                .bound_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            description: v.description,
            id: v.id,
            inherited_parameter_contexts: v
                .inherited_parameter_contexts
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            name: v.name,
            parameter_provider_configuration: v
                .parameter_provider_configuration
                .map(|v| v.try_into())
                .transpose()?,
            parameters: v
                .parameters
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextEntity>
    for crate::v2_7_2::types::ParameterContextEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterContextEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextReferenceDto>
    for crate::v2_7_2::types::ParameterContextReferenceDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterContextReferenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextReferenceEntity>
    for crate::v2_7_2::types::ParameterContextReferenceEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextReferenceEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            component: v.component.map(|v| v.try_into()).transpose()?,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextUpdateEntity>
    for crate::v2_7_2::types::ParameterContextUpdateEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterContextUpdateEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            parameter_context_revision: v
                .parameter_context_revision
                .map(|v| v.try_into())
                .transpose()?,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextUpdateRequestDto>
    for crate::v2_7_2::types::ParameterContextUpdateRequestDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextUpdateRequestDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            percent_completed: v.percent_completed,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextUpdateRequestEntity>
    for crate::v2_7_2::types::ParameterContextUpdateRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextUpdateRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter_context_revision: v
                .parameter_context_revision
                .map(|v| v.try_into())
                .transpose()?,
            request: v.request.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextUpdateStepDto>
    for crate::v2_7_2::types::ParameterContextUpdateStepDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextUpdateStepDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextValidationRequestDto>
    for crate::v2_7_2::types::ParameterContextValidationRequestDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextValidationRequestDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            component_validation_results: v
                .component_validation_results
                .map(|v| v.try_into())
                .transpose()?,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            percent_completed: v.percent_completed,
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextValidationRequestEntity>
    for crate::v2_7_2::types::ParameterContextValidationRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextValidationRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            request: v.request.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextValidationStepDto>
    for crate::v2_7_2::types::ParameterContextValidationStepDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterContextValidationStepDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        })
    }
}

impl TryFrom<super::super::types::ParameterContextsEntity>
    for crate::v2_7_2::types::ParameterContextsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterContextsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterDto> for crate::v2_7_2::types::ParameterDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterDto) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            inherited: v.inherited,
            name: v.name,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            provided: v.provided,
            referenced_assets: v
                .referenced_assets
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            sensitive: v.sensitive,
            value: v.value,
            value_removed: v.value_removed,
        })
    }
}

impl TryFrom<super::super::types::ParameterEntity> for crate::v2_7_2::types::ParameterEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_write: v.can_write,
            parameter: v.parameter.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterGroupConfigurationEntity>
    for crate::v2_7_2::types::ParameterGroupConfigurationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterGroupConfigurationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            group_name: v.group_name,
            parameter_context_name: v.parameter_context_name,
            parameter_sensitivities: v
                .parameter_sensitivities
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((
                                k,
                                v.map(|v| {
                                    serde_json::from_value::<_>(serde_json::Value::String(v))
                                        .map_err(|e| crate::NifiError::Api {
                                            status: 0,
                                            message: format!("enum parse: {}", e),
                                        })
                                })
                                .transpose()?,
                            ))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            synchronized: v.synchronized,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderApplyParametersRequestDto>
    for crate::v2_7_2::types::ParameterProviderApplyParametersRequestDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderApplyParametersRequestDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            parameter_context_updates: v
                .parameter_context_updates
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_provider: v.parameter_provider.map(|v| v.try_into()).transpose()?,
            percent_completed: v.percent_completed,
            referencing_components: v
                .referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            request_id: v.request_id,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderApplyParametersRequestEntity>
    for crate::v2_7_2::types::ParameterProviderApplyParametersRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderApplyParametersRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            request: v
                .request
                .map(crate::v2_7_2::types::ParameterProviderApplyParametersRequestDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderApplyParametersUpdateStepDto>
    for crate::v2_7_2::types::ParameterProviderApplyParametersUpdateStepDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderApplyParametersUpdateStepDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderConfigurationDto>
    for crate::v2_7_2::types::ParameterProviderConfigurationDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderConfigurationDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter_group_name: v.parameter_group_name,
            parameter_provider_id: v.parameter_provider_id,
            parameter_provider_name: v.parameter_provider_name,
            synchronized: v.synchronized,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderConfigurationEntity>
    for crate::v2_7_2::types::ParameterProviderConfigurationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderConfigurationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            component: v.component.map(|v| v.try_into()).transpose()?,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderDefinition>
    for crate::v2_7_2::types::ParameterProviderDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProviderDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderDto>
    for crate::v2_7_2::types::ParameterProviderDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProviderDto) -> Result<Self, Self::Error> {
        Ok(Self {
            affected_components: v
                .affected_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            custom_ui_url: v.custom_ui_url,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parameter_group_configurations: v
                .parameter_group_configurations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_status: v
                .parameter_status
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            referencing_parameter_contexts: v
                .referencing_parameter_contexts
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderEntity>
    for crate::v2_7_2::types::ParameterProviderEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProviderEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderParameterApplicationEntity>
    for crate::v2_7_2::types::ParameterProviderParameterApplicationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderParameterApplicationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            parameter_group_configurations: v
                .parameter_group_configurations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderParameterFetchEntity>
    for crate::v2_7_2::types::ParameterProviderParameterFetchEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderParameterFetchEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderReference>
    for crate::v2_7_2::types::ParameterProviderReference
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProviderReference) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            identifier: v.identifier,
            name: v.name,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderReferencingComponentDto>
    for crate::v2_7_2::types::ParameterProviderReferencingComponentDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderReferencingComponentDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderReferencingComponentEntity>
    for crate::v2_7_2::types::ParameterProviderReferencingComponentEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderReferencingComponentEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderReferencingComponentsEntity>
    for crate::v2_7_2::types::ParameterProviderReferencingComponentsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ParameterProviderReferencingComponentsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter_provider_referencing_components: v
                .parameter_provider_referencing_components
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProviderTypesEntity>
    for crate::v2_7_2::types::ParameterProviderTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProviderTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter_provider_types: v
                .parameter_provider_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterProvidersEntity>
    for crate::v2_7_2::types::ParameterProvidersEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterProvidersEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            parameter_providers: v
                .parameter_providers
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ParameterStatusDto> for crate::v2_7_2::types::ParameterStatusDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ParameterStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            parameter: v.parameter.map(|v| v.try_into()).transpose()?,
            status: v
                .status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PasteRequestEntity> for crate::v2_7_2::types::PasteRequestEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PasteRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            copy_response: v.copy_response.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PasteResponseEntity>
    for crate::v2_7_2::types::PasteResponseEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PasteResponseEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow: v.flow.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PeerDto> for crate::v2_7_2::types::PeerDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PeerDto) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_file_count: v.flow_file_count,
            hostname: v.hostname,
            port: v.port,
            secure: v.secure,
        })
    }
}

impl TryFrom<super::super::types::PeersEntity> for crate::v2_7_2::types::PeersEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PeersEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            peers: v
                .peers
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PermissionsDto> for crate::v2_7_2::types::PermissionsDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PermissionsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            can_write: v.can_write,
        })
    }
}

impl TryFrom<super::super::types::PortDto> for crate::v2_7_2::types::PortDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortDto) -> Result<Self, Self::Error> {
        Ok(Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            port_function: v
                .port_function
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            transmitting: v.transmitting,
            r#type: v
                .r#type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::PortEntity> for crate::v2_7_2::types::PortEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            allow_remote_access: v.allow_remote_access,
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            port_type: v.port_type,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::PortRunStatusEntity>
    for crate::v2_7_2::types::PortRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortRunStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PortStatusDto> for crate::v2_7_2::types::PortStatusDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
            transmitting: v.transmitting,
        })
    }
}

impl TryFrom<super::super::types::PortStatusEntity> for crate::v2_7_2::types::PortStatusEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            port_status: v.port_status.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PortStatusSnapshotDto>
    for crate::v2_7_2::types::PortStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortStatusSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            transmitting: v.transmitting,
        })
    }
}

impl TryFrom<super::super::types::PortStatusSnapshotEntity>
    for crate::v2_7_2::types::PortStatusSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PortStatusSnapshotEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            id: v.id,
            port_status_snapshot: v.port_status_snapshot.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::Position> for crate::v2_7_2::types::Position {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Position) -> Result<Self, Self::Error> {
        Ok(Self { x: v.x, y: v.y })
    }
}

impl TryFrom<super::super::types::PositionDto> for crate::v2_7_2::types::PositionDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PositionDto) -> Result<Self, Self::Error> {
        Ok(Self { x: v.x, y: v.y })
    }
}

impl TryFrom<super::super::types::PreviousValueDto> for crate::v2_7_2::types::PreviousValueDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PreviousValueDto) -> Result<Self, Self::Error> {
        Ok(Self {
            previous_value: v.previous_value,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        })
    }
}

impl TryFrom<super::super::types::PrioritizerTypesEntity>
    for crate::v2_7_2::types::PrioritizerTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PrioritizerTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            prioritizer_types: v
                .prioritizer_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupDto> for crate::v2_7_2::types::ProcessGroupDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            contents: v.contents.map(|v| v.try_into()).transpose()?,
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            disabled_count: v.disabled_count,
            execution_engine: v
                .execution_engine
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            flowfile_concurrency: v
                .flowfile_concurrency
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            flowfile_outbound_policy: v
                .flowfile_outbound_policy
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            invalid_count: v.invalid_count,
            local_input_port_count: v.local_input_port_count,
            local_output_port_count: v.local_output_port_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            log_file_suffix: v.log_file_suffix,
            max_concurrent_tasks: v.max_concurrent_tasks,
            name: v.name,
            output_port_count: v.output_port_count,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            public_input_port_count: v.public_input_port_count,
            public_output_port_count: v.public_output_port_count,
            running_count: v.running_count,
            stale_count: v.stale_count,
            stateless_flow_timeout: v.stateless_flow_timeout,
            stateless_group_scheduled_state: v
                .stateless_group_scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            version_control_information: v
                .version_control_information
                .map(|v| v.try_into())
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupEntity> for crate::v2_7_2::types::ProcessGroupEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            active_remote_port_count: v.active_remote_port_count,
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disabled_count: v.disabled_count,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            inactive_remote_port_count: v.inactive_remote_port_count,
            input_port_count: v.input_port_count,
            invalid_count: v.invalid_count,
            local_input_port_count: v.local_input_port_count,
            local_output_port_count: v.local_output_port_count,
            locally_modified_and_stale_count: v.locally_modified_and_stale_count,
            locally_modified_count: v.locally_modified_count,
            output_port_count: v.output_port_count,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            process_group_update_strategy: v
                .process_group_update_strategy
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            public_input_port_count: v.public_input_port_count,
            public_output_port_count: v.public_output_port_count,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            running_count: v.running_count,
            stale_count: v.stale_count,
            status: v.status.map(|v| v.try_into()).transpose()?,
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            uri: v.uri,
            versioned_flow_snapshot: v
                .versioned_flow_snapshot
                .map(|v| v.try_into())
                .transpose()?,
            versioned_flow_state: v
                .versioned_flow_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupFlowDto>
    for crate::v2_7_2::types::ProcessGroupFlowDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupFlowDto) -> Result<Self, Self::Error> {
        Ok(Self {
            breadcrumb: v.breadcrumb.map(|v| v.try_into()).transpose()?,
            flow: v.flow.map(|v| v.try_into()).transpose()?,
            id: v.id,
            last_refreshed: v.last_refreshed,
            parameter_context: v.parameter_context.map(|v| v.try_into()).transpose()?,
            parent_group_id: v.parent_group_id,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupFlowEntity>
    for crate::v2_7_2::types::ProcessGroupFlowEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupFlowEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            process_group_flow: v.process_group_flow.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupImportEntity>
    for crate::v2_7_2::types::ProcessGroupImportEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupImportEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            versioned_flow_snapshot: v
                .versioned_flow_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupNameDto>
    for crate::v2_7_2::types::ProcessGroupNameDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupNameDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupReplaceRequestDto>
    for crate::v2_7_2::types::ProcessGroupReplaceRequestDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessGroupReplaceRequestDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            process_group_id: v.process_group_id,
            request_id: v.request_id,
            state: v.state,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupReplaceRequestEntity>
    for crate::v2_7_2::types::ProcessGroupReplaceRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessGroupReplaceRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            request: v.request.map(|v| v.try_into()).transpose()?,
            versioned_flow_snapshot: v
                .versioned_flow_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupStatusDto>
    for crate::v2_7_2::types::ProcessGroupStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupStatusEntity>
    for crate::v2_7_2::types::ProcessGroupStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            process_group_status: v.process_group_status.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupStatusSnapshotDto>
    for crate::v2_7_2::types::ProcessGroupStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessGroupStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_queued: v.bytes_queued,
            bytes_read: v.bytes_read,
            bytes_received: v.bytes_received,
            bytes_sent: v.bytes_sent,
            bytes_transferred: v.bytes_transferred,
            bytes_written: v.bytes_written,
            connection_status_snapshots: v
                .connection_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            flow_files_queued: v.flow_files_queued,
            flow_files_received: v.flow_files_received,
            flow_files_sent: v.flow_files_sent,
            flow_files_transferred: v.flow_files_transferred,
            id: v.id,
            input: v.input,
            input_port_status_snapshots: v
                .input_port_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            name: v.name,
            output: v.output,
            output_port_status_snapshots: v
                .output_port_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            process_group_status_snapshots: v
                .process_group_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processing_nanos: v.processing_nanos,
            processing_performance_status: v
                .processing_performance_status
                .map(|v| v.try_into())
                .transpose()?,
            processor_status_snapshots: v
                .processor_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            queued: v.queued,
            queued_count: v.queued_count,
            queued_size: v.queued_size,
            read: v.read,
            received: v.received,
            remote_process_group_status_snapshots: v
                .remote_process_group_status_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            sent: v.sent,
            stateless_active_thread_count: v.stateless_active_thread_count,
            terminated_thread_count: v.terminated_thread_count,
            transferred: v.transferred,
            versioned_flow_state: v
                .versioned_flow_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            written: v.written,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupStatusSnapshotEntity>
    for crate::v2_7_2::types::ProcessGroupStatusSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessGroupStatusSnapshotEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            id: v.id,
            process_group_status_snapshot: v
                .process_group_status_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupUploadEntity>
    for crate::v2_7_2::types::ProcessGroupUploadEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupUploadEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            flow_snapshot: v.flow_snapshot.map(|v| v.try_into()).transpose()?,
            group_id: v.group_id,
            group_name: v.group_name,
            position_d_t_o: v.position_d_t_o.map(|v| v.try_into()).transpose()?,
            revision_d_t_o: v.revision_d_t_o.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessGroupsEntity>
    for crate::v2_7_2::types::ProcessGroupsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessGroupsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessingPerformanceStatusDto>
    for crate::v2_7_2::types::ProcessingPerformanceStatusDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessingPerformanceStatusDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            content_read_duration: v.content_read_duration,
            content_write_duration: v.content_write_duration,
            cpu_duration: v.cpu_duration,
            garbage_collection_duration: v.garbage_collection_duration,
            identifier: v.identifier,
            session_commit_duration: v.session_commit_duration,
        })
    }
}

impl TryFrom<super::super::types::ProcessorConfigDto> for crate::v2_7_2::types::ProcessorConfigDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorConfigDto) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            custom_ui_url: v.custom_ui_url,
            default_concurrent_tasks: v.default_concurrent_tasks,
            default_scheduling_period: v.default_scheduling_period,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            execution_node: v.execution_node,
            loss_tolerant: v.loss_tolerant,
            max_backoff_period: v.max_backoff_period,
            penalty_duration: v.penalty_duration,
            properties: v.properties,
            retried_relationships: v.retried_relationships,
            retry_count: v.retry_count,
            run_duration_millis: v.run_duration_millis,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            yield_duration: v.yield_duration,
        })
    }
}

impl TryFrom<super::super::types::ProcessorConfiguration>
    for crate::v2_7_2::types::ProcessorConfiguration
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            configuration: v.configuration,
            processor_class_name: v.processor_class_name,
        })
    }
}

impl TryFrom<super::super::types::ProcessorDefinition>
    for crate::v2_7_2::types::ProcessorDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            default_bulletin_level: v.default_bulletin_level,
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_penalty_duration: v.default_penalty_duration,
            default_scheduling_period_by_scheduling_strategy: v
                .default_scheduling_period_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy,
            default_yield_duration: v.default_yield_duration,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            dynamic_relationship: v.dynamic_relationship.map(|v| v.try_into()).transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            input_requirement: v
                .input_requirement
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            multi_processor_use_cases: v
                .multi_processor_use_cases
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            primary_node_only: v.primary_node_only,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reads_attributes: v
                .reads_attributes
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            side_effect_free: v.side_effect_free,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supported_relationships: v
                .supported_relationships
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            supported_scheduling_strategies: v.supported_scheduling_strategies,
            supports_batching: v.supports_batching,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_dynamic_relationships: v.supports_dynamic_relationships,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            trigger_serially: v.trigger_serially,
            trigger_when_any_destination_available: v.trigger_when_any_destination_available,
            trigger_when_empty: v.trigger_when_empty,
            r#type: v.r#type,
            type_description: v.type_description,
            use_cases: v
                .use_cases
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            version: v.version,
            writes_attributes: v
                .writes_attributes
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorDto> for crate::v2_7_2::types::ProcessorDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorDto) -> Result<Self, Self::Error> {
        Ok(Self {
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            config: v.config.map(|v| v.try_into()).transpose()?,
            deprecated: v.deprecated,
            description: v.description,
            execution_node_restricted: v.execution_node_restricted,
            extension_missing: v.extension_missing,
            id: v.id,
            input_requirement: v.input_requirement,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            physical_state: v
                .physical_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            relationships: v
                .relationships
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            style: v.style,
            supports_batching: v.supports_batching,
            supports_parallel_processing: v.supports_parallel_processing,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ProcessorEntity> for crate::v2_7_2::types::ProcessorEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            input_requirement: v.input_requirement,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            physical_state: v
                .physical_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ProcessorRunStatusDetailsDto>
    for crate::v2_7_2::types::ProcessorRunStatusDetailsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorRunStatusDetailsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            validation_errors: v.validation_errors,
        })
    }
}

impl TryFrom<super::super::types::ProcessorRunStatusDetailsEntity>
    for crate::v2_7_2::types::ProcessorRunStatusDetailsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessorRunStatusDetailsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            run_status_details: v.run_status_details.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorRunStatusEntity>
    for crate::v2_7_2::types::ProcessorRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorRunStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorStatusDto> for crate::v2_7_2::types::ProcessorStatusDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::ProcessorStatusEntity>
    for crate::v2_7_2::types::ProcessorStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            processor_status: v.processor_status.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorStatusSnapshotDto>
    for crate::v2_7_2::types::ProcessorStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorStatusSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_read: v.bytes_read,
            bytes_written: v.bytes_written,
            execution_node: v
                .execution_node
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            processing_performance_status: v
                .processing_performance_status
                .map(|v| v.try_into())
                .transpose()?,
            read: v.read,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            task_count: v.task_count,
            tasks: v.tasks,
            tasks_duration: v.tasks_duration,
            tasks_duration_nanos: v.tasks_duration_nanos,
            terminated_thread_count: v.terminated_thread_count,
            r#type: v.r#type,
            written: v.written,
        })
    }
}

impl TryFrom<super::super::types::ProcessorStatusSnapshotEntity>
    for crate::v2_7_2::types::ProcessorStatusSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessorStatusSnapshotEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            id: v.id,
            processor_status_snapshot: v
                .processor_status_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorTypesEntity>
    for crate::v2_7_2::types::ProcessorTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            processor_types: v
                .processor_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorsEntity> for crate::v2_7_2::types::ProcessorsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProcessorsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProcessorsRunStatusDetailsEntity>
    for crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ProcessorsRunStatusDetailsEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            run_status_details: v
                .run_status_details
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PropertyAllowableValue>
    for crate::v2_7_2::types::PropertyAllowableValue
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyAllowableValue) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::PropertyDependency> for crate::v2_7_2::types::PropertyDependency {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyDependency) -> Result<Self, Self::Error> {
        Ok(Self {
            dependent_values: v.dependent_values,
            property_display_name: v.property_display_name,
            property_name: v.property_name,
        })
    }
}

impl TryFrom<super::super::types::PropertyDependencyDto>
    for crate::v2_7_2::types::PropertyDependencyDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyDependencyDto) -> Result<Self, Self::Error> {
        Ok(Self {
            dependent_values: v.dependent_values,
            property_name: v.property_name,
        })
    }
}

impl TryFrom<super::super::types::PropertyDescriptor> for crate::v2_7_2::types::PropertyDescriptor {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyDescriptor) -> Result<Self, Self::Error> {
        Ok(Self {
            allowable_values: v
                .allowable_values
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            default_value: v.default_value,
            dependencies: v
                .dependencies
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            description: v.description,
            display_name: v.display_name,
            dynamic: v.dynamic,
            expression_language_scope: v
                .expression_language_scope
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            expression_language_scope_description: v.expression_language_scope_description,
            listen_port_definition: v.listen_port_definition.map(|v| v.try_into()).transpose()?,
            name: v.name,
            required: v.required,
            resource_definition: v.resource_definition.map(|v| v.try_into()).transpose()?,
            sensitive: v.sensitive,
            type_provided_by_value: v.type_provided_by_value.map(|v| v.try_into()).transpose()?,
            valid_regex: v.valid_regex,
            validator: v.validator,
        })
    }
}

impl TryFrom<super::super::types::PropertyDescriptorDto>
    for crate::v2_7_2::types::PropertyDescriptorDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyDescriptorDto) -> Result<Self, Self::Error> {
        Ok(Self {
            allowable_values: v
                .allowable_values
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            default_value: v.default_value,
            dependencies: v
                .dependencies
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            description: v.description,
            display_name: v.display_name,
            dynamic: v.dynamic,
            expression_language_scope: v.expression_language_scope,
            identifies_controller_service: v.identifies_controller_service,
            identifies_controller_service_bundle: v
                .identifies_controller_service_bundle
                .map(|v| v.try_into())
                .transpose()?,
            name: v.name,
            required: v.required,
            sensitive: v.sensitive,
            supports_el: v.supports_el,
        })
    }
}

impl TryFrom<super::super::types::PropertyDescriptorEntity>
    for crate::v2_7_2::types::PropertyDescriptorEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyDescriptorEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            property_descriptor: v
                .property_descriptor
                .map(crate::v2_7_2::types::PropertyDescriptorDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PropertyHistoryDto> for crate::v2_7_2::types::PropertyHistoryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyHistoryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            previous_values: v
                .previous_values
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PropertyListenPortDefinition>
    for crate::v2_7_2::types::PropertyListenPortDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyListenPortDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            application_protocols: v.application_protocols,
            transport_protocol: v
                .transport_protocol
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::PropertyResourceDefinition>
    for crate::v2_7_2::types::PropertyResourceDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::PropertyResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            cardinality: v
                .cardinality
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            resource_types: v
                .resource_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| {
                            serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                                crate::NifiError::Api {
                                    status: 0,
                                    message: format!("enum parse: {}", e),
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceDto> for crate::v2_7_2::types::ProvenanceDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            expiration: v.expiration,
            finished: v.finished,
            id: v.id,
            percent_completed: v.percent_completed,
            request: v.request.map(|v| v.try_into()).transpose()?,
            results: v.results.map(|v| v.try_into()).transpose()?,
            submission_time: v.submission_time,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceEntity> for crate::v2_7_2::types::ProvenanceEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            provenance: v
                .provenance
                .map(crate::v2_7_2::types::ProvenanceDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceEventDto> for crate::v2_7_2::types::ProvenanceEventDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceEventDto) -> Result<Self, Self::Error> {
        Ok(Self {
            alternate_identifier_uri: v.alternate_identifier_uri,
            attributes: v
                .attributes
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            child_uuids: v.child_uuids,
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            component_id: v.component_id,
            component_name: v.component_name,
            component_type: v.component_type,
            content_equal: v.content_equal,
            details: v.details,
            event_duration: v.event_duration,
            event_id: v.event_id,
            event_time: v.event_time,
            event_type: v.event_type,
            file_size: v.file_size,
            file_size_bytes: v.file_size_bytes,
            flow_file_uuid: v.flow_file_uuid,
            group_id: v.group_id,
            id: v.id,
            input_content_available: v.input_content_available,
            input_content_claim_container: v.input_content_claim_container,
            input_content_claim_file_size: v.input_content_claim_file_size,
            input_content_claim_file_size_bytes: v.input_content_claim_file_size_bytes,
            input_content_claim_identifier: v.input_content_claim_identifier,
            input_content_claim_offset: v.input_content_claim_offset,
            input_content_claim_section: v.input_content_claim_section,
            lineage_duration: v.lineage_duration,
            output_content_available: v.output_content_available,
            output_content_claim_container: v.output_content_claim_container,
            output_content_claim_file_size: v.output_content_claim_file_size,
            output_content_claim_file_size_bytes: v.output_content_claim_file_size_bytes,
            output_content_claim_identifier: v.output_content_claim_identifier,
            output_content_claim_offset: v.output_content_claim_offset,
            output_content_claim_section: v.output_content_claim_section,
            parent_uuids: v.parent_uuids,
            relationship: v.relationship,
            replay_available: v.replay_available,
            replay_explanation: v.replay_explanation,
            source_connection_identifier: v.source_connection_identifier,
            source_system_flow_file_id: v.source_system_flow_file_id,
            transit_uri: v.transit_uri,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceEventEntity>
    for crate::v2_7_2::types::ProvenanceEventEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceEventEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            provenance_event: v
                .provenance_event
                .map(crate::v2_7_2::types::ProvenanceEventDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceLinkDto> for crate::v2_7_2::types::ProvenanceLinkDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceLinkDto) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_file_uuid: v.flow_file_uuid,
            millis: v.millis,
            source_id: v.source_id,
            target_id: v.target_id,
            timestamp: v.timestamp,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceNodeDto> for crate::v2_7_2::types::ProvenanceNodeDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceNodeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            child_uuids: v.child_uuids,
            cluster_node_identifier: v.cluster_node_identifier,
            event_type: v.event_type,
            flow_file_uuid: v.flow_file_uuid,
            id: v.id,
            millis: v.millis,
            parent_uuids: v.parent_uuids,
            timestamp: v.timestamp,
            r#type: v
                .r#type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceOptionsDto>
    for crate::v2_7_2::types::ProvenanceOptionsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceOptionsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            searchable_fields: v
                .searchable_fields
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceOptionsEntity>
    for crate::v2_7_2::types::ProvenanceOptionsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceOptionsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            provenance_options: v
                .provenance_options
                .map(crate::v2_7_2::types::ProvenanceOptionsDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceRequestDto>
    for crate::v2_7_2::types::ProvenanceRequestDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_node_id: v.cluster_node_id,
            end_date: v.end_date,
            incremental_results: v.incremental_results,
            max_results: v.max_results,
            maximum_file_size: v.maximum_file_size,
            minimum_file_size: v.minimum_file_size,
            search_terms: v
                .search_terms
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            start_date: v.start_date,
            summarize: v.summarize,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceResultsDto>
    for crate::v2_7_2::types::ProvenanceResultsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceResultsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            errors: v.errors,
            generated: v.generated,
            oldest_event: v.oldest_event,
            provenance_events: v
                .provenance_events
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            time_offset: v.time_offset,
            total: v.total,
            total_count: v.total_count,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceSearchValueDto>
    for crate::v2_7_2::types::ProvenanceSearchValueDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceSearchValueDto) -> Result<Self, Self::Error> {
        Ok(Self {
            inverse: v.inverse,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::ProvenanceSearchableFieldDto>
    for crate::v2_7_2::types::ProvenanceSearchableFieldDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ProvenanceSearchableFieldDto) -> Result<Self, Self::Error> {
        Ok(Self {
            field: v.field,
            id: v.id,
            label: v.label,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::QueueSizeDto> for crate::v2_7_2::types::QueueSizeDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::QueueSizeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            byte_count: v.byte_count,
            object_count: v.object_count,
        })
    }
}

impl TryFrom<super::super::types::RegisteredFlow> for crate::v2_7_2::types::RegisteredFlow {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RegisteredFlow) -> Result<Self, Self::Error> {
        Ok(Self {
            branch: v.branch,
            bucket_identifier: v.bucket_identifier,
            bucket_name: v.bucket_name,
            created_timestamp: v.created_timestamp,
            description: v.description,
            identifier: v.identifier,
            last_modified_timestamp: v.last_modified_timestamp,
            name: v.name,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            version_count: v.version_count,
            version_info: v.version_info.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RegisteredFlowSnapshot>
    for crate::v2_7_2::types::RegisteredFlowSnapshot
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RegisteredFlowSnapshot) -> Result<Self, Self::Error> {
        Ok(Self {
            bucket: v.bucket.map(|v| v.try_into()).transpose()?,
            external_controller_services: v
                .external_controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            flow: v.flow.map(|v| v.try_into()).transpose()?,
            flow_contents: v.flow_contents.map(|v| v.try_into()).transpose()?,
            flow_encoding_version: v.flow_encoding_version,
            latest: v.latest,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            parameter_providers: v
                .parameter_providers
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            snapshot_metadata: v.snapshot_metadata.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RegisteredFlowSnapshotMetadata>
    for crate::v2_7_2::types::RegisteredFlowSnapshotMetadata
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RegisteredFlowSnapshotMetadata,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            author: v.author,
            branch: v.branch,
            bucket_identifier: v.bucket_identifier,
            comments: v.comments,
            flow_identifier: v.flow_identifier,
            flow_name: v.flow_name,
            registry_identifier: v.registry_identifier,
            registry_name: v.registry_name,
            timestamp: v.timestamp,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::RegisteredFlowVersionInfo>
    for crate::v2_7_2::types::RegisteredFlowVersionInfo
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RegisteredFlowVersionInfo) -> Result<Self, Self::Error> {
        Ok(Self { version: v.version })
    }
}

impl TryFrom<super::super::types::Relationship> for crate::v2_7_2::types::Relationship {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Relationship) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::RelationshipDto> for crate::v2_7_2::types::RelationshipDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RelationshipDto) -> Result<Self, Self::Error> {
        Ok(Self {
            auto_terminate: v.auto_terminate,
            description: v.description,
            name: v.name,
            retry: v.retry,
        })
    }
}

impl TryFrom<super::super::types::RemotePortRunStatusEntity>
    for crate::v2_7_2::types::RemotePortRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemotePortRunStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupContentsDto>
    for crate::v2_7_2::types::RemoteProcessGroupContentsDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RemoteProcessGroupContentsDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupDto>
    for crate::v2_7_2::types::RemoteProcessGroupDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_remote_input_port_count: v.active_remote_input_port_count,
            active_remote_output_port_count: v.active_remote_output_port_count,
            authorization_issues: v.authorization_issues,
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            contents: v.contents.map(|v| v.try_into()).transpose()?,
            flow_refreshed: v.flow_refreshed,
            id: v.id,
            inactive_remote_input_port_count: v.inactive_remote_input_port_count,
            inactive_remote_output_port_count: v.inactive_remote_output_port_count,
            input_port_count: v.input_port_count,
            local_network_interface: v.local_network_interface,
            name: v.name,
            output_port_count: v.output_port_count,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            proxy_host: v.proxy_host,
            proxy_password: v.proxy_password,
            proxy_port: v.proxy_port,
            proxy_user: v.proxy_user,
            target_secure: v.target_secure,
            target_uri: v.target_uri,
            target_uris: v.target_uris,
            transmitting: v.transmitting,
            transport_protocol: v.transport_protocol,
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
            yield_duration: v.yield_duration,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupEntity>
    for crate::v2_7_2::types::RemoteProcessGroupEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            input_port_count: v.input_port_count,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            output_port_count: v.output_port_count,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupPortDto>
    for crate::v2_7_2::types::RemoteProcessGroupPortDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupPortDto) -> Result<Self, Self::Error> {
        Ok(Self {
            batch_settings: v.batch_settings.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            connected: v.connected,
            exists: v.exists,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            target_id: v.target_id,
            target_running: v.target_running,
            transmitting: v.transmitting,
            use_compression: v.use_compression,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupPortEntity>
    for crate::v2_7_2::types::RemoteProcessGroupPortEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupPortEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            remote_process_group_port: v
                .remote_process_group_port
                .map(|v| v.try_into())
                .transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupStatusDto>
    for crate::v2_7_2::types::RemoteProcessGroupStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
            target_uri: v.target_uri,
            transmission_status: v.transmission_status,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupStatusEntity>
    for crate::v2_7_2::types::RemoteProcessGroupStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RemoteProcessGroupStatusEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            remote_process_group_status: v
                .remote_process_group_status
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupStatusSnapshotDto>
    for crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RemoteProcessGroupStatusSnapshotDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            bytes_received: v.bytes_received,
            bytes_sent: v.bytes_sent,
            flow_files_received: v.flow_files_received,
            flow_files_sent: v.flow_files_sent,
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            received: v.received,
            sent: v.sent,
            target_uri: v.target_uri,
            transmission_status: v.transmission_status,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupStatusSnapshotEntity>
    for crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RemoteProcessGroupStatusSnapshotEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            id: v.id,
            remote_process_group_status_snapshot: v
                .remote_process_group_status_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RemoteProcessGroupsEntity>
    for crate::v2_7_2::types::RemoteProcessGroupsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RemoteProcessGroupsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReplayLastEventRequestEntity>
    for crate::v2_7_2::types::ReplayLastEventRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReplayLastEventRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            component_id: v.component_id,
            nodes: v
                .nodes
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReplayLastEventResponseEntity>
    for crate::v2_7_2::types::ReplayLastEventResponseEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::ReplayLastEventResponseEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            component_id: v.component_id,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            nodes: v
                .nodes
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReplayLastEventSnapshotDto>
    for crate::v2_7_2::types::ReplayLastEventSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReplayLastEventSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            event_available: v.event_available,
            events_replayed: v.events_replayed,
            failure_explanation: v.failure_explanation,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskDefinition>
    for crate::v2_7_2::types::ReportingTaskDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            additional_details: v.additional_details,
            artifact: v.artifact,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            default_scheduling_period_by_scheduling_strategy: v
                .default_scheduling_period_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy,
            deprecated: v.deprecated,
            deprecation_alternatives: v.deprecation_alternatives,
            deprecation_reason: v.deprecation_reason,
            dynamic_properties: v
                .dynamic_properties
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            explicit_restrictions: v
                .explicit_restrictions
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group: v.group,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            provided_api_implementations: v
                .provided_api_implementations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            restricted: v.restricted,
            restricted_explanation: v.restricted_explanation,
            see_also: v.see_also,
            stateful: v.stateful.map(|v| v.try_into()).transpose()?,
            supported_scheduling_strategies: v.supported_scheduling_strategies,
            supports_dynamic_properties: v.supports_dynamic_properties,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            system_resource_considerations: v
                .system_resource_considerations
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            tags: v.tags,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskDto> for crate::v2_7_2::types::ReportingTaskDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            custom_ui_url: v.custom_ui_url,
            default_scheduling_period: v.default_scheduling_period,
            deprecated: v.deprecated,
            descriptors: v
                .descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            extension_missing: v.extension_missing,
            id: v.id,
            multiple_versions_available: v.multiple_versions_available,
            name: v.name,
            parent_group_id: v.parent_group_id,
            persists_state: v.persists_state,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            restricted: v.restricted,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            sensitive_dynamic_property_names: v.sensitive_dynamic_property_names,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskEntity>
    for crate::v2_7_2::types::ReportingTaskEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            operate_permissions: v.operate_permissions.map(|v| v.try_into()).transpose()?,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            status: v.status.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskRunStatusEntity>
    for crate::v2_7_2::types::ReportingTaskRunStatusEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskRunStatusEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskStatusDto>
    for crate::v2_7_2::types::ReportingTaskStatusDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskStatusDto) -> Result<Self, Self::Error> {
        Ok(Self {
            active_thread_count: v.active_thread_count,
            run_status: v
                .run_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            validation_status: v
                .validation_status
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReportingTaskTypesEntity>
    for crate::v2_7_2::types::ReportingTaskTypesEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTaskTypesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            reporting_task_types: v
                .reporting_task_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ReportingTasksEntity>
    for crate::v2_7_2::types::ReportingTasksEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ReportingTasksEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            current_time: v.current_time,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::RequiredPermissionDto>
    for crate::v2_7_2::types::RequiredPermissionDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RequiredPermissionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            label: v.label,
        })
    }
}

impl TryFrom<super::super::types::ResourceClaimDetailsDto>
    for crate::v2_7_2::types::ResourceClaimDetailsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ResourceClaimDetailsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            awaiting_destruction: v.awaiting_destruction,
            claimant_count: v.claimant_count,
            container: v.container,
            identifier: v.identifier,
            in_use: v.in_use,
            section: v.section,
            writable: v.writable,
        })
    }
}

impl TryFrom<super::super::types::ResourceDto> for crate::v2_7_2::types::ResourceDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ResourceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: v.identifier,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::ResourcesEntity> for crate::v2_7_2::types::ResourcesEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ResourcesEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            resources: v
                .resources
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::Restriction> for crate::v2_7_2::types::Restriction {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Restriction) -> Result<Self, Self::Error> {
        Ok(Self {
            explanation: v.explanation,
            required_permission: v.required_permission,
        })
    }
}

impl TryFrom<super::super::types::RevisionDto> for crate::v2_7_2::types::RevisionDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RevisionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            client_id: v.client_id,
            last_modifier: v.last_modifier,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::RunStatusDetailsRequestEntity>
    for crate::v2_7_2::types::RunStatusDetailsRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::RunStatusDetailsRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            processor_ids: v.processor_ids,
        })
    }
}

impl TryFrom<super::super::types::RuntimeManifest> for crate::v2_7_2::types::RuntimeManifest {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RuntimeManifest) -> Result<Self, Self::Error> {
        Ok(Self {
            agent_type: v.agent_type,
            build_info: v.build_info.map(|v| v.try_into()).transpose()?,
            bundles: v
                .bundles
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            identifier: v.identifier,
            scheduling_defaults: v.scheduling_defaults.map(|v| v.try_into()).transpose()?,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::RuntimeManifestEntity>
    for crate::v2_7_2::types::RuntimeManifestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::RuntimeManifestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            runtime_manifest: v
                .runtime_manifest
                .map(crate::v2_7_2::types::RuntimeManifest::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::ScheduleComponentsEntity>
    for crate::v2_7_2::types::ScheduleComponentsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::ScheduleComponentsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            components: v
                .components
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SchedulingDefaults> for crate::v2_7_2::types::SchedulingDefaults {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SchedulingDefaults) -> Result<Self, Self::Error> {
        Ok(Self {
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_max_concurrent_tasks: v.default_max_concurrent_tasks,
            default_run_duration_nanos: v.default_run_duration_nanos,
            default_scheduling_period_millis: v.default_scheduling_period_millis,
            default_scheduling_periods_by_scheduling_strategy: v
                .default_scheduling_periods_by_scheduling_strategy,
            default_scheduling_strategy: v
                .default_scheduling_strategy
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            penalization_period_millis: v.penalization_period_millis,
            yield_duration_millis: v.yield_duration_millis,
        })
    }
}

impl TryFrom<super::super::types::SearchResultGroupDto>
    for crate::v2_7_2::types::SearchResultGroupDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SearchResultGroupDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::SearchResultsDto> for crate::v2_7_2::types::SearchResultsDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SearchResultsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            connection_results: v
                .connection_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_service_node_results: v
                .controller_service_node_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            funnel_results: v
                .funnel_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            input_port_results: v
                .input_port_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            label_results: v
                .label_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            output_port_results: v
                .output_port_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_context_results: v
                .parameter_context_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_provider_node_results: v
                .parameter_provider_node_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_results: v
                .parameter_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            process_group_results: v
                .process_group_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processor_results: v
                .processor_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_process_group_results: v
                .remote_process_group_results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SearchResultsEntity>
    for crate::v2_7_2::types::SearchResultsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SearchResultsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            search_results_d_t_o: v
                .search_results_d_t_o
                .map(crate::v2_7_2::types::SearchResultsDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SnippetDto> for crate::v2_7_2::types::SnippetDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SnippetDto) -> Result<Self, Self::Error> {
        Ok(Self {
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            id: v.id,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            parent_group_id: v.parent_group_id,
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::SnippetEntity> for crate::v2_7_2::types::SnippetEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SnippetEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            snippet: v.snippet.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::StartVersionControlRequestEntity>
    for crate::v2_7_2::types::StartVersionControlRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::StartVersionControlRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            versioned_flow: v.versioned_flow.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::StateEntryDto> for crate::v2_7_2::types::StateEntryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StateEntryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            key: v.key,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::StateMapDto> for crate::v2_7_2::types::StateMapDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StateMapDto) -> Result<Self, Self::Error> {
        Ok(Self {
            scope: v.scope,
            state: v
                .state
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            total_entry_count: v.total_entry_count,
        })
    }
}

impl TryFrom<super::super::types::Stateful> for crate::v2_7_2::types::Stateful {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::Stateful) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            scopes: v
                .scopes
                .map(|v| {
                    v.into_iter()
                        .map(|v| {
                            serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                                crate::NifiError::Api {
                                    status: 0,
                                    message: format!("enum parse: {}", e),
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::StatusDescriptorDto>
    for crate::v2_7_2::types::StatusDescriptorDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StatusDescriptorDto) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            field: v.field,
            formatter: v.formatter,
            label: v.label,
        })
    }
}

impl TryFrom<super::super::types::StatusHistoryDto> for crate::v2_7_2::types::StatusHistoryDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StatusHistoryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshots: v
                .aggregate_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component_details: v.component_details,
            field_descriptors: v
                .field_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            generated: v.generated,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::StatusHistoryEntity>
    for crate::v2_7_2::types::StatusHistoryEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StatusHistoryEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            can_read: v.can_read,
            status_history: v.status_history.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::StatusSnapshotDto> for crate::v2_7_2::types::StatusSnapshotDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StatusSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            status_metrics: v.status_metrics,
            timestamp: v.timestamp,
        })
    }
}

impl TryFrom<super::super::types::StorageUsageDto> for crate::v2_7_2::types::StorageUsageDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::StorageUsageDto) -> Result<Self, Self::Error> {
        Ok(Self {
            free_space: v.free_space,
            free_space_bytes: v.free_space_bytes,
            identifier: v.identifier,
            total_space: v.total_space,
            total_space_bytes: v.total_space_bytes,
            used_space: v.used_space,
            used_space_bytes: v.used_space_bytes,
            utilization: v.utilization,
        })
    }
}

impl TryFrom<super::super::types::StreamingOutput> for crate::v2_7_2::types::StreamingOutput {
    type Error = crate::NifiError;
    fn try_from(_v: super::super::types::StreamingOutput) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<super::super::types::SubmitReplayRequestEntity>
    for crate::v2_7_2::types::SubmitReplayRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SubmitReplayRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
        })
    }
}

impl TryFrom<super::super::types::SupportedMimeTypesDto>
    for crate::v2_7_2::types::SupportedMimeTypesDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SupportedMimeTypesDto) -> Result<Self, Self::Error> {
        Ok(Self {
            display_name: v.display_name,
            mime_types: v.mime_types,
        })
    }
}

impl TryFrom<super::super::types::SystemDiagnosticsDto>
    for crate::v2_7_2::types::SystemDiagnosticsDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SystemDiagnosticsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            aggregate_snapshot: v.aggregate_snapshot.map(|v| v.try_into()).transpose()?,
            node_snapshots: v
                .node_snapshots
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SystemDiagnosticsEntity>
    for crate::v2_7_2::types::SystemDiagnosticsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SystemDiagnosticsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            system_diagnostics: v
                .system_diagnostics
                .map(crate::v2_7_2::types::SystemDiagnosticsDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SystemDiagnosticsSnapshotDto>
    for crate::v2_7_2::types::SystemDiagnosticsSnapshotDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SystemDiagnosticsSnapshotDto) -> Result<Self, Self::Error> {
        Ok(Self {
            available_processors: v.available_processors,
            content_repository_storage_usage: v
                .content_repository_storage_usage
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            daemon_threads: v.daemon_threads,
            flow_file_repository_storage_usage: v
                .flow_file_repository_storage_usage
                .map(|v| v.try_into())
                .transpose()?,
            free_heap: v.free_heap,
            free_heap_bytes: v.free_heap_bytes,
            free_non_heap: v.free_non_heap,
            free_non_heap_bytes: v.free_non_heap_bytes,
            garbage_collection: v
                .garbage_collection
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            heap_utilization: v.heap_utilization,
            max_heap: v.max_heap,
            max_heap_bytes: v.max_heap_bytes,
            max_non_heap: v.max_non_heap,
            max_non_heap_bytes: v.max_non_heap_bytes,
            non_heap_utilization: v.non_heap_utilization,
            processor_load_average: v.processor_load_average,
            provenance_repository_storage_usage: v
                .provenance_repository_storage_usage
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            resource_claim_details: v
                .resource_claim_details
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            stats_last_refreshed: v.stats_last_refreshed,
            total_heap: v.total_heap,
            total_heap_bytes: v.total_heap_bytes,
            total_non_heap: v.total_non_heap,
            total_non_heap_bytes: v.total_non_heap_bytes,
            total_threads: v.total_threads,
            uptime: v.uptime,
            used_heap: v.used_heap,
            used_heap_bytes: v.used_heap_bytes,
            used_non_heap: v.used_non_heap,
            used_non_heap_bytes: v.used_non_heap_bytes,
            version_info: v.version_info.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::SystemResourceConsideration>
    for crate::v2_7_2::types::SystemResourceConsideration
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::SystemResourceConsideration) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            resource: v.resource,
        })
    }
}

impl TryFrom<super::super::types::TenantDto> for crate::v2_7_2::types::TenantDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::TenantDto) -> Result<Self, Self::Error> {
        Ok(Self {
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::TenantEntity> for crate::v2_7_2::types::TenantEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::TenantEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::TenantsEntity> for crate::v2_7_2::types::TenantsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::TenantsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            user_groups: v
                .user_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            users: v
                .users
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::TransactionResultEntity>
    for crate::v2_7_2::types::TransactionResultEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::TransactionResultEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            flow_file_sent: v.flow_file_sent,
            message: v.message,
            response_code: v.response_code,
        })
    }
}

impl TryFrom<super::super::types::UpdateControllerServiceReferenceRequestEntity>
    for crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            referencing_component_revisions: v
                .referencing_component_revisions
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            ui_only: v.ui_only,
        })
    }
}

impl TryFrom<super::super::types::UseCase> for crate::v2_7_2::types::UseCase {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UseCase) -> Result<Self, Self::Error> {
        Ok(Self {
            configuration: v.configuration,
            description: v.description,
            input_requirement: v
                .input_requirement
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            keywords: v.keywords,
            notes: v.notes,
        })
    }
}

impl TryFrom<super::super::types::UserDto> for crate::v2_7_2::types::UserDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UserDto) -> Result<Self, Self::Error> {
        Ok(Self {
            access_policies: v
                .access_policies
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            user_groups: v
                .user_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::UserEntity> for crate::v2_7_2::types::UserEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UserEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::UserGroupDto> for crate::v2_7_2::types::UserGroupDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UserGroupDto) -> Result<Self, Self::Error> {
        Ok(Self {
            access_policies: v
                .access_policies
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            configurable: v.configurable,
            id: v.id,
            identity: v.identity,
            parent_group_id: v.parent_group_id,
            position: v.position.map(|v| v.try_into()).transpose()?,
            users: v
                .users
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            versioned_component_id: v.versioned_component_id,
        })
    }
}

impl TryFrom<super::super::types::UserGroupEntity> for crate::v2_7_2::types::UserGroupEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UserGroupEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            bulletins: v
                .bulletins
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            component: v.component.map(|v| v.try_into()).transpose()?,
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            permissions: v.permissions.map(|v| v.try_into()).transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            revision: v.revision.map(|v| v.try_into()).transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::UserGroupsEntity> for crate::v2_7_2::types::UserGroupsEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UserGroupsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            user_groups: v
                .user_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::UsersEntity> for crate::v2_7_2::types::UsersEntity {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::UsersEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            generated: v.generated,
            users: v
                .users
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VerifyConfigRequestDto>
    for crate::v2_7_2::types::VerifyConfigRequestDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VerifyConfigRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            attributes: v.attributes,
            complete: v.complete,
            component_id: v.component_id,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            properties: v.properties,
            request_id: v.request_id,
            results: v
                .results
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            state: v.state,
            submission_time: v.submission_time,
            update_steps: v
                .update_steps
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            uri: v.uri,
        })
    }
}

impl TryFrom<super::super::types::VerifyConfigRequestEntity>
    for crate::v2_7_2::types::VerifyConfigRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VerifyConfigRequestEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            request: v
                .request
                .map(crate::v2_7_2::types::VerifyConfigRequestDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VerifyConfigUpdateStepDto>
    for crate::v2_7_2::types::VerifyConfigUpdateStepDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VerifyConfigUpdateStepDto) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        })
    }
}

impl TryFrom<super::super::types::VersionControlComponentMappingEntity>
    for crate::v2_7_2::types::VersionControlComponentMappingEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionControlComponentMappingEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            version_control_component_mapping: v.version_control_component_mapping,
            version_control_information: v
                .version_control_information
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionControlInformationDto>
    for crate::v2_7_2::types::VersionControlInformationDto
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionControlInformationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            branch: v.branch,
            bucket_id: v.bucket_id,
            bucket_name: v.bucket_name,
            flow_description: v.flow_description,
            flow_id: v.flow_id,
            flow_name: v.flow_name,
            group_id: v.group_id,
            registry_id: v.registry_id,
            registry_name: v.registry_name,
            state: v
                .state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            state_explanation: v.state_explanation,
            storage_location: v.storage_location,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::VersionControlInformationEntity>
    for crate::v2_7_2::types::VersionControlInformationEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionControlInformationEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            version_control_information: v
                .version_control_information
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionInfoDto> for crate::v2_7_2::types::VersionInfoDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionInfoDto) -> Result<Self, Self::Error> {
        Ok(Self {
            build_branch: v.build_branch,
            build_revision: v.build_revision,
            build_tag: v.build_tag,
            build_timestamp: v.build_timestamp,
            java_vendor: v.java_vendor,
            java_version: v.java_version,
            ni_fi_version: v.ni_fi_version,
            os_architecture: v.os_architecture,
            os_name: v.os_name,
            os_version: v.os_version,
        })
    }
}

impl TryFrom<super::super::types::VersionedAsset> for crate::v2_7_2::types::VersionedAsset {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedAsset) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: v.identifier,
            name: v.name,
        })
    }
}

impl TryFrom<super::super::types::VersionedConnection>
    for crate::v2_7_2::types::VersionedConnection
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedConnection) -> Result<Self, Self::Error> {
        Ok(Self {
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v
                .bends
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            destination: v.destination.map(|v| v.try_into()).transpose()?,
            flow_file_expiration: v.flow_file_expiration,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            label_index: v.label_index,
            load_balance_compression: v.load_balance_compression,
            load_balance_strategy: v.load_balance_strategy,
            name: v.name,
            partitioning_attribute: v.partitioning_attribute,
            position: v.position.map(|v| v.try_into()).transpose()?,
            prioritizers: v.prioritizers,
            selected_relationships: v.selected_relationships,
            source: v.source.map(|v| v.try_into()).transpose()?,
            z_index: v.z_index,
        })
    }
}

impl TryFrom<super::super::types::VersionedControllerService>
    for crate::v2_7_2::types::VersionedControllerService
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedControllerService) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            controller_service_apis: v
                .controller_service_apis
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowCoordinates>
    for crate::v2_7_2::types::VersionedFlowCoordinates
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFlowCoordinates) -> Result<Self, Self::Error> {
        Ok(Self {
            branch: v.branch,
            bucket_id: v.bucket_id,
            flow_id: v.flow_id,
            latest: v.latest,
            registry_id: v.registry_id,
            storage_location: v.storage_location,
            version: v.version,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowDto> for crate::v2_7_2::types::VersionedFlowDto {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFlowDto) -> Result<Self, Self::Error> {
        Ok(Self {
            action: v
                .action
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            branch: v.branch,
            bucket_id: v.bucket_id,
            comments: v.comments,
            description: v.description,
            flow_id: v.flow_id,
            flow_name: v.flow_name,
            registry_id: v.registry_id,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowEntity>
    for crate::v2_7_2::types::VersionedFlowEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFlowEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            versioned_flow: v
                .versioned_flow
                .map(crate::v2_7_2::types::VersionedFlowDto::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowSnapshotEntity>
    for crate::v2_7_2::types::VersionedFlowSnapshotEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFlowSnapshotEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            registry_id: v.registry_id,
            update_descendant_versioned_flows: v.update_descendant_versioned_flows,
            versioned_flow: v.versioned_flow.map(|v| v.try_into()).transpose()?,
            versioned_flow_snapshot: v
                .versioned_flow_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowSnapshotMetadataEntity>
    for crate::v2_7_2::types::VersionedFlowSnapshotMetadataEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedFlowSnapshotMetadataEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            registry_id: v.registry_id,
            versioned_flow_snapshot_metadata: v
                .versioned_flow_snapshot_metadata
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowSnapshotMetadataSetEntity>
    for crate::v2_7_2::types::VersionedFlowSnapshotMetadataSetEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedFlowSnapshotMetadataSetEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            versioned_flow_snapshot_metadata_set: v
                .versioned_flow_snapshot_metadata_set
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowUpdateRequestDto>
    for crate::v2_7_2::types::VersionedFlowUpdateRequestDto
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedFlowUpdateRequestDto,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            complete: v.complete,
            failure_reason: v.failure_reason,
            last_updated: v.last_updated,
            percent_completed: v.percent_completed,
            process_group_id: v.process_group_id,
            request_id: v.request_id,
            state: v.state,
            uri: v.uri,
            version_control_information: v
                .version_control_information
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowUpdateRequestEntity>
    for crate::v2_7_2::types::VersionedFlowUpdateRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedFlowUpdateRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            process_group_revision: v.process_group_revision.map(|v| v.try_into()).transpose()?,
            request: v.request.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFlowsEntity>
    for crate::v2_7_2::types::VersionedFlowsEntity
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFlowsEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            versioned_flows: v
                .versioned_flows
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedFunnel> for crate::v2_7_2::types::VersionedFunnel {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedFunnel) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(|v| v.try_into()).transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedLabel> for crate::v2_7_2::types::VersionedLabel {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedLabel) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            height: v.height,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            label: v.label,
            name: v.name,
            position: v.position.map(|v| v.try_into()).transpose()?,
            style: v.style,
            width: v.width,
            z_index: v.z_index,
        })
    }
}

impl TryFrom<super::super::types::VersionedListenPortDefinition>
    for crate::v2_7_2::types::VersionedListenPortDefinition
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedListenPortDefinition,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            application_protocols: v.application_protocols,
            transport_protocol: v
                .transport_protocol
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedParameter> for crate::v2_7_2::types::VersionedParameter {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedParameter) -> Result<Self, Self::Error> {
        Ok(Self {
            description: v.description,
            name: v.name,
            provided: v.provided,
            referenced_assets: v
                .referenced_assets
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            sensitive: v.sensitive,
            value: v.value,
        })
    }
}

impl TryFrom<super::super::types::VersionedParameterContext>
    for crate::v2_7_2::types::VersionedParameterContext
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedParameterContext) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            description: v.description,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            inherited_parameter_contexts: v.inherited_parameter_contexts,
            instance_identifier: v.instance_identifier,
            name: v.name,
            parameter_group_name: v.parameter_group_name,
            parameter_provider: v.parameter_provider,
            parameters: v
                .parameters
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            synchronized: v.synchronized,
        })
    }
}

impl TryFrom<super::super::types::VersionedPort> for crate::v2_7_2::types::VersionedPort {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedPort) -> Result<Self, Self::Error> {
        Ok(Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            port_function: v
                .port_function
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            r#type: v
                .r#type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedProcessGroup>
    for crate::v2_7_2::types::VersionedProcessGroup
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedProcessGroup) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            connections: v
                .connections
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            execution_engine: v
                .execution_engine
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            flow_file_concurrency: v.flow_file_concurrency,
            flow_file_outbound_policy: v.flow_file_outbound_policy,
            funnels: v
                .funnels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            instance_identifier: v.instance_identifier,
            labels: v
                .labels
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            log_file_suffix: v.log_file_suffix,
            max_concurrent_tasks: v.max_concurrent_tasks,
            name: v.name,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            parameter_context_name: v.parameter_context_name,
            position: v.position.map(|v| v.try_into()).transpose()?,
            process_groups: v
                .process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| (*v).try_into().map(Box::new))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            processors: v
                .processors
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            remote_process_groups: v
                .remote_process_groups
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            stateless_flow_timeout: v.stateless_flow_timeout,
            versioned_flow_coordinates: v
                .versioned_flow_coordinates
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedProcessor> for crate::v2_7_2::types::VersionedProcessor {
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedProcessor) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            execution_node: v.execution_node,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            max_backoff_period: v.max_backoff_period,
            name: v.name,
            penalty_duration: v.penalty_duration,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            retried_relationships: v.retried_relationships,
            retry_count: v.retry_count,
            run_duration_millis: v.run_duration_millis,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            style: v.style,
            r#type: v.r#type,
            yield_duration: v.yield_duration,
        })
    }
}

impl TryFrom<super::super::types::VersionedPropertyDescriptor>
    for crate::v2_7_2::types::VersionedPropertyDescriptor
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedPropertyDescriptor) -> Result<Self, Self::Error> {
        Ok(Self {
            display_name: v.display_name,
            dynamic: v.dynamic,
            identifies_controller_service: v.identifies_controller_service,
            listen_port_definition: v.listen_port_definition.map(|v| v.try_into()).transpose()?,
            name: v.name,
            resource_definition: v.resource_definition.map(|v| v.try_into()).transpose()?,
            sensitive: v.sensitive,
        })
    }
}

impl TryFrom<super::super::types::VersionedRemoteGroupPort>
    for crate::v2_7_2::types::VersionedRemoteGroupPort
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedRemoteGroupPort) -> Result<Self, Self::Error> {
        Ok(Self {
            batch_size: v.batch_size.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(|v| v.try_into()).transpose()?,
            remote_group_id: v.remote_group_id,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            target_id: v.target_id,
            use_compression: v.use_compression,
        })
    }
}

impl TryFrom<super::super::types::VersionedRemoteProcessGroup>
    for crate::v2_7_2::types::VersionedRemoteProcessGroup
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedRemoteProcessGroup) -> Result<Self, Self::Error> {
        Ok(Self {
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            input_ports: v
                .input_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            instance_identifier: v.instance_identifier,
            local_network_interface: v.local_network_interface,
            name: v.name,
            output_ports: v
                .output_ports
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            position: v.position.map(|v| v.try_into()).transpose()?,
            proxy_host: v.proxy_host,
            proxy_password: v.proxy_password,
            proxy_port: v.proxy_port,
            proxy_user: v.proxy_user,
            target_uris: v.target_uris,
            transport_protocol: v.transport_protocol,
            yield_duration: v.yield_duration,
        })
    }
}

impl TryFrom<super::super::types::VersionedReportingTask>
    for crate::v2_7_2::types::VersionedReportingTask
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedReportingTask) -> Result<Self, Self::Error> {
        Ok(Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(|v| v.try_into()).transpose()?,
            comments: v.comments,
            component_type: v
                .component_type
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(|v| v.try_into()).transpose()?,
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|v| {
                    v.into_iter()
                        .map(|(k, v)| {
                            Ok::<_, crate::NifiError>((k, v.map(|v| v.try_into()).transpose()?))
                        })
                        .collect::<Result<std::collections::HashMap<_, _>, _>>()
                })
                .transpose()?,
            scheduled_state: v
                .scheduled_state
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            r#type: v.r#type,
        })
    }
}

impl TryFrom<super::super::types::VersionedReportingTaskImportRequestEntity>
    for crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            reporting_task_snapshot: v
                .reporting_task_snapshot
                .map(|v| v.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedReportingTaskImportResponseEntity>
    for crate::v2_7_2::types::VersionedReportingTaskImportResponseEntity
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedReportingTaskImportResponseEntity,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedReportingTaskSnapshot>
    for crate::v2_7_2::types::VersionedReportingTaskSnapshot
{
    type Error = crate::NifiError;
    fn try_from(
        v: super::super::types::VersionedReportingTaskSnapshot,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            controller_services: v
                .controller_services
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| {
                    v.into_iter()
                        .map(|v| v.try_into())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

impl TryFrom<super::super::types::VersionedResourceDefinition>
    for crate::v2_7_2::types::VersionedResourceDefinition
{
    type Error = crate::NifiError;
    fn try_from(v: super::super::types::VersionedResourceDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            cardinality: v
                .cardinality
                .map(|v| {
                    serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                        crate::NifiError::Api {
                            status: 0,
                            message: format!("enum parse: {}", e),
                        }
                    })
                })
                .transpose()?,
            resource_types: v
                .resource_types
                .map(|v| {
                    v.into_iter()
                        .map(|v| {
                            serde_json::from_value::<_>(serde_json::Value::String(v)).map_err(|e| {
                                crate::NifiError::Api {
                                    status: 0,
                                    message: format!("enum parse: {}", e),
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        })
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

impl From<crate::v2_7_2::types::AboutDto> for super::types::AboutDto {
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

impl From<crate::v2_7_2::types::AboutEntity> for super::types::AboutEntity {
    fn from(v: crate::v2_7_2::types::AboutEntity) -> Self {
        Self {
            about: Some(v.about.into()),
        }
    }
}

impl From<crate::v2_7_2::types::AccessPolicyDto> for super::types::AccessPolicyDto {
    fn from(v: crate::v2_7_2::types::AccessPolicyDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::AccessPolicyEntity> for super::types::AccessPolicyEntity {
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

impl From<crate::v2_7_2::types::AccessPolicySummaryDto> for super::types::AccessPolicySummaryDto {
    fn from(v: crate::v2_7_2::types::AccessPolicySummaryDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::AccessPolicySummaryEntity
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

impl From<crate::v2_7_2::types::ActionDetailsDto> for super::types::ActionDetailsDto {
    fn from(v: crate::v2_7_2::types::ActionDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::ActionDto> for super::types::ActionDto {
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

impl From<crate::v2_7_2::types::ActionEntity> for super::types::ActionEntity {
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
    for super::types::ActivateControllerServicesEntity
{
    fn from(v: crate::v2_7_2::types::ActivateControllerServicesEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::AdditionalDetailsEntity> for super::types::AdditionalDetailsEntity {
    fn from(v: crate::v2_7_2::types::AdditionalDetailsEntity) -> Self {
        Self {
            additional_details: v.additional_details,
        }
    }
}

impl From<crate::v2_7_2::types::AffectedComponentDto> for super::types::AffectedComponentDto {
    fn from(v: crate::v2_7_2::types::AffectedComponentDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::AffectedComponentEntity> for super::types::AffectedComponentEntity {
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
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::AllowableValueDto> for super::types::AllowableValueDto {
    fn from(v: crate::v2_7_2::types::AllowableValueDto) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::AllowableValueEntity> for super::types::AllowableValueEntity {
    fn from(v: crate::v2_7_2::types::AllowableValueEntity) -> Self {
        Self {
            allowable_value: v.allowable_value.map(Into::into),
            can_read: v.can_read,
        }
    }
}

impl From<crate::v2_7_2::types::AssetDto> for super::types::AssetDto {
    fn from(v: crate::v2_7_2::types::AssetDto) -> Self {
        Self {
            digest: v.digest,
            id: v.id,
            missing_content: v.missing_content,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AssetEntity> for super::types::AssetEntity {
    fn from(v: crate::v2_7_2::types::AssetEntity) -> Self {
        Self {
            asset: Some(v.asset.into()),
        }
    }
}

impl From<crate::v2_7_2::types::AssetReferenceDto> for super::types::AssetReferenceDto {
    fn from(v: crate::v2_7_2::types::AssetReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AssetsEntity> for super::types::AssetsEntity {
    fn from(v: crate::v2_7_2::types::AssetsEntity) -> Self {
        Self {
            assets: v.assets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::Attribute> for super::types::Attribute {
    fn from(v: crate::v2_7_2::types::Attribute) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::AttributeDto> for super::types::AttributeDto {
    fn from(v: crate::v2_7_2::types::AttributeDto) -> Self {
        Self {
            name: v.name,
            previous_value: v.previous_value,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::AuthenticationConfigurationDto>
    for super::types::AuthenticationConfigurationDto
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
    for super::types::AuthenticationConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::AuthenticationConfigurationEntity) -> Self {
        Self {
            authentication_configuration: Some(v.authentication_configuration.into()),
        }
    }
}

impl From<crate::v2_7_2::types::BannerDto> for super::types::BannerDto {
    fn from(v: crate::v2_7_2::types::BannerDto) -> Self {
        Self {
            footer_text: v.footer_text,
            header_text: v.header_text,
        }
    }
}

impl From<crate::v2_7_2::types::BannerEntity> for super::types::BannerEntity {
    fn from(v: crate::v2_7_2::types::BannerEntity) -> Self {
        Self {
            banners: Some(v.banners.into()),
        }
    }
}

impl From<crate::v2_7_2::types::BatchSettingsDto> for super::types::BatchSettingsDto {
    fn from(v: crate::v2_7_2::types::BatchSettingsDto) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_7_2::types::BatchSize> for super::types::BatchSize {
    fn from(v: crate::v2_7_2::types::BatchSize) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_7_2::types::BuildInfo> for super::types::BuildInfo {
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

impl From<crate::v2_7_2::types::BulletinBoardDto> for super::types::BulletinBoardDto {
    fn from(v: crate::v2_7_2::types::BulletinBoardDto) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinBoardEntity> for super::types::BulletinBoardEntity {
    fn from(v: crate::v2_7_2::types::BulletinBoardEntity) -> Self {
        Self {
            bulletin_board: Some(v.bulletin_board.into()),
        }
    }
}

impl From<crate::v2_7_2::types::BulletinBoardPatternParameter>
    for super::types::BulletinBoardPatternParameter
{
    fn from(v: crate::v2_7_2::types::BulletinBoardPatternParameter) -> Self {
        Self {
            pattern: v.pattern.map(Into::into),
            raw_pattern: v.raw_pattern,
        }
    }
}

impl From<crate::v2_7_2::types::BulletinDto> for super::types::BulletinDto {
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

impl From<crate::v2_7_2::types::BulletinEntity> for super::types::BulletinEntity {
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

impl From<crate::v2_7_2::types::Bundle> for super::types::Bundle {
    fn from(v: crate::v2_7_2::types::Bundle) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::BundleDto> for super::types::BundleDto {
    fn from(v: crate::v2_7_2::types::BundleDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity>
    for super::types::ClearBulletinsForGroupRequestEntity
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
    for super::types::ClearBulletinsForGroupResultsEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsForGroupResultsEntity) -> Self {
        Self {
            bulletins_cleared: v.bulletins_cleared,
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsRequestEntity>
    for super::types::ClearBulletinsRequestEntity
{
    fn from(v: crate::v2_7_2::types::ClearBulletinsRequestEntity) -> Self {
        Self {
            from_timestamp: Some(v.from_timestamp),
        }
    }
}

impl From<crate::v2_7_2::types::ClearBulletinsResultEntity>
    for super::types::ClearBulletinsResultEntity
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

impl From<crate::v2_7_2::types::ClientIdParameter> for super::types::ClientIdParameter {
    fn from(v: crate::v2_7_2::types::ClientIdParameter) -> Self {
        Self {
            client_id: v.client_id,
        }
    }
}

impl From<crate::v2_7_2::types::ClusterDto> for super::types::ClusterDto {
    fn from(v: crate::v2_7_2::types::ClusterDto) -> Self {
        Self {
            generated: v.generated,
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterEntity> for super::types::ClusterEntity {
    fn from(v: crate::v2_7_2::types::ClusterEntity) -> Self {
        Self {
            cluster: Some(v.cluster.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterSearchResultsEntity>
    for super::types::ClusterSearchResultsEntity
{
    fn from(v: crate::v2_7_2::types::ClusterSearchResultsEntity) -> Self {
        Self {
            node_results: v
                .node_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ClusterSummaryDto> for super::types::ClusterSummaryDto {
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

impl From<crate::v2_7_2::types::ClusterSummaryEntity> for super::types::ClusterSummaryEntity {
    fn from(v: crate::v2_7_2::types::ClusterSummaryEntity) -> Self {
        Self {
            cluster_summary: Some(v.cluster_summary.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentDetailsDto> for super::types::ComponentDetailsDto {
    fn from(v: crate::v2_7_2::types::ComponentDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::ComponentDifferenceDto> for super::types::ComponentDifferenceDto {
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

impl From<crate::v2_7_2::types::ComponentHistoryDto> for super::types::ComponentHistoryDto {
    fn from(v: crate::v2_7_2::types::ComponentHistoryDto) -> Self {
        Self {
            component_id: v.component_id,
            property_history: v
                .property_history
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentHistoryEntity> for super::types::ComponentHistoryEntity {
    fn from(v: crate::v2_7_2::types::ComponentHistoryEntity) -> Self {
        Self {
            component_history: Some(v.component_history.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentManifest> for super::types::ComponentManifest {
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

impl From<crate::v2_7_2::types::ComponentReferenceDto> for super::types::ComponentReferenceDto {
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
    for super::types::ComponentReferenceEntity
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
    for super::types::ComponentRestrictionPermissionDto
{
    fn from(v: crate::v2_7_2::types::ComponentRestrictionPermissionDto) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentSearchResultDto>
    for super::types::ComponentSearchResultDto
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

impl From<crate::v2_7_2::types::ComponentStateDto> for super::types::ComponentStateDto {
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

impl From<crate::v2_7_2::types::ComponentStateEntity> for super::types::ComponentStateEntity {
    fn from(v: crate::v2_7_2::types::ComponentStateEntity) -> Self {
        Self {
            component_state: Some(v.component_state.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ComponentValidationResultDto>
    for super::types::ComponentValidationResultDto
{
    fn from(v: crate::v2_7_2::types::ComponentValidationResultDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            currently_valid: v.currently_valid,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resultant_validation_errors: v.resultant_validation_errors,
            results_valid: v.results_valid,
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::ComponentValidationResultEntity>
    for super::types::ComponentValidationResultEntity
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
    for super::types::ComponentValidationResultsEntity
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
    for super::types::ConfigVerificationResultDto
{
    fn from(v: crate::v2_7_2::types::ConfigVerificationResultDto) -> Self {
        Self {
            explanation: v.explanation,
            outcome: v.outcome.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            verification_step_name: v.verification_step_name,
        }
    }
}

impl From<crate::v2_7_2::types::ConfigurationAnalysisDto>
    for super::types::ConfigurationAnalysisDto
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
    for super::types::ConfigurationAnalysisEntity
{
    fn from(v: crate::v2_7_2::types::ConfigurationAnalysisEntity) -> Self {
        Self {
            configuration_analysis: Some(v.configuration_analysis.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectableComponent> for super::types::ConnectableComponent {
    fn from(v: crate::v2_7_2::types::ConnectableComponent) -> Self {
        Self {
            comments: v.comments,
            group_id: v.group_id,
            id: v.id,
            instance_identifier: v.instance_identifier,
            name: v.name,
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectableDto> for super::types::ConnectableDto {
    fn from(v: crate::v2_7_2::types::ConnectableDto) -> Self {
        Self {
            comments: v.comments,
            exists: v.exists,
            group_id: Some(v.group_id),
            id: Some(v.id),
            name: v.name,
            running: v.running,
            transmitting: v.transmitting,
            r#type: Some(
                serde_json::to_value(&v.r#type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionDto> for super::types::ConnectionDto {
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

impl From<crate::v2_7_2::types::ConnectionEntity> for super::types::ConnectionEntity {
    fn from(v: crate::v2_7_2::types::ConnectionEntity) -> Self {
        Self {
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            destination_group_id: v.destination_group_id,
            destination_id: v.destination_id,
            destination_type: Some(
                serde_json::to_value(&v.destination_type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            source_group_id: v.source_group_id,
            source_id: v.source_id,
            source_type: Some(
                serde_json::to_value(&v.source_type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatisticsDto> for super::types::ConnectionStatisticsDto {
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
    for super::types::ConnectionStatisticsEntity
{
    fn from(v: crate::v2_7_2::types::ConnectionStatisticsEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_statistics: v.connection_statistics.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatisticsSnapshotDto>
    for super::types::ConnectionStatisticsSnapshotDto
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

impl From<crate::v2_7_2::types::ConnectionStatusDto> for super::types::ConnectionStatusDto {
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

impl From<crate::v2_7_2::types::ConnectionStatusEntity> for super::types::ConnectionStatusEntity {
    fn from(v: crate::v2_7_2::types::ConnectionStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status: v.connection_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionStatusPredictionsSnapshotDto>
    for super::types::ConnectionStatusPredictionsSnapshotDto
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
    for super::types::ConnectionStatusSnapshotDto
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
            load_balance_status: v.load_balance_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::ConnectionStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::ConnectionStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status_snapshot: v.connection_status_snapshot.map(Into::into),
            id: v.id,
        }
    }
}

impl From<crate::v2_7_2::types::ConnectionsEntity> for super::types::ConnectionsEntity {
    fn from(v: crate::v2_7_2::types::ConnectionsEntity) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ContentViewerDto> for super::types::ContentViewerDto {
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

impl From<crate::v2_7_2::types::ContentViewerEntity> for super::types::ContentViewerEntity {
    fn from(v: crate::v2_7_2::types::ContentViewerEntity) -> Self {
        Self {
            content_viewers: v
                .content_viewers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerBulletinsEntity>
    for super::types::ControllerBulletinsEntity
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
    for super::types::ControllerConfigurationDto
{
    fn from(v: crate::v2_7_2::types::ControllerConfigurationDto) -> Self {
        Self {
            max_timer_driven_thread_count: v.max_timer_driven_thread_count,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerConfigurationEntity>
    for super::types::ControllerConfigurationEntity
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

impl From<crate::v2_7_2::types::ControllerDto> for super::types::ControllerDto {
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

impl From<crate::v2_7_2::types::ControllerEntity> for super::types::ControllerEntity {
    fn from(v: crate::v2_7_2::types::ControllerEntity) -> Self {
        Self {
            controller: Some(v.controller.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceAPI> for super::types::ControllerServiceAPI {
    fn from(v: crate::v2_7_2::types::ControllerServiceAPI) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceApiDto> for super::types::ControllerServiceApiDto {
    fn from(v: crate::v2_7_2::types::ControllerServiceApiDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceDefinition>
    for super::types::ControllerServiceDefinition
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

impl From<crate::v2_7_2::types::ControllerServiceDto> for super::types::ControllerServiceDto {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceEntity> for super::types::ControllerServiceEntity {
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
    for super::types::ControllerServiceReferencingComponentDto
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
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::ControllerServiceReferencingComponentEntity
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
    for super::types::ControllerServiceReferencingComponentsEntity
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
    for super::types::ControllerServiceRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ControllerServiceRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceStatusDto>
    for super::types::ControllerServiceStatusDto
{
    fn from(v: crate::v2_7_2::types::ControllerServiceStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ControllerServiceTypesEntity>
    for super::types::ControllerServiceTypesEntity
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
    for super::types::ControllerServicesEntity
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

impl From<crate::v2_7_2::types::ControllerStatusDto> for super::types::ControllerStatusDto {
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

impl From<crate::v2_7_2::types::ControllerStatusEntity> for super::types::ControllerStatusEntity {
    fn from(v: crate::v2_7_2::types::ControllerStatusEntity) -> Self {
        Self {
            controller_status: Some(v.controller_status.into()),
        }
    }
}

impl From<crate::v2_7_2::types::CopyRequestEntity> for super::types::CopyRequestEntity {
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

impl From<crate::v2_7_2::types::CopyResponseEntity> for super::types::CopyResponseEntity {
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
    for super::types::CopySnippetRequestEntity
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

impl From<crate::v2_7_2::types::CounterDto> for super::types::CounterDto {
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

impl From<crate::v2_7_2::types::CounterEntity> for super::types::CounterEntity {
    fn from(v: crate::v2_7_2::types::CounterEntity) -> Self {
        Self {
            counter: Some(v.counter.into()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersDto> for super::types::CountersDto {
    fn from(v: crate::v2_7_2::types::CountersDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersEntity> for super::types::CountersEntity {
    fn from(v: crate::v2_7_2::types::CountersEntity) -> Self {
        Self {
            counters: Some(v.counters.into()),
        }
    }
}

impl From<crate::v2_7_2::types::CountersSnapshotDto> for super::types::CountersSnapshotDto {
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
    for super::types::CreateActiveRequestEntity
{
    fn from(v: crate::v2_7_2::types::CreateActiveRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_id: v.process_group_id,
        }
    }
}

impl From<crate::v2_7_2::types::CurrentUserEntity> for super::types::CurrentUserEntity {
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

impl From<crate::v2_7_2::types::DateTimeParameter> for super::types::DateTimeParameter {
    fn from(v: crate::v2_7_2::types::DateTimeParameter) -> Self {
        Self {
            date_time: v.date_time,
        }
    }
}

impl From<crate::v2_7_2::types::DefinedType> for super::types::DefinedType {
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

impl From<crate::v2_7_2::types::DiagnosticLevel> for super::types::DiagnosticLevel {
    fn from(v: crate::v2_7_2::types::DiagnosticLevel) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::DifferenceDto> for super::types::DifferenceDto {
    fn from(v: crate::v2_7_2::types::DifferenceDto) -> Self {
        Self {
            difference: v.difference,
            difference_type: v.difference_type,
        }
    }
}

impl From<crate::v2_7_2::types::DimensionsDto> for super::types::DimensionsDto {
    fn from(v: crate::v2_7_2::types::DimensionsDto) -> Self {
        Self {
            height: v.height,
            width: v.width,
        }
    }
}

impl From<crate::v2_7_2::types::DocumentedTypeDto> for super::types::DocumentedTypeDto {
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

impl From<crate::v2_7_2::types::DropRequestDto> for super::types::DropRequestDto {
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

impl From<crate::v2_7_2::types::DropRequestEntity> for super::types::DropRequestEntity {
    fn from(v: crate::v2_7_2::types::DropRequestEntity) -> Self {
        Self {
            drop_request: Some(v.drop_request.into()),
        }
    }
}

impl From<crate::v2_7_2::types::DynamicProperty> for super::types::DynamicProperty {
    fn from(v: crate::v2_7_2::types::DynamicProperty) -> Self {
        Self {
            description: v.description,
            expression_language_scope: v.expression_language_scope.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            name: v.name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::DynamicRelationship> for super::types::DynamicRelationship {
    fn from(v: crate::v2_7_2::types::DynamicRelationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ExplicitRestrictionDto> for super::types::ExplicitRestrictionDto {
    fn from(v: crate::v2_7_2::types::ExplicitRestrictionDto) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ExternalControllerServiceReference>
    for super::types::ExternalControllerServiceReference
{
    fn from(v: crate::v2_7_2::types::ExternalControllerServiceReference) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisResultEntity>
    for super::types::FlowAnalysisResultEntity
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
    for super::types::FlowAnalysisRuleDefinition
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

impl From<crate::v2_7_2::types::FlowAnalysisRuleDto> for super::types::FlowAnalysisRuleDto {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleEntity> for super::types::FlowAnalysisRuleEntity {
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
    for super::types::FlowAnalysisRuleRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleStatusDto>
    for super::types::FlowAnalysisRuleStatusDto
{
    fn from(v: crate::v2_7_2::types::FlowAnalysisRuleStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::FlowAnalysisRuleTypesEntity>
    for super::types::FlowAnalysisRuleTypesEntity
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
    for super::types::FlowAnalysisRuleViolationDto
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

impl From<crate::v2_7_2::types::FlowAnalysisRulesEntity> for super::types::FlowAnalysisRulesEntity {
    fn from(v: crate::v2_7_2::types::FlowAnalysisRulesEntity) -> Self {
        Self {
            current_time: v.current_time,
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowBreadcrumbDto> for super::types::FlowBreadcrumbDto {
    fn from(v: crate::v2_7_2::types::FlowBreadcrumbDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::FlowBreadcrumbEntity> for super::types::FlowBreadcrumbEntity {
    fn from(v: crate::v2_7_2::types::FlowBreadcrumbEntity) -> Self {
        Self {
            breadcrumb: v.breadcrumb.map(Into::into),
            id: v.id,
            parent_breadcrumb: v.parent_breadcrumb.map(|v| Box::new((*v).into())),
            permissions: v.permissions.map(Into::into),
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::FlowComparisonEntity> for super::types::FlowComparisonEntity {
    fn from(v: crate::v2_7_2::types::FlowComparisonEntity) -> Self {
        Self {
            component_differences: v
                .component_differences
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowConfigurationDto> for super::types::FlowConfigurationDto {
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

impl From<crate::v2_7_2::types::FlowConfigurationEntity> for super::types::FlowConfigurationEntity {
    fn from(v: crate::v2_7_2::types::FlowConfigurationEntity) -> Self {
        Self {
            flow_configuration: Some(v.flow_configuration.into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowDto> for super::types::FlowDto {
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

impl From<crate::v2_7_2::types::FlowEntity> for super::types::FlowEntity {
    fn from(v: crate::v2_7_2::types::FlowEntity) -> Self {
        Self {
            flow: Some(v.flow.into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowFileDto> for super::types::FlowFileDto {
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

impl From<crate::v2_7_2::types::FlowFileEntity> for super::types::FlowFileEntity {
    fn from(v: crate::v2_7_2::types::FlowFileEntity) -> Self {
        Self {
            flow_file: Some(v.flow_file.into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowFileSummaryDto> for super::types::FlowFileSummaryDto {
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
    for super::types::FlowMetricsReportingStrategy
{
    fn from(v: crate::v2_7_2::types::FlowMetricsReportingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchDto> for super::types::FlowRegistryBranchDto {
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchDto) -> Self {
        Self { name: v.name }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchEntity>
    for super::types::FlowRegistryBranchEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchEntity) -> Self {
        Self {
            branch: Some(v.branch.into()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBranchesEntity>
    for super::types::FlowRegistryBranchesEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBranchesEntity) -> Self {
        Self {
            branches: v
                .branches
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryBucket> for super::types::FlowRegistryBucket {
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

impl From<crate::v2_7_2::types::FlowRegistryBucketDto> for super::types::FlowRegistryBucketDto {
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
    for super::types::FlowRegistryBucketEntity
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
    for super::types::FlowRegistryBucketsEntity
{
    fn from(v: crate::v2_7_2::types::FlowRegistryBucketsEntity) -> Self {
        Self {
            buckets: v.buckets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientDefinition>
    for super::types::FlowRegistryClientDefinition
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

impl From<crate::v2_7_2::types::FlowRegistryClientDto> for super::types::FlowRegistryClientDto {
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::FlowRegistryClientEntity>
    for super::types::FlowRegistryClientEntity
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
    for super::types::FlowRegistryClientTypesEntity
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
    for super::types::FlowRegistryClientsEntity
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

impl From<crate::v2_7_2::types::FlowRegistryPermissions> for super::types::FlowRegistryPermissions {
    fn from(v: crate::v2_7_2::types::FlowRegistryPermissions) -> Self {
        Self {
            can_delete: v.can_delete,
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_7_2::types::FlowSnippetDto> for super::types::FlowSnippetDto {
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

impl From<crate::v2_7_2::types::FunnelDto> for super::types::FunnelDto {
    fn from(v: crate::v2_7_2::types::FunnelDto) -> Self {
        Self {
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::FunnelEntity> for super::types::FunnelEntity {
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

impl From<crate::v2_7_2::types::FunnelsEntity> for super::types::FunnelsEntity {
    fn from(v: crate::v2_7_2::types::FunnelsEntity) -> Self {
        Self {
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::GarbageCollectionDto> for super::types::GarbageCollectionDto {
    fn from(v: crate::v2_7_2::types::GarbageCollectionDto) -> Self {
        Self {
            collection_count: v.collection_count,
            collection_millis: v.collection_millis,
            collection_time: v.collection_time,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::HistoryDto> for super::types::HistoryDto {
    fn from(v: crate::v2_7_2::types::HistoryDto) -> Self {
        Self {
            actions: v.actions.map(|v| v.into_iter().map(|v| v.into()).collect()),
            last_refreshed: v.last_refreshed,
            total: v.total,
        }
    }
}

impl From<crate::v2_7_2::types::HistoryEntity> for super::types::HistoryEntity {
    fn from(v: crate::v2_7_2::types::HistoryEntity) -> Self {
        Self {
            history: Some(v.history.into()),
        }
    }
}

impl From<crate::v2_7_2::types::IncludedRegistries> for super::types::IncludedRegistries {
    fn from(v: crate::v2_7_2::types::IncludedRegistries) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::InputPortsEntity> for super::types::InputPortsEntity {
    fn from(v: crate::v2_7_2::types::InputPortsEntity) -> Self {
        Self {
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::IntegerParameter> for super::types::IntegerParameter {
    fn from(v: crate::v2_7_2::types::IntegerParameter) -> Self {
        Self { integer: v.integer }
    }
}

impl From<crate::v2_7_2::types::JmxMetricsResultDto> for super::types::JmxMetricsResultDto {
    fn from(v: crate::v2_7_2::types::JmxMetricsResultDto) -> Self {
        Self {
            attribute_name: v.attribute_name,
            attribute_value: v.attribute_value.map(Into::into),
            bean_name: v.bean_name,
        }
    }
}

impl From<crate::v2_7_2::types::JmxMetricsResultsEntity> for super::types::JmxMetricsResultsEntity {
    fn from(v: crate::v2_7_2::types::JmxMetricsResultsEntity) -> Self {
        Self {
            jmx_metrics_results: v
                .jmx_metrics_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::LabelDto> for super::types::LabelDto {
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

impl From<crate::v2_7_2::types::LabelEntity> for super::types::LabelEntity {
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

impl From<crate::v2_7_2::types::LabelsEntity> for super::types::LabelsEntity {
    fn from(v: crate::v2_7_2::types::LabelsEntity) -> Self {
        Self {
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::LatestProvenanceEventsDto>
    for super::types::LatestProvenanceEventsDto
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
    for super::types::LatestProvenanceEventsEntity
{
    fn from(v: crate::v2_7_2::types::LatestProvenanceEventsEntity) -> Self {
        Self {
            latest_provenance_events: Some(v.latest_provenance_events.into()),
        }
    }
}

impl From<crate::v2_7_2::types::LineageDto> for super::types::LineageDto {
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

impl From<crate::v2_7_2::types::LineageEntity> for super::types::LineageEntity {
    fn from(v: crate::v2_7_2::types::LineageEntity) -> Self {
        Self {
            lineage: Some(v.lineage.into()),
        }
    }
}

impl From<crate::v2_7_2::types::LineageRequestDto> for super::types::LineageRequestDto {
    fn from(v: crate::v2_7_2::types::LineageRequestDto) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
            lineage_request_type: v.lineage_request_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            uuid: v.uuid,
        }
    }
}

impl From<crate::v2_7_2::types::LineageResultsDto> for super::types::LineageResultsDto {
    fn from(v: crate::v2_7_2::types::LineageResultsDto) -> Self {
        Self {
            errors: v.errors,
            links: v.links.map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ListenPortDto> for super::types::ListenPortDto {
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

impl From<crate::v2_7_2::types::ListenPortsEntity> for super::types::ListenPortsEntity {
    fn from(v: crate::v2_7_2::types::ListenPortsEntity) -> Self {
        Self {
            listen_ports: v
                .listen_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ListingRequestDto> for super::types::ListingRequestDto {
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

impl From<crate::v2_7_2::types::ListingRequestEntity> for super::types::ListingRequestEntity {
    fn from(v: crate::v2_7_2::types::ListingRequestEntity) -> Self {
        Self {
            listing_request: Some(v.listing_request.into()),
        }
    }
}

impl From<crate::v2_7_2::types::LongParameter> for super::types::LongParameter {
    fn from(v: crate::v2_7_2::types::LongParameter) -> Self {
        Self { long: v.long }
    }
}

impl From<crate::v2_7_2::types::MultiProcessorUseCase> for super::types::MultiProcessorUseCase {
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

impl From<crate::v2_7_2::types::NarCoordinateDto> for super::types::NarCoordinateDto {
    fn from(v: crate::v2_7_2::types::NarCoordinateDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::NarDetailsEntity> for super::types::NarDetailsEntity {
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

impl From<crate::v2_7_2::types::NarSummariesEntity> for super::types::NarSummariesEntity {
    fn from(v: crate::v2_7_2::types::NarSummariesEntity) -> Self {
        Self {
            current_time: v.current_time,
            nar_summaries: v
                .nar_summaries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::NarSummaryDto> for super::types::NarSummaryDto {
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

impl From<crate::v2_7_2::types::NarSummaryEntity> for super::types::NarSummaryEntity {
    fn from(v: crate::v2_7_2::types::NarSummaryEntity) -> Self {
        Self {
            nar_summary: Some(v.nar_summary.into()),
        }
    }
}

impl From<crate::v2_7_2::types::NodeConnectionStatisticsSnapshotDto>
    for super::types::NodeConnectionStatisticsSnapshotDto
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
    for super::types::NodeConnectionStatusSnapshotDto
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

impl From<crate::v2_7_2::types::NodeCountersSnapshotDto> for super::types::NodeCountersSnapshotDto {
    fn from(v: crate::v2_7_2::types::NodeCountersSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::NodeDto> for super::types::NodeDto {
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

impl From<crate::v2_7_2::types::NodeEntity> for super::types::NodeEntity {
    fn from(v: crate::v2_7_2::types::NodeEntity) -> Self {
        Self {
            node: Some(v.node.into()),
        }
    }
}

impl From<crate::v2_7_2::types::NodeEventDto> for super::types::NodeEventDto {
    fn from(v: crate::v2_7_2::types::NodeEventDto) -> Self {
        Self {
            category: v.category,
            message: v.message,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::NodePortStatusSnapshotDto>
    for super::types::NodePortStatusSnapshotDto
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
    for super::types::NodeProcessGroupStatusSnapshotDto
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
    for super::types::NodeProcessorStatusSnapshotDto
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
    for super::types::NodeRemoteProcessGroupStatusSnapshotDto
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
    for super::types::NodeReplayLastEventSnapshotDto
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

impl From<crate::v2_7_2::types::NodeSearchResultDto> for super::types::NodeSearchResultDto {
    fn from(v: crate::v2_7_2::types::NodeSearchResultDto) -> Self {
        Self {
            address: v.address,
            id: v.id,
        }
    }
}

impl From<crate::v2_7_2::types::NodeStatusSnapshotsDto> for super::types::NodeStatusSnapshotsDto {
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
    for super::types::NodeSystemDiagnosticsSnapshotDto
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

impl From<crate::v2_7_2::types::OutputPortsEntity> for super::types::OutputPortsEntity {
    fn from(v: crate::v2_7_2::types::OutputPortsEntity) -> Self {
        Self {
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextDto> for super::types::ParameterContextDto {
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

impl From<crate::v2_7_2::types::ParameterContextEntity> for super::types::ParameterContextEntity {
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
    for super::types::ParameterContextHandlingStrategy
{
    fn from(v: crate::v2_7_2::types::ParameterContextHandlingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_7_2::types::ParameterContextReferenceDto>
    for super::types::ParameterContextReferenceDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextReferenceEntity>
    for super::types::ParameterContextReferenceEntity
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
    for super::types::ParameterContextUpdateEntity
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
    for super::types::ParameterContextUpdateRequestDto
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
    for super::types::ParameterContextUpdateRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextUpdateRequestEntity) -> Self {
        Self {
            parameter_context_revision: v.parameter_context_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextUpdateStepDto>
    for super::types::ParameterContextUpdateStepDto
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
    for super::types::ParameterContextValidationRequestDto
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
    for super::types::ParameterContextValidationRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterContextValidationRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextValidationStepDto>
    for super::types::ParameterContextValidationStepDto
{
    fn from(v: crate::v2_7_2::types::ParameterContextValidationStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterContextsEntity> for super::types::ParameterContextsEntity {
    fn from(v: crate::v2_7_2::types::ParameterContextsEntity) -> Self {
        Self {
            current_time: v.current_time,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterDto> for super::types::ParameterDto {
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

impl From<crate::v2_7_2::types::ParameterEntity> for super::types::ParameterEntity {
    fn from(v: crate::v2_7_2::types::ParameterEntity) -> Self {
        Self {
            can_write: v.can_write,
            parameter: v.parameter.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterGroupConfigurationEntity>
    for super::types::ParameterGroupConfigurationEntity
{
    fn from(v: crate::v2_7_2::types::ParameterGroupConfigurationEntity) -> Self {
        Self {
            group_name: v.group_name,
            parameter_context_name: v.parameter_context_name,
            parameter_sensitivities: v.parameter_sensitivities.map(|m| {
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            v.map(|v| {
                                serde_json::to_value(&v)
                                    .ok()
                                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                                    .unwrap_or_default()
                            }),
                        )
                    })
                    .collect()
            }),
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderApplyParametersRequestDto>
    for super::types::ParameterProviderApplyParametersRequestDto
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
    for super::types::ParameterProviderApplyParametersRequestEntity
{
    fn from(v: crate::v2_7_2::types::ParameterProviderApplyParametersRequestEntity) -> Self {
        Self {
            request: Some(v.request.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderApplyParametersUpdateStepDto>
    for super::types::ParameterProviderApplyParametersUpdateStepDto
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
    for super::types::ParameterProviderConfigurationDto
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
    for super::types::ParameterProviderConfigurationEntity
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
    for super::types::ParameterProviderDefinition
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

impl From<crate::v2_7_2::types::ParameterProviderDto> for super::types::ParameterProviderDto {
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderEntity> for super::types::ParameterProviderEntity {
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
    for super::types::ParameterProviderParameterApplicationEntity
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
    for super::types::ParameterProviderParameterFetchEntity
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
    for super::types::ParameterProviderReference
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
    for super::types::ParameterProviderReferencingComponentDto
{
    fn from(v: crate::v2_7_2::types::ParameterProviderReferencingComponentDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ParameterProviderReferencingComponentEntity>
    for super::types::ParameterProviderReferencingComponentEntity
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
    for super::types::ParameterProviderReferencingComponentsEntity
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
    for super::types::ParameterProviderTypesEntity
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
    for super::types::ParameterProvidersEntity
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

impl From<crate::v2_7_2::types::ParameterStatusDto> for super::types::ParameterStatusDto {
    fn from(v: crate::v2_7_2::types::ParameterStatusDto) -> Self {
        Self {
            parameter: v.parameter.map(Into::into),
            status: v.status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::PasteRequestEntity> for super::types::PasteRequestEntity {
    fn from(v: crate::v2_7_2::types::PasteRequestEntity) -> Self {
        Self {
            copy_response: v.copy_response.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PasteResponseEntity> for super::types::PasteResponseEntity {
    fn from(v: crate::v2_7_2::types::PasteResponseEntity) -> Self {
        Self {
            flow: v.flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PeerDto> for super::types::PeerDto {
    fn from(v: crate::v2_7_2::types::PeerDto) -> Self {
        Self {
            flow_file_count: v.flow_file_count,
            hostname: v.hostname,
            port: v.port,
            secure: v.secure,
        }
    }
}

impl From<crate::v2_7_2::types::PeersEntity> for super::types::PeersEntity {
    fn from(v: crate::v2_7_2::types::PeersEntity) -> Self {
        Self {
            peers: v.peers.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PermissionsDto> for super::types::PermissionsDto {
    fn from(v: crate::v2_7_2::types::PermissionsDto) -> Self {
        Self {
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_7_2::types::PortDto> for super::types::PortDto {
    fn from(v: crate::v2_7_2::types::PortDto) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            port_function: v.port_function.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            transmitting: v.transmitting,
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::PortEntity> for super::types::PortEntity {
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

impl From<crate::v2_7_2::types::PortRunStatusEntity> for super::types::PortRunStatusEntity {
    fn from(v: crate::v2_7_2::types::PortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusDto> for super::types::PortStatusDto {
    fn from(v: crate::v2_7_2::types::PortStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stats_last_refreshed: v.stats_last_refreshed,
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusEntity> for super::types::PortStatusEntity {
    fn from(v: crate::v2_7_2::types::PortStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            port_status: v.port_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusSnapshotDto> for super::types::PortStatusSnapshotDto {
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
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_7_2::types::PortStatusSnapshotEntity>
    for super::types::PortStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::PortStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            port_status_snapshot: v.port_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::Position> for super::types::Position {
    fn from(v: crate::v2_7_2::types::Position) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_7_2::types::PositionDto> for super::types::PositionDto {
    fn from(v: crate::v2_7_2::types::PositionDto) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_7_2::types::PreviousValueDto> for super::types::PreviousValueDto {
    fn from(v: crate::v2_7_2::types::PreviousValueDto) -> Self {
        Self {
            previous_value: v.previous_value,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        }
    }
}

impl From<crate::v2_7_2::types::PrioritizerTypesEntity> for super::types::PrioritizerTypesEntity {
    fn from(v: crate::v2_7_2::types::PrioritizerTypesEntity) -> Self {
        Self {
            prioritizer_types: v
                .prioritizer_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupDto> for super::types::ProcessGroupDto {
    fn from(v: crate::v2_7_2::types::ProcessGroupDto) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            contents: v.contents.map(Into::into),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            disabled_count: v.disabled_count,
            execution_engine: v.execution_engine.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flowfile_concurrency: v.flowfile_concurrency.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flowfile_outbound_policy: v.flowfile_outbound_policy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            stateless_group_scheduled_state: v.stateless_group_scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            version_control_information: v.version_control_information.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupEntity> for super::types::ProcessGroupEntity {
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
            process_group_update_strategy: v.process_group_update_strategy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupFlowDto> for super::types::ProcessGroupFlowDto {
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

impl From<crate::v2_7_2::types::ProcessGroupFlowEntity> for super::types::ProcessGroupFlowEntity {
    fn from(v: crate::v2_7_2::types::ProcessGroupFlowEntity) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            process_group_flow: v.process_group_flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupImportEntity>
    for super::types::ProcessGroupImportEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupImportEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupNameDto> for super::types::ProcessGroupNameDto {
    fn from(v: crate::v2_7_2::types::ProcessGroupNameDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupReplaceRequestDto>
    for super::types::ProcessGroupReplaceRequestDto
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
    for super::types::ProcessGroupReplaceRequestEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupReplaceRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusDto> for super::types::ProcessGroupStatusDto {
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
    for super::types::ProcessGroupStatusEntity
{
    fn from(v: crate::v2_7_2::types::ProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            process_group_status: v.process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusSnapshotDto>
    for super::types::ProcessGroupStatusSnapshotDto
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
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            written: v.written,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessGroupStatusSnapshotEntity>
    for super::types::ProcessGroupStatusSnapshotEntity
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
    for super::types::ProcessGroupUploadEntity
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

impl From<crate::v2_7_2::types::ProcessGroupsEntity> for super::types::ProcessGroupsEntity {
    fn from(v: crate::v2_7_2::types::ProcessGroupsEntity) -> Self {
        Self {
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessingPerformanceStatusDto>
    for super::types::ProcessingPerformanceStatusDto
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

impl From<crate::v2_7_2::types::ProcessorConfigDto> for super::types::ProcessorConfigDto {
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

impl From<crate::v2_7_2::types::ProcessorConfiguration> for super::types::ProcessorConfiguration {
    fn from(v: crate::v2_7_2::types::ProcessorConfiguration) -> Self {
        Self {
            configuration: v.configuration,
            processor_class_name: v.processor_class_name,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorDefinition> for super::types::ProcessorDefinition {
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
            input_requirement: v.input_requirement.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::ProcessorDto> for super::types::ProcessorDto {
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
            physical_state: v.physical_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            relationships: v
                .relationships
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            style: v.style,
            supports_batching: v.supports_batching,
            supports_parallel_processing: v.supports_parallel_processing,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorEntity> for super::types::ProcessorEntity {
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
            physical_state: v.physical_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorRunStatusDetailsDto>
    for super::types::ProcessorRunStatusDetailsDto
{
    fn from(v: crate::v2_7_2::types::ProcessorRunStatusDetailsDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorRunStatusDetailsEntity>
    for super::types::ProcessorRunStatusDetailsEntity
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
    for super::types::ProcessorRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusDto> for super::types::ProcessorStatusDto {
    fn from(v: crate::v2_7_2::types::ProcessorStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stats_last_refreshed: v.stats_last_refreshed,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusEntity> for super::types::ProcessorStatusEntity {
    fn from(v: crate::v2_7_2::types::ProcessorStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            processor_status: v.processor_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorStatusSnapshotDto>
    for super::types::ProcessorStatusSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ProcessorStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_read: v.bytes_read,
            bytes_written: v.bytes_written,
            execution_node: v.execution_node.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            processing_performance_status: v.processing_performance_status.map(Into::into),
            read: v.read,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::ProcessorStatusSnapshotEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            processor_status_snapshot: v.processor_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorTypesEntity> for super::types::ProcessorTypesEntity {
    fn from(v: crate::v2_7_2::types::ProcessorTypesEntity) -> Self {
        Self {
            processor_types: v
                .processor_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorsEntity> for super::types::ProcessorsEntity {
    fn from(v: crate::v2_7_2::types::ProcessorsEntity) -> Self {
        Self {
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity>
    for super::types::ProcessorsRunStatusDetailsEntity
{
    fn from(v: crate::v2_7_2::types::ProcessorsRunStatusDetailsEntity) -> Self {
        Self {
            run_status_details: v
                .run_status_details
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyAllowableValue> for super::types::PropertyAllowableValue {
    fn from(v: crate::v2_7_2::types::PropertyAllowableValue) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDependency> for super::types::PropertyDependency {
    fn from(v: crate::v2_7_2::types::PropertyDependency) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_display_name: v.property_display_name,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDependencyDto> for super::types::PropertyDependencyDto {
    fn from(v: crate::v2_7_2::types::PropertyDependencyDto) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_7_2::types::PropertyDescriptor> for super::types::PropertyDescriptor {
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
            expression_language_scope: v.expression_language_scope.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::PropertyDescriptorDto> for super::types::PropertyDescriptorDto {
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
    for super::types::PropertyDescriptorEntity
{
    fn from(v: crate::v2_7_2::types::PropertyDescriptorEntity) -> Self {
        Self {
            property_descriptor: Some(v.property_descriptor.into()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyHistoryDto> for super::types::PropertyHistoryDto {
    fn from(v: crate::v2_7_2::types::PropertyHistoryDto) -> Self {
        Self {
            previous_values: v
                .previous_values
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyListenPortDefinition>
    for super::types::PropertyListenPortDefinition
{
    fn from(v: crate::v2_7_2::types::PropertyListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::PropertyResourceDefinition>
    for super::types::PropertyResourceDefinition
{
    fn from(v: crate::v2_7_2::types::PropertyResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resource_types: v.resource_types.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceDto> for super::types::ProvenanceDto {
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

impl From<crate::v2_7_2::types::ProvenanceEntity> for super::types::ProvenanceEntity {
    fn from(v: crate::v2_7_2::types::ProvenanceEntity) -> Self {
        Self {
            provenance: Some(v.provenance.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceEventDto> for super::types::ProvenanceEventDto {
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

impl From<crate::v2_7_2::types::ProvenanceEventEntity> for super::types::ProvenanceEventEntity {
    fn from(v: crate::v2_7_2::types::ProvenanceEventEntity) -> Self {
        Self {
            provenance_event: Some(v.provenance_event.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceLinkDto> for super::types::ProvenanceLinkDto {
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

impl From<crate::v2_7_2::types::ProvenanceNodeDto> for super::types::ProvenanceNodeDto {
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
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceOptionsDto> for super::types::ProvenanceOptionsDto {
    fn from(v: crate::v2_7_2::types::ProvenanceOptionsDto) -> Self {
        Self {
            searchable_fields: v
                .searchable_fields
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceOptionsEntity> for super::types::ProvenanceOptionsEntity {
    fn from(v: crate::v2_7_2::types::ProvenanceOptionsEntity) -> Self {
        Self {
            provenance_options: Some(v.provenance_options.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceRequestDto> for super::types::ProvenanceRequestDto {
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

impl From<crate::v2_7_2::types::ProvenanceResultsDto> for super::types::ProvenanceResultsDto {
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
    for super::types::ProvenanceSearchValueDto
{
    fn from(v: crate::v2_7_2::types::ProvenanceSearchValueDto) -> Self {
        Self {
            inverse: v.inverse,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::ProvenanceSearchableFieldDto>
    for super::types::ProvenanceSearchableFieldDto
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

impl From<crate::v2_7_2::types::QueueSizeDto> for super::types::QueueSizeDto {
    fn from(v: crate::v2_7_2::types::QueueSizeDto) -> Self {
        Self {
            byte_count: v.byte_count,
            object_count: v.object_count,
        }
    }
}

impl From<crate::v2_7_2::types::RegisteredFlow> for super::types::RegisteredFlow {
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

impl From<crate::v2_7_2::types::RegisteredFlowSnapshot> for super::types::RegisteredFlowSnapshot {
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
    for super::types::RegisteredFlowSnapshotMetadata
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
    for super::types::RegisteredFlowVersionInfo
{
    fn from(v: crate::v2_7_2::types::RegisteredFlowVersionInfo) -> Self {
        Self { version: v.version }
    }
}

impl From<crate::v2_7_2::types::Relationship> for super::types::Relationship {
    fn from(v: crate::v2_7_2::types::Relationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::RelationshipDto> for super::types::RelationshipDto {
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
    for super::types::RemotePortRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::RemotePortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupContentsDto>
    for super::types::RemoteProcessGroupContentsDto
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

impl From<crate::v2_7_2::types::RemoteProcessGroupDto> for super::types::RemoteProcessGroupDto {
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
    for super::types::RemoteProcessGroupEntity
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
    for super::types::RemoteProcessGroupPortDto
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
    for super::types::RemoteProcessGroupPortEntity
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
    for super::types::RemoteProcessGroupStatusDto
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusEntity>
    for super::types::RemoteProcessGroupStatusEntity
{
    fn from(v: crate::v2_7_2::types::RemoteProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            remote_process_group_status: v.remote_process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::RemoteProcessGroupStatusSnapshotDto>
    for super::types::RemoteProcessGroupStatusSnapshotDto
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
    for super::types::RemoteProcessGroupStatusSnapshotEntity
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
    for super::types::RemoteProcessGroupsEntity
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
    for super::types::ReplayLastEventRequestEntity
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventRequestEntity) -> Self {
        Self {
            component_id: v.component_id,
            nodes: v.nodes.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ReplayLastEventResponseEntity>
    for super::types::ReplayLastEventResponseEntity
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventResponseEntity) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            component_id: v.component_id,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ReplayLastEventSnapshotDto>
    for super::types::ReplayLastEventSnapshotDto
{
    fn from(v: crate::v2_7_2::types::ReplayLastEventSnapshotDto) -> Self {
        Self {
            event_available: v.event_available,
            events_replayed: v.events_replayed,
            failure_explanation: v.failure_explanation,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskDefinition> for super::types::ReportingTaskDefinition {
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

impl From<crate::v2_7_2::types::ReportingTaskDto> for super::types::ReportingTaskDto {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskEntity> for super::types::ReportingTaskEntity {
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
    for super::types::ReportingTaskRunStatusEntity
{
    fn from(v: crate::v2_7_2::types::ReportingTaskRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskStatusDto> for super::types::ReportingTaskStatusDto {
    fn from(v: crate::v2_7_2::types::ReportingTaskStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTaskTypesEntity>
    for super::types::ReportingTaskTypesEntity
{
    fn from(v: crate::v2_7_2::types::ReportingTaskTypesEntity) -> Self {
        Self {
            reporting_task_types: v
                .reporting_task_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::ReportingTasksEntity> for super::types::ReportingTasksEntity {
    fn from(v: crate::v2_7_2::types::ReportingTasksEntity) -> Self {
        Self {
            current_time: v.current_time,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::RequiredPermissionDto> for super::types::RequiredPermissionDto {
    fn from(v: crate::v2_7_2::types::RequiredPermissionDto) -> Self {
        Self {
            id: v.id,
            label: v.label,
        }
    }
}

impl From<crate::v2_7_2::types::ResourceClaimDetailsDto> for super::types::ResourceClaimDetailsDto {
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

impl From<crate::v2_7_2::types::ResourceDto> for super::types::ResourceDto {
    fn from(v: crate::v2_7_2::types::ResourceDto) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::ResourcesEntity> for super::types::ResourcesEntity {
    fn from(v: crate::v2_7_2::types::ResourcesEntity) -> Self {
        Self {
            resources: v
                .resources
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::Restriction> for super::types::Restriction {
    fn from(v: crate::v2_7_2::types::Restriction) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission,
        }
    }
}

impl From<crate::v2_7_2::types::RevisionDto> for super::types::RevisionDto {
    fn from(v: crate::v2_7_2::types::RevisionDto) -> Self {
        Self {
            client_id: v.client_id,
            last_modifier: v.last_modifier,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::RunStatusDetailsRequestEntity>
    for super::types::RunStatusDetailsRequestEntity
{
    fn from(v: crate::v2_7_2::types::RunStatusDetailsRequestEntity) -> Self {
        Self {
            processor_ids: v.processor_ids,
        }
    }
}

impl From<crate::v2_7_2::types::RuntimeManifest> for super::types::RuntimeManifest {
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

impl From<crate::v2_7_2::types::RuntimeManifestEntity> for super::types::RuntimeManifestEntity {
    fn from(v: crate::v2_7_2::types::RuntimeManifestEntity) -> Self {
        Self {
            runtime_manifest: Some(v.runtime_manifest.into()),
        }
    }
}

impl From<crate::v2_7_2::types::ScheduleComponentsEntity>
    for super::types::ScheduleComponentsEntity
{
    fn from(v: crate::v2_7_2::types::ScheduleComponentsEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::SchedulingDefaults> for super::types::SchedulingDefaults {
    fn from(v: crate::v2_7_2::types::SchedulingDefaults) -> Self {
        Self {
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_max_concurrent_tasks: v.default_max_concurrent_tasks,
            default_run_duration_nanos: v.default_run_duration_nanos,
            default_scheduling_period_millis: v.default_scheduling_period_millis,
            default_scheduling_periods_by_scheduling_strategy: v
                .default_scheduling_periods_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            penalization_period_millis: v.penalization_period_millis,
            yield_duration_millis: v.yield_duration_millis,
        }
    }
}

impl From<crate::v2_7_2::types::SearchResultGroupDto> for super::types::SearchResultGroupDto {
    fn from(v: crate::v2_7_2::types::SearchResultGroupDto) -> Self {
        Self {
            id: Some(v.id),
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::SearchResultsDto> for super::types::SearchResultsDto {
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

impl From<crate::v2_7_2::types::SearchResultsEntity> for super::types::SearchResultsEntity {
    fn from(v: crate::v2_7_2::types::SearchResultsEntity) -> Self {
        Self {
            search_results_d_t_o: Some(v.search_results_d_t_o.into()),
        }
    }
}

impl From<crate::v2_7_2::types::SnippetDto> for super::types::SnippetDto {
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

impl From<crate::v2_7_2::types::SnippetEntity> for super::types::SnippetEntity {
    fn from(v: crate::v2_7_2::types::SnippetEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            snippet: v.snippet.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StartVersionControlRequestEntity>
    for super::types::StartVersionControlRequestEntity
{
    fn from(v: crate::v2_7_2::types::StartVersionControlRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow: v.versioned_flow.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StateEntryDto> for super::types::StateEntryDto {
    fn from(v: crate::v2_7_2::types::StateEntryDto) -> Self {
        Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            key: v.key,
            value: v.value,
        }
    }
}

impl From<crate::v2_7_2::types::StateMapDto> for super::types::StateMapDto {
    fn from(v: crate::v2_7_2::types::StateMapDto) -> Self {
        Self {
            scope: v.scope,
            state: v.state.map(|v| v.into_iter().map(|v| v.into()).collect()),
            total_entry_count: v.total_entry_count,
        }
    }
}

impl From<crate::v2_7_2::types::Stateful> for super::types::Stateful {
    fn from(v: crate::v2_7_2::types::Stateful) -> Self {
        Self {
            description: v.description,
            scopes: v.scopes.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::StatusDescriptorDto> for super::types::StatusDescriptorDto {
    fn from(v: crate::v2_7_2::types::StatusDescriptorDto) -> Self {
        Self {
            description: v.description,
            field: v.field,
            formatter: v.formatter,
            label: v.label,
        }
    }
}

impl From<crate::v2_7_2::types::StatusHistoryDto> for super::types::StatusHistoryDto {
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

impl From<crate::v2_7_2::types::StatusHistoryEntity> for super::types::StatusHistoryEntity {
    fn from(v: crate::v2_7_2::types::StatusHistoryEntity) -> Self {
        Self {
            can_read: v.can_read,
            status_history: v.status_history.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::StatusSnapshotDto> for super::types::StatusSnapshotDto {
    fn from(v: crate::v2_7_2::types::StatusSnapshotDto) -> Self {
        Self {
            status_metrics: v.status_metrics,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_7_2::types::StorageUsageDto> for super::types::StorageUsageDto {
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

impl From<crate::v2_7_2::types::StreamingOutput> for super::types::StreamingOutput {
    fn from(v: crate::v2_7_2::types::StreamingOutput) -> Self {
        Self {}
    }
}

impl From<crate::v2_7_2::types::SubmitReplayRequestEntity>
    for super::types::SubmitReplayRequestEntity
{
    fn from(v: crate::v2_7_2::types::SubmitReplayRequestEntity) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
        }
    }
}

impl From<crate::v2_7_2::types::SupportedMimeTypesDto> for super::types::SupportedMimeTypesDto {
    fn from(v: crate::v2_7_2::types::SupportedMimeTypesDto) -> Self {
        Self {
            display_name: v.display_name,
            mime_types: v.mime_types,
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsDto> for super::types::SystemDiagnosticsDto {
    fn from(v: crate::v2_7_2::types::SystemDiagnosticsDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsEntity> for super::types::SystemDiagnosticsEntity {
    fn from(v: crate::v2_7_2::types::SystemDiagnosticsEntity) -> Self {
        Self {
            system_diagnostics: Some(v.system_diagnostics.into()),
        }
    }
}

impl From<crate::v2_7_2::types::SystemDiagnosticsSnapshotDto>
    for super::types::SystemDiagnosticsSnapshotDto
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
    for super::types::SystemResourceConsideration
{
    fn from(v: crate::v2_7_2::types::SystemResourceConsideration) -> Self {
        Self {
            description: v.description,
            resource: v.resource,
        }
    }
}

impl From<crate::v2_7_2::types::TenantDto> for super::types::TenantDto {
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

impl From<crate::v2_7_2::types::TenantEntity> for super::types::TenantEntity {
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

impl From<crate::v2_7_2::types::TenantsEntity> for super::types::TenantsEntity {
    fn from(v: crate::v2_7_2::types::TenantsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::TransactionResultEntity> for super::types::TransactionResultEntity {
    fn from(v: crate::v2_7_2::types::TransactionResultEntity) -> Self {
        Self {
            flow_file_sent: v.flow_file_sent,
            message: v.message,
            response_code: v.response_code,
        }
    }
}

impl From<crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity>
    for super::types::UpdateControllerServiceReferenceRequestEntity
{
    fn from(v: crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            referencing_component_revisions: v
                .referencing_component_revisions
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_7_2::types::UseCase> for super::types::UseCase {
    fn from(v: crate::v2_7_2::types::UseCase) -> Self {
        Self {
            configuration: v.configuration,
            description: v.description,
            input_requirement: v.input_requirement.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            keywords: v.keywords,
            notes: v.notes,
        }
    }
}

impl From<crate::v2_7_2::types::UserDto> for super::types::UserDto {
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

impl From<crate::v2_7_2::types::UserEntity> for super::types::UserEntity {
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

impl From<crate::v2_7_2::types::UserGroupDto> for super::types::UserGroupDto {
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

impl From<crate::v2_7_2::types::UserGroupEntity> for super::types::UserGroupEntity {
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

impl From<crate::v2_7_2::types::UserGroupsEntity> for super::types::UserGroupsEntity {
    fn from(v: crate::v2_7_2::types::UserGroupsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::UsersEntity> for super::types::UsersEntity {
    fn from(v: crate::v2_7_2::types::UsersEntity) -> Self {
        Self {
            generated: v.generated,
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VerifyConfigRequestDto> for super::types::VerifyConfigRequestDto {
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
    for super::types::VerifyConfigRequestEntity
{
    fn from(v: crate::v2_7_2::types::VerifyConfigRequestEntity) -> Self {
        Self {
            request: Some(v.request.into()),
        }
    }
}

impl From<crate::v2_7_2::types::VerifyConfigUpdateStepDto>
    for super::types::VerifyConfigUpdateStepDto
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
    for super::types::VersionControlComponentMappingEntity
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
    for super::types::VersionControlInformationDto
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            state_explanation: v.state_explanation,
            storage_location: v.storage_location,
            version: v.version,
        }
    }
}

impl From<crate::v2_7_2::types::VersionControlInformationEntity>
    for super::types::VersionControlInformationEntity
{
    fn from(v: crate::v2_7_2::types::VersionControlInformationEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionInfoDto> for super::types::VersionInfoDto {
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

impl From<crate::v2_7_2::types::VersionedAsset> for super::types::VersionedAsset {
    fn from(v: crate::v2_7_2::types::VersionedAsset) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedConnection> for super::types::VersionedConnection {
    fn from(v: crate::v2_7_2::types::VersionedConnection) -> Self {
        Self {
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::VersionedControllerService
{
    fn from(v: crate::v2_7_2::types::VersionedControllerService) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowCoordinates>
    for super::types::VersionedFlowCoordinates
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

impl From<crate::v2_7_2::types::VersionedFlowDto> for super::types::VersionedFlowDto {
    fn from(v: crate::v2_7_2::types::VersionedFlowDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::VersionedFlowEntity> for super::types::VersionedFlowEntity {
    fn from(v: crate::v2_7_2::types::VersionedFlowEntity) -> Self {
        Self {
            versioned_flow: Some(v.versioned_flow.into()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowSnapshotEntity>
    for super::types::VersionedFlowSnapshotEntity
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
    for super::types::VersionedFlowSnapshotMetadataEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowSnapshotMetadataEntity) -> Self {
        Self {
            registry_id: v.registry_id,
            versioned_flow_snapshot_metadata: v.versioned_flow_snapshot_metadata.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowSnapshotMetadataSetEntity>
    for super::types::VersionedFlowSnapshotMetadataSetEntity
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
    for super::types::VersionedFlowUpdateRequestDto
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
    for super::types::VersionedFlowUpdateRequestEntity
{
    fn from(v: crate::v2_7_2::types::VersionedFlowUpdateRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFlowsEntity> for super::types::VersionedFlowsEntity {
    fn from(v: crate::v2_7_2::types::VersionedFlowsEntity) -> Self {
        Self {
            versioned_flows: v
                .versioned_flows
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedFunnel> for super::types::VersionedFunnel {
    fn from(v: crate::v2_7_2::types::VersionedFunnel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedLabel> for super::types::VersionedLabel {
    fn from(v: crate::v2_7_2::types::VersionedLabel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
    for super::types::VersionedListenPortDefinition
{
    fn from(v: crate::v2_7_2::types::VersionedListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedParameter> for super::types::VersionedParameter {
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
    for super::types::VersionedParameterContext
{
    fn from(v: crate::v2_7_2::types::VersionedParameterContext) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::VersionedPort> for super::types::VersionedPort {
    fn from(v: crate::v2_7_2::types::VersionedPort) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            port_function: v.port_function.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedProcessGroup> for super::types::VersionedProcessGroup {
    fn from(v: crate::v2_7_2::types::VersionedProcessGroup) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            execution_engine: v.execution_engine.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stateless_flow_timeout: v.stateless_flow_timeout,
            versioned_flow_coordinates: v.versioned_flow_coordinates.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedProcessor> for super::types::VersionedProcessor {
    fn from(v: crate::v2_7_2::types::VersionedProcessor) -> Self {
        Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            style: v.style,
            r#type: v.r#type,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedPropertyDescriptor>
    for super::types::VersionedPropertyDescriptor
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
    for super::types::VersionedRemoteGroupPort
{
    fn from(v: crate::v2_7_2::types::VersionedRemoteGroupPort) -> Self {
        Self {
            batch_size: v.batch_size.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            remote_group_id: v.remote_group_id,
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            target_id: v.target_id,
            use_compression: v.use_compression,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedRemoteProcessGroup>
    for super::types::VersionedRemoteProcessGroup
{
    fn from(v: crate::v2_7_2::types::VersionedRemoteProcessGroup) -> Self {
        Self {
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_7_2::types::VersionedReportingTask> for super::types::VersionedReportingTask {
    fn from(v: crate::v2_7_2::types::VersionedReportingTask) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity>
    for super::types::VersionedReportingTaskImportRequestEntity
{
    fn from(v: crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            reporting_task_snapshot: v.reporting_task_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_7_2::types::VersionedReportingTaskImportResponseEntity>
    for super::types::VersionedReportingTaskImportResponseEntity
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
    for super::types::VersionedReportingTaskSnapshot
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
    for super::types::VersionedResourceDefinition
{
    fn from(v: crate::v2_7_2::types::VersionedResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resource_types: v.resource_types.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::AboutDto> for super::types::AboutDto {
    fn from(v: crate::v2_8_0::types::AboutDto) -> Self {
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

impl From<crate::v2_8_0::types::AboutEntity> for super::types::AboutEntity {
    fn from(v: crate::v2_8_0::types::AboutEntity) -> Self {
        Self {
            about: Some(v.about.into()),
        }
    }
}

impl From<crate::v2_8_0::types::AccessPolicyDto> for super::types::AccessPolicyDto {
    fn from(v: crate::v2_8_0::types::AccessPolicyDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::AccessPolicyEntity> for super::types::AccessPolicyEntity {
    fn from(v: crate::v2_8_0::types::AccessPolicyEntity) -> Self {
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

impl From<crate::v2_8_0::types::AccessPolicySummaryDto> for super::types::AccessPolicySummaryDto {
    fn from(v: crate::v2_8_0::types::AccessPolicySummaryDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::AccessPolicySummaryEntity>
    for super::types::AccessPolicySummaryEntity
{
    fn from(v: crate::v2_8_0::types::AccessPolicySummaryEntity) -> Self {
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

impl From<crate::v2_8_0::types::ActionDetailsDto> for super::types::ActionDetailsDto {
    fn from(v: crate::v2_8_0::types::ActionDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_8_0::types::ActionDto> for super::types::ActionDto {
    fn from(v: crate::v2_8_0::types::ActionDto) -> Self {
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

impl From<crate::v2_8_0::types::ActionEntity> for super::types::ActionEntity {
    fn from(v: crate::v2_8_0::types::ActionEntity) -> Self {
        Self {
            action: v.action.map(Into::into),
            can_read: v.can_read,
            id: v.id,
            source_id: v.source_id,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_8_0::types::ActivateControllerServicesEntity>
    for super::types::ActivateControllerServicesEntity
{
    fn from(v: crate::v2_8_0::types::ActivateControllerServicesEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::AdditionalDetailsEntity> for super::types::AdditionalDetailsEntity {
    fn from(v: crate::v2_8_0::types::AdditionalDetailsEntity) -> Self {
        Self {
            additional_details: v.additional_details,
        }
    }
}

impl From<crate::v2_8_0::types::AffectedComponentDto> for super::types::AffectedComponentDto {
    fn from(v: crate::v2_8_0::types::AffectedComponentDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_8_0::types::AffectedComponentEntity> for super::types::AffectedComponentEntity {
    fn from(v: crate::v2_8_0::types::AffectedComponentEntity) -> Self {
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
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            revision: v.revision.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_8_0::types::AllowableValueDto> for super::types::AllowableValueDto {
    fn from(v: crate::v2_8_0::types::AllowableValueDto) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::AllowableValueEntity> for super::types::AllowableValueEntity {
    fn from(v: crate::v2_8_0::types::AllowableValueEntity) -> Self {
        Self {
            allowable_value: v.allowable_value.map(Into::into),
            can_read: v.can_read,
        }
    }
}

impl From<crate::v2_8_0::types::AssetDto> for super::types::AssetDto {
    fn from(v: crate::v2_8_0::types::AssetDto) -> Self {
        Self {
            digest: v.digest,
            id: v.id,
            missing_content: v.missing_content,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::AssetEntity> for super::types::AssetEntity {
    fn from(v: crate::v2_8_0::types::AssetEntity) -> Self {
        Self {
            asset: Some(v.asset.into()),
        }
    }
}

impl From<crate::v2_8_0::types::AssetReferenceDto> for super::types::AssetReferenceDto {
    fn from(v: crate::v2_8_0::types::AssetReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::AssetsEntity> for super::types::AssetsEntity {
    fn from(v: crate::v2_8_0::types::AssetsEntity) -> Self {
        Self {
            assets: v.assets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::Attribute> for super::types::Attribute {
    fn from(v: crate::v2_8_0::types::Attribute) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::AttributeDto> for super::types::AttributeDto {
    fn from(v: crate::v2_8_0::types::AttributeDto) -> Self {
        Self {
            name: v.name,
            previous_value: v.previous_value,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::AuthenticationConfigurationDto>
    for super::types::AuthenticationConfigurationDto
{
    fn from(v: crate::v2_8_0::types::AuthenticationConfigurationDto) -> Self {
        Self {
            external_login_required: v.external_login_required,
            login_supported: v.login_supported,
            login_uri: v.login_uri,
            logout_uri: v.logout_uri,
        }
    }
}

impl From<crate::v2_8_0::types::AuthenticationConfigurationEntity>
    for super::types::AuthenticationConfigurationEntity
{
    fn from(v: crate::v2_8_0::types::AuthenticationConfigurationEntity) -> Self {
        Self {
            authentication_configuration: Some(v.authentication_configuration.into()),
        }
    }
}

impl From<crate::v2_8_0::types::BannerDto> for super::types::BannerDto {
    fn from(v: crate::v2_8_0::types::BannerDto) -> Self {
        Self {
            footer_text: v.footer_text,
            header_text: v.header_text,
        }
    }
}

impl From<crate::v2_8_0::types::BannerEntity> for super::types::BannerEntity {
    fn from(v: crate::v2_8_0::types::BannerEntity) -> Self {
        Self {
            banners: Some(v.banners.into()),
        }
    }
}

impl From<crate::v2_8_0::types::BatchSettingsDto> for super::types::BatchSettingsDto {
    fn from(v: crate::v2_8_0::types::BatchSettingsDto) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_8_0::types::BatchSize> for super::types::BatchSize {
    fn from(v: crate::v2_8_0::types::BatchSize) -> Self {
        Self {
            count: v.count,
            duration: v.duration,
            size: v.size,
        }
    }
}

impl From<crate::v2_8_0::types::BuildInfo> for super::types::BuildInfo {
    fn from(v: crate::v2_8_0::types::BuildInfo) -> Self {
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

impl From<crate::v2_8_0::types::BulletinBoardDto> for super::types::BulletinBoardDto {
    fn from(v: crate::v2_8_0::types::BulletinBoardDto) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
        }
    }
}

impl From<crate::v2_8_0::types::BulletinBoardEntity> for super::types::BulletinBoardEntity {
    fn from(v: crate::v2_8_0::types::BulletinBoardEntity) -> Self {
        Self {
            bulletin_board: Some(v.bulletin_board.into()),
        }
    }
}

impl From<crate::v2_8_0::types::BulletinBoardPatternParameter>
    for super::types::BulletinBoardPatternParameter
{
    fn from(v: crate::v2_8_0::types::BulletinBoardPatternParameter) -> Self {
        Self {
            pattern: v.pattern.map(Into::into),
            raw_pattern: v.raw_pattern,
        }
    }
}

impl From<crate::v2_8_0::types::BulletinDto> for super::types::BulletinDto {
    fn from(v: crate::v2_8_0::types::BulletinDto) -> Self {
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

impl From<crate::v2_8_0::types::BulletinEntity> for super::types::BulletinEntity {
    fn from(v: crate::v2_8_0::types::BulletinEntity) -> Self {
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

impl From<crate::v2_8_0::types::Bundle> for super::types::Bundle {
    fn from(v: crate::v2_8_0::types::Bundle) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::BundleDto> for super::types::BundleDto {
    fn from(v: crate::v2_8_0::types::BundleDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::ClearBulletinsForGroupRequestEntity>
    for super::types::ClearBulletinsForGroupRequestEntity
{
    fn from(v: crate::v2_8_0::types::ClearBulletinsForGroupRequestEntity) -> Self {
        Self {
            components: v.components,
            from_timestamp: Some(v.from_timestamp),
            id: v.id,
        }
    }
}

impl From<crate::v2_8_0::types::ClearBulletinsForGroupResultsEntity>
    for super::types::ClearBulletinsForGroupResultsEntity
{
    fn from(v: crate::v2_8_0::types::ClearBulletinsForGroupResultsEntity) -> Self {
        Self {
            bulletins_cleared: v.bulletins_cleared,
        }
    }
}

impl From<crate::v2_8_0::types::ClearBulletinsRequestEntity>
    for super::types::ClearBulletinsRequestEntity
{
    fn from(v: crate::v2_8_0::types::ClearBulletinsRequestEntity) -> Self {
        Self {
            from_timestamp: Some(v.from_timestamp),
        }
    }
}

impl From<crate::v2_8_0::types::ClearBulletinsResultEntity>
    for super::types::ClearBulletinsResultEntity
{
    fn from(v: crate::v2_8_0::types::ClearBulletinsResultEntity) -> Self {
        Self {
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            bulletins_cleared: v.bulletins_cleared,
            component_id: v.component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ClientIdParameter> for super::types::ClientIdParameter {
    fn from(v: crate::v2_8_0::types::ClientIdParameter) -> Self {
        Self {
            client_id: v.client_id,
        }
    }
}

impl From<crate::v2_8_0::types::ClusterDto> for super::types::ClusterDto {
    fn from(v: crate::v2_8_0::types::ClusterDto) -> Self {
        Self {
            generated: v.generated,
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ClusterEntity> for super::types::ClusterEntity {
    fn from(v: crate::v2_8_0::types::ClusterEntity) -> Self {
        Self {
            cluster: Some(v.cluster.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ClusterSearchResultsEntity>
    for super::types::ClusterSearchResultsEntity
{
    fn from(v: crate::v2_8_0::types::ClusterSearchResultsEntity) -> Self {
        Self {
            node_results: v
                .node_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ClusterSummaryDto> for super::types::ClusterSummaryDto {
    fn from(v: crate::v2_8_0::types::ClusterSummaryDto) -> Self {
        Self {
            clustered: v.clustered,
            connected_node_count: v.connected_node_count,
            connected_nodes: v.connected_nodes,
            connected_to_cluster: v.connected_to_cluster,
            total_node_count: v.total_node_count,
        }
    }
}

impl From<crate::v2_8_0::types::ClusterSummaryEntity> for super::types::ClusterSummaryEntity {
    fn from(v: crate::v2_8_0::types::ClusterSummaryEntity) -> Self {
        Self {
            cluster_summary: Some(v.cluster_summary.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ComponentDetailsDto> for super::types::ComponentDetailsDto {
    fn from(v: crate::v2_8_0::types::ComponentDetailsDto) -> Self {
        Self {}
    }
}

impl From<crate::v2_8_0::types::ComponentDifferenceDto> for super::types::ComponentDifferenceDto {
    fn from(v: crate::v2_8_0::types::ComponentDifferenceDto) -> Self {
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

impl From<crate::v2_8_0::types::ComponentHistoryDto> for super::types::ComponentHistoryDto {
    fn from(v: crate::v2_8_0::types::ComponentHistoryDto) -> Self {
        Self {
            component_id: v.component_id,
            property_history: v
                .property_history
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ComponentHistoryEntity> for super::types::ComponentHistoryEntity {
    fn from(v: crate::v2_8_0::types::ComponentHistoryEntity) -> Self {
        Self {
            component_history: Some(v.component_history.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ComponentManifest> for super::types::ComponentManifest {
    fn from(v: crate::v2_8_0::types::ComponentManifest) -> Self {
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

impl From<crate::v2_8_0::types::ComponentReferenceDto> for super::types::ComponentReferenceDto {
    fn from(v: crate::v2_8_0::types::ComponentReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ComponentReferenceEntity>
    for super::types::ComponentReferenceEntity
{
    fn from(v: crate::v2_8_0::types::ComponentReferenceEntity) -> Self {
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

impl From<crate::v2_8_0::types::ComponentRestrictionPermissionDto>
    for super::types::ComponentRestrictionPermissionDto
{
    fn from(v: crate::v2_8_0::types::ComponentRestrictionPermissionDto) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ComponentSearchResultDto>
    for super::types::ComponentSearchResultDto
{
    fn from(v: crate::v2_8_0::types::ComponentSearchResultDto) -> Self {
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

impl From<crate::v2_8_0::types::ComponentStateDto> for super::types::ComponentStateDto {
    fn from(v: crate::v2_8_0::types::ComponentStateDto) -> Self {
        Self {
            cluster_state: v.cluster_state.map(Into::into),
            component_id: v.component_id,
            drop_state_key_supported: v.drop_state_key_supported,
            local_state: v.local_state.map(Into::into),
            state_description: v.state_description,
        }
    }
}

impl From<crate::v2_8_0::types::ComponentStateEntity> for super::types::ComponentStateEntity {
    fn from(v: crate::v2_8_0::types::ComponentStateEntity) -> Self {
        Self {
            component_state: Some(v.component_state.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ComponentValidationResultDto>
    for super::types::ComponentValidationResultDto
{
    fn from(v: crate::v2_8_0::types::ComponentValidationResultDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            currently_valid: v.currently_valid,
            id: v.id,
            name: v.name,
            process_group_id: v.process_group_id,
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resultant_validation_errors: v.resultant_validation_errors,
            results_valid: v.results_valid,
            state: v.state,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_8_0::types::ComponentValidationResultEntity>
    for super::types::ComponentValidationResultEntity
{
    fn from(v: crate::v2_8_0::types::ComponentValidationResultEntity) -> Self {
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

impl From<crate::v2_8_0::types::ComponentValidationResultsEntity>
    for super::types::ComponentValidationResultsEntity
{
    fn from(v: crate::v2_8_0::types::ComponentValidationResultsEntity) -> Self {
        Self {
            validation_results: v
                .validation_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ConfigVerificationResultDto>
    for super::types::ConfigVerificationResultDto
{
    fn from(v: crate::v2_8_0::types::ConfigVerificationResultDto) -> Self {
        Self {
            explanation: v.explanation,
            outcome: v.outcome.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            verification_step_name: v.verification_step_name,
        }
    }
}

impl From<crate::v2_8_0::types::ConfigurationAnalysisDto>
    for super::types::ConfigurationAnalysisDto
{
    fn from(v: crate::v2_8_0::types::ConfigurationAnalysisDto) -> Self {
        Self {
            component_id: v.component_id,
            properties: v.properties,
            referenced_attributes: v.referenced_attributes,
            supports_verification: v.supports_verification,
        }
    }
}

impl From<crate::v2_8_0::types::ConfigurationAnalysisEntity>
    for super::types::ConfigurationAnalysisEntity
{
    fn from(v: crate::v2_8_0::types::ConfigurationAnalysisEntity) -> Self {
        Self {
            configuration_analysis: Some(v.configuration_analysis.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ConnectableComponent> for super::types::ConnectableComponent {
    fn from(v: crate::v2_8_0::types::ConnectableComponent) -> Self {
        Self {
            comments: v.comments,
            group_id: v.group_id,
            id: v.id,
            instance_identifier: v.instance_identifier,
            name: v.name,
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ConnectableDto> for super::types::ConnectableDto {
    fn from(v: crate::v2_8_0::types::ConnectableDto) -> Self {
        Self {
            comments: v.comments,
            exists: v.exists,
            group_id: Some(v.group_id),
            id: Some(v.id),
            name: v.name,
            running: v.running,
            transmitting: v.transmitting,
            r#type: Some(
                serde_json::to_value(&v.r#type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ConnectionDto> for super::types::ConnectionDto {
    fn from(v: crate::v2_8_0::types::ConnectionDto) -> Self {
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

impl From<crate::v2_8_0::types::ConnectionEntity> for super::types::ConnectionEntity {
    fn from(v: crate::v2_8_0::types::ConnectionEntity) -> Self {
        Self {
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            bulletins: v
                .bulletins
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            component: v.component.map(Into::into),
            destination_group_id: v.destination_group_id,
            destination_id: v.destination_id,
            destination_type: Some(
                serde_json::to_value(&v.destination_type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            getz_index: v.getz_index,
            id: v.id,
            label_index: v.label_index,
            permissions: v.permissions.map(Into::into),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            source_group_id: v.source_group_id,
            source_id: v.source_id,
            source_type: Some(
                serde_json::to_value(&v.source_type)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_8_0::types::ConnectionStatisticsDto> for super::types::ConnectionStatisticsDto {
    fn from(v: crate::v2_8_0::types::ConnectionStatisticsDto) -> Self {
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

impl From<crate::v2_8_0::types::ConnectionStatisticsEntity>
    for super::types::ConnectionStatisticsEntity
{
    fn from(v: crate::v2_8_0::types::ConnectionStatisticsEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_statistics: v.connection_statistics.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ConnectionStatisticsSnapshotDto>
    for super::types::ConnectionStatisticsSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ConnectionStatisticsSnapshotDto) -> Self {
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

impl From<crate::v2_8_0::types::ConnectionStatusDto> for super::types::ConnectionStatusDto {
    fn from(v: crate::v2_8_0::types::ConnectionStatusDto) -> Self {
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

impl From<crate::v2_8_0::types::ConnectionStatusEntity> for super::types::ConnectionStatusEntity {
    fn from(v: crate::v2_8_0::types::ConnectionStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status: v.connection_status.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ConnectionStatusPredictionsSnapshotDto>
    for super::types::ConnectionStatusPredictionsSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ConnectionStatusPredictionsSnapshotDto) -> Self {
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

impl From<crate::v2_8_0::types::ConnectionStatusSnapshotDto>
    for super::types::ConnectionStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ConnectionStatusSnapshotDto) -> Self {
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
            load_balance_status: v.load_balance_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::ConnectionStatusSnapshotEntity>
    for super::types::ConnectionStatusSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::ConnectionStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            connection_status_snapshot: v.connection_status_snapshot.map(Into::into),
            id: v.id,
        }
    }
}

impl From<crate::v2_8_0::types::ConnectionsEntity> for super::types::ConnectionsEntity {
    fn from(v: crate::v2_8_0::types::ConnectionsEntity) -> Self {
        Self {
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ContentViewerDto> for super::types::ContentViewerDto {
    fn from(v: crate::v2_8_0::types::ContentViewerDto) -> Self {
        Self {
            display_name: v.display_name,
            supported_mime_types: v
                .supported_mime_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_8_0::types::ContentViewerEntity> for super::types::ContentViewerEntity {
    fn from(v: crate::v2_8_0::types::ContentViewerEntity) -> Self {
        Self {
            content_viewers: v
                .content_viewers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerBulletinsEntity>
    for super::types::ControllerBulletinsEntity
{
    fn from(v: crate::v2_8_0::types::ControllerBulletinsEntity) -> Self {
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

impl From<crate::v2_8_0::types::ControllerConfigurationDto>
    for super::types::ControllerConfigurationDto
{
    fn from(v: crate::v2_8_0::types::ControllerConfigurationDto) -> Self {
        Self {
            max_timer_driven_thread_count: v.max_timer_driven_thread_count,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerConfigurationEntity>
    for super::types::ControllerConfigurationEntity
{
    fn from(v: crate::v2_8_0::types::ControllerConfigurationEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            permissions: v.permissions.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerDto> for super::types::ControllerDto {
    fn from(v: crate::v2_8_0::types::ControllerDto) -> Self {
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

impl From<crate::v2_8_0::types::ControllerEntity> for super::types::ControllerEntity {
    fn from(v: crate::v2_8_0::types::ControllerEntity) -> Self {
        Self {
            controller: Some(v.controller.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceAPI> for super::types::ControllerServiceAPI {
    fn from(v: crate::v2_8_0::types::ControllerServiceAPI) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceApiDto> for super::types::ControllerServiceApiDto {
    fn from(v: crate::v2_8_0::types::ControllerServiceApiDto) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceDefinition>
    for super::types::ControllerServiceDefinition
{
    fn from(v: crate::v2_8_0::types::ControllerServiceDefinition) -> Self {
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

impl From<crate::v2_8_0::types::ControllerServiceDto> for super::types::ControllerServiceDto {
    fn from(v: crate::v2_8_0::types::ControllerServiceDto) -> Self {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceEntity> for super::types::ControllerServiceEntity {
    fn from(v: crate::v2_8_0::types::ControllerServiceEntity) -> Self {
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

impl From<crate::v2_8_0::types::ControllerServiceReferencingComponentDto>
    for super::types::ControllerServiceReferencingComponentDto
{
    fn from(v: crate::v2_8_0::types::ControllerServiceReferencingComponentDto) -> Self {
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
            reference_type: v.reference_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            state: v.state,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceReferencingComponentEntity>
    for super::types::ControllerServiceReferencingComponentEntity
{
    fn from(v: crate::v2_8_0::types::ControllerServiceReferencingComponentEntity) -> Self {
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

impl From<crate::v2_8_0::types::ControllerServiceReferencingComponentsEntity>
    for super::types::ControllerServiceReferencingComponentsEntity
{
    fn from(v: crate::v2_8_0::types::ControllerServiceReferencingComponentsEntity) -> Self {
        Self {
            controller_service_referencing_components: v
                .controller_service_referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceRunStatusEntity>
    for super::types::ControllerServiceRunStatusEntity
{
    fn from(v: crate::v2_8_0::types::ControllerServiceRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceStatusDto>
    for super::types::ControllerServiceStatusDto
{
    fn from(v: crate::v2_8_0::types::ControllerServiceStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServiceTypesEntity>
    for super::types::ControllerServiceTypesEntity
{
    fn from(v: crate::v2_8_0::types::ControllerServiceTypesEntity) -> Self {
        Self {
            controller_service_types: v
                .controller_service_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ControllerServicesEntity>
    for super::types::ControllerServicesEntity
{
    fn from(v: crate::v2_8_0::types::ControllerServicesEntity) -> Self {
        Self {
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            current_time: v.current_time,
        }
    }
}

impl From<crate::v2_8_0::types::ControllerStatusDto> for super::types::ControllerStatusDto {
    fn from(v: crate::v2_8_0::types::ControllerStatusDto) -> Self {
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

impl From<crate::v2_8_0::types::ControllerStatusEntity> for super::types::ControllerStatusEntity {
    fn from(v: crate::v2_8_0::types::ControllerStatusEntity) -> Self {
        Self {
            controller_status: Some(v.controller_status.into()),
        }
    }
}

impl From<crate::v2_8_0::types::CopyRequestEntity> for super::types::CopyRequestEntity {
    fn from(v: crate::v2_8_0::types::CopyRequestEntity) -> Self {
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

impl From<crate::v2_8_0::types::CopyResponseEntity> for super::types::CopyResponseEntity {
    fn from(v: crate::v2_8_0::types::CopyResponseEntity) -> Self {
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

impl From<crate::v2_8_0::types::CopySnippetRequestEntity>
    for super::types::CopySnippetRequestEntity
{
    fn from(v: crate::v2_8_0::types::CopySnippetRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            origin_x: v.origin_x,
            origin_y: v.origin_y,
            snippet_id: v.snippet_id,
        }
    }
}

impl From<crate::v2_8_0::types::CounterDto> for super::types::CounterDto {
    fn from(v: crate::v2_8_0::types::CounterDto) -> Self {
        Self {
            context: v.context,
            id: v.id,
            name: v.name,
            value: v.value,
            value_count: v.value_count,
        }
    }
}

impl From<crate::v2_8_0::types::CounterEntity> for super::types::CounterEntity {
    fn from(v: crate::v2_8_0::types::CounterEntity) -> Self {
        Self {
            counter: Some(v.counter.into()),
        }
    }
}

impl From<crate::v2_8_0::types::CountersDto> for super::types::CountersDto {
    fn from(v: crate::v2_8_0::types::CountersDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::CountersEntity> for super::types::CountersEntity {
    fn from(v: crate::v2_8_0::types::CountersEntity) -> Self {
        Self {
            counters: Some(v.counters.into()),
        }
    }
}

impl From<crate::v2_8_0::types::CountersSnapshotDto> for super::types::CountersSnapshotDto {
    fn from(v: crate::v2_8_0::types::CountersSnapshotDto) -> Self {
        Self {
            counters: v
                .counters
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            generated: v.generated,
        }
    }
}

impl From<crate::v2_8_0::types::CreateActiveRequestEntity>
    for super::types::CreateActiveRequestEntity
{
    fn from(v: crate::v2_8_0::types::CreateActiveRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_id: v.process_group_id,
        }
    }
}

impl From<crate::v2_8_0::types::CurrentUserEntity> for super::types::CurrentUserEntity {
    fn from(v: crate::v2_8_0::types::CurrentUserEntity) -> Self {
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

impl From<crate::v2_8_0::types::DateTimeParameter> for super::types::DateTimeParameter {
    fn from(v: crate::v2_8_0::types::DateTimeParameter) -> Self {
        Self {
            date_time: v.date_time,
        }
    }
}

impl From<crate::v2_8_0::types::DefinedType> for super::types::DefinedType {
    fn from(v: crate::v2_8_0::types::DefinedType) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            r#type: v.r#type,
            type_description: v.type_description,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::DiagnosticLevel> for super::types::DiagnosticLevel {
    fn from(v: crate::v2_8_0::types::DiagnosticLevel) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_8_0::types::DifferenceDto> for super::types::DifferenceDto {
    fn from(v: crate::v2_8_0::types::DifferenceDto) -> Self {
        Self {
            difference: v.difference,
            difference_type: v.difference_type,
        }
    }
}

impl From<crate::v2_8_0::types::DimensionsDto> for super::types::DimensionsDto {
    fn from(v: crate::v2_8_0::types::DimensionsDto) -> Self {
        Self {
            height: v.height,
            width: v.width,
        }
    }
}

impl From<crate::v2_8_0::types::DocumentedTypeDto> for super::types::DocumentedTypeDto {
    fn from(v: crate::v2_8_0::types::DocumentedTypeDto) -> Self {
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

impl From<crate::v2_8_0::types::DropRequestDto> for super::types::DropRequestDto {
    fn from(v: crate::v2_8_0::types::DropRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::DropRequestEntity> for super::types::DropRequestEntity {
    fn from(v: crate::v2_8_0::types::DropRequestEntity) -> Self {
        Self {
            drop_request: Some(v.drop_request.into()),
        }
    }
}

impl From<crate::v2_8_0::types::DynamicProperty> for super::types::DynamicProperty {
    fn from(v: crate::v2_8_0::types::DynamicProperty) -> Self {
        Self {
            description: v.description,
            expression_language_scope: v.expression_language_scope.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            name: v.name,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::DynamicRelationship> for super::types::DynamicRelationship {
    fn from(v: crate::v2_8_0::types::DynamicRelationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::ExplicitRestrictionDto> for super::types::ExplicitRestrictionDto {
    fn from(v: crate::v2_8_0::types::ExplicitRestrictionDto) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ExternalControllerServiceReference>
    for super::types::ExternalControllerServiceReference
{
    fn from(v: crate::v2_8_0::types::ExternalControllerServiceReference) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisResultEntity>
    for super::types::FlowAnalysisResultEntity
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisResultEntity) -> Self {
        Self {
            flow_analysis_pending: v.flow_analysis_pending,
            rule_violations: v
                .rule_violations
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            rules: v.rules.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisRuleDefinition>
    for super::types::FlowAnalysisRuleDefinition
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleDefinition) -> Self {
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

impl From<crate::v2_8_0::types::FlowAnalysisRuleDto> for super::types::FlowAnalysisRuleDto {
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleDto) -> Self {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisRuleEntity> for super::types::FlowAnalysisRuleEntity {
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleEntity) -> Self {
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

impl From<crate::v2_8_0::types::FlowAnalysisRuleRunStatusEntity>
    for super::types::FlowAnalysisRuleRunStatusEntity
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisRuleStatusDto>
    for super::types::FlowAnalysisRuleStatusDto
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisRuleTypesEntity>
    for super::types::FlowAnalysisRuleTypesEntity
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleTypesEntity) -> Self {
        Self {
            flow_analysis_rule_types: v
                .flow_analysis_rule_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowAnalysisRuleViolationDto>
    for super::types::FlowAnalysisRuleViolationDto
{
    fn from(v: crate::v2_8_0::types::FlowAnalysisRuleViolationDto) -> Self {
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

impl From<crate::v2_8_0::types::FlowAnalysisRulesEntity> for super::types::FlowAnalysisRulesEntity {
    fn from(v: crate::v2_8_0::types::FlowAnalysisRulesEntity) -> Self {
        Self {
            current_time: v.current_time,
            flow_analysis_rules: v
                .flow_analysis_rules
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowBreadcrumbDto> for super::types::FlowBreadcrumbDto {
    fn from(v: crate::v2_8_0::types::FlowBreadcrumbDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::FlowBreadcrumbEntity> for super::types::FlowBreadcrumbEntity {
    fn from(v: crate::v2_8_0::types::FlowBreadcrumbEntity) -> Self {
        Self {
            breadcrumb: v.breadcrumb.map(Into::into),
            id: v.id,
            parent_breadcrumb: v.parent_breadcrumb.map(|v| Box::new((*v).into())),
            permissions: v.permissions.map(Into::into),
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::FlowComparisonEntity> for super::types::FlowComparisonEntity {
    fn from(v: crate::v2_8_0::types::FlowComparisonEntity) -> Self {
        Self {
            component_differences: v
                .component_differences
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowConfigurationDto> for super::types::FlowConfigurationDto {
    fn from(v: crate::v2_8_0::types::FlowConfigurationDto) -> Self {
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

impl From<crate::v2_8_0::types::FlowConfigurationEntity> for super::types::FlowConfigurationEntity {
    fn from(v: crate::v2_8_0::types::FlowConfigurationEntity) -> Self {
        Self {
            flow_configuration: Some(v.flow_configuration.into()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowDto> for super::types::FlowDto {
    fn from(v: crate::v2_8_0::types::FlowDto) -> Self {
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

impl From<crate::v2_8_0::types::FlowEntity> for super::types::FlowEntity {
    fn from(v: crate::v2_8_0::types::FlowEntity) -> Self {
        Self {
            flow: Some(v.flow.into()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowFileDto> for super::types::FlowFileDto {
    fn from(v: crate::v2_8_0::types::FlowFileDto) -> Self {
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

impl From<crate::v2_8_0::types::FlowFileEntity> for super::types::FlowFileEntity {
    fn from(v: crate::v2_8_0::types::FlowFileEntity) -> Self {
        Self {
            flow_file: Some(v.flow_file.into()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowFileSummaryDto> for super::types::FlowFileSummaryDto {
    fn from(v: crate::v2_8_0::types::FlowFileSummaryDto) -> Self {
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

impl From<crate::v2_8_0::types::FlowMetricsReportingStrategy>
    for super::types::FlowMetricsReportingStrategy
{
    fn from(v: crate::v2_8_0::types::FlowMetricsReportingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBranchDto> for super::types::FlowRegistryBranchDto {
    fn from(v: crate::v2_8_0::types::FlowRegistryBranchDto) -> Self {
        Self { name: v.name }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBranchEntity>
    for super::types::FlowRegistryBranchEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryBranchEntity) -> Self {
        Self {
            branch: Some(v.branch.into()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBranchesEntity>
    for super::types::FlowRegistryBranchesEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryBranchesEntity) -> Self {
        Self {
            branches: v
                .branches
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBucket> for super::types::FlowRegistryBucket {
    fn from(v: crate::v2_8_0::types::FlowRegistryBucket) -> Self {
        Self {
            created_timestamp: v.created_timestamp,
            description: v.description,
            identifier: v.identifier,
            name: v.name,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBucketDto> for super::types::FlowRegistryBucketDto {
    fn from(v: crate::v2_8_0::types::FlowRegistryBucketDto) -> Self {
        Self {
            created: v.created,
            description: v.description,
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBucketEntity>
    for super::types::FlowRegistryBucketEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryBucketEntity) -> Self {
        Self {
            bucket: v.bucket.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryBucketsEntity>
    for super::types::FlowRegistryBucketsEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryBucketsEntity) -> Self {
        Self {
            buckets: v.buckets.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryClientDefinition>
    for super::types::FlowRegistryClientDefinition
{
    fn from(v: crate::v2_8_0::types::FlowRegistryClientDefinition) -> Self {
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

impl From<crate::v2_8_0::types::FlowRegistryClientDto> for super::types::FlowRegistryClientDto {
    fn from(v: crate::v2_8_0::types::FlowRegistryClientDto) -> Self {
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryClientEntity>
    for super::types::FlowRegistryClientEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryClientEntity) -> Self {
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

impl From<crate::v2_8_0::types::FlowRegistryClientTypesEntity>
    for super::types::FlowRegistryClientTypesEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryClientTypesEntity) -> Self {
        Self {
            flow_registry_client_types: v
                .flow_registry_client_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryClientsEntity>
    for super::types::FlowRegistryClientsEntity
{
    fn from(v: crate::v2_8_0::types::FlowRegistryClientsEntity) -> Self {
        Self {
            current_time: v.current_time,
            registries: v
                .registries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::FlowRegistryPermissions> for super::types::FlowRegistryPermissions {
    fn from(v: crate::v2_8_0::types::FlowRegistryPermissions) -> Self {
        Self {
            can_delete: v.can_delete,
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_8_0::types::FlowSnippetDto> for super::types::FlowSnippetDto {
    fn from(v: crate::v2_8_0::types::FlowSnippetDto) -> Self {
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

impl From<crate::v2_8_0::types::FunnelDto> for super::types::FunnelDto {
    fn from(v: crate::v2_8_0::types::FunnelDto) -> Self {
        Self {
            id: v.id,
            parent_group_id: v.parent_group_id,
            position: v.position.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::FunnelEntity> for super::types::FunnelEntity {
    fn from(v: crate::v2_8_0::types::FunnelEntity) -> Self {
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

impl From<crate::v2_8_0::types::FunnelsEntity> for super::types::FunnelsEntity {
    fn from(v: crate::v2_8_0::types::FunnelsEntity) -> Self {
        Self {
            funnels: v.funnels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::GarbageCollectionDto> for super::types::GarbageCollectionDto {
    fn from(v: crate::v2_8_0::types::GarbageCollectionDto) -> Self {
        Self {
            collection_count: v.collection_count,
            collection_millis: v.collection_millis,
            collection_time: v.collection_time,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::HistoryDto> for super::types::HistoryDto {
    fn from(v: crate::v2_8_0::types::HistoryDto) -> Self {
        Self {
            actions: v.actions.map(|v| v.into_iter().map(|v| v.into()).collect()),
            last_refreshed: v.last_refreshed,
            total: v.total,
        }
    }
}

impl From<crate::v2_8_0::types::HistoryEntity> for super::types::HistoryEntity {
    fn from(v: crate::v2_8_0::types::HistoryEntity) -> Self {
        Self {
            history: Some(v.history.into()),
        }
    }
}

impl From<crate::v2_8_0::types::IncludedRegistries> for super::types::IncludedRegistries {
    fn from(v: crate::v2_8_0::types::IncludedRegistries) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_8_0::types::InputPortsEntity> for super::types::InputPortsEntity {
    fn from(v: crate::v2_8_0::types::InputPortsEntity) -> Self {
        Self {
            input_ports: v
                .input_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::IntegerParameter> for super::types::IntegerParameter {
    fn from(v: crate::v2_8_0::types::IntegerParameter) -> Self {
        Self { integer: v.integer }
    }
}

impl From<crate::v2_8_0::types::JmxMetricsResultDto> for super::types::JmxMetricsResultDto {
    fn from(v: crate::v2_8_0::types::JmxMetricsResultDto) -> Self {
        Self {
            attribute_name: v.attribute_name,
            attribute_value: v.attribute_value.map(Into::into),
            bean_name: v.bean_name,
        }
    }
}

impl From<crate::v2_8_0::types::JmxMetricsResultsEntity> for super::types::JmxMetricsResultsEntity {
    fn from(v: crate::v2_8_0::types::JmxMetricsResultsEntity) -> Self {
        Self {
            jmx_metrics_results: v
                .jmx_metrics_results
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::LabelDto> for super::types::LabelDto {
    fn from(v: crate::v2_8_0::types::LabelDto) -> Self {
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

impl From<crate::v2_8_0::types::LabelEntity> for super::types::LabelEntity {
    fn from(v: crate::v2_8_0::types::LabelEntity) -> Self {
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

impl From<crate::v2_8_0::types::LabelsEntity> for super::types::LabelsEntity {
    fn from(v: crate::v2_8_0::types::LabelsEntity) -> Self {
        Self {
            labels: v.labels.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::LatestProvenanceEventsDto>
    for super::types::LatestProvenanceEventsDto
{
    fn from(v: crate::v2_8_0::types::LatestProvenanceEventsDto) -> Self {
        Self {
            component_id: v.component_id,
            provenance_events: v
                .provenance_events
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::LatestProvenanceEventsEntity>
    for super::types::LatestProvenanceEventsEntity
{
    fn from(v: crate::v2_8_0::types::LatestProvenanceEventsEntity) -> Self {
        Self {
            latest_provenance_events: Some(v.latest_provenance_events.into()),
        }
    }
}

impl From<crate::v2_8_0::types::LineageDto> for super::types::LineageDto {
    fn from(v: crate::v2_8_0::types::LineageDto) -> Self {
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

impl From<crate::v2_8_0::types::LineageEntity> for super::types::LineageEntity {
    fn from(v: crate::v2_8_0::types::LineageEntity) -> Self {
        Self {
            lineage: Some(v.lineage.into()),
        }
    }
}

impl From<crate::v2_8_0::types::LineageRequestDto> for super::types::LineageRequestDto {
    fn from(v: crate::v2_8_0::types::LineageRequestDto) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
            lineage_request_type: v.lineage_request_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            uuid: v.uuid,
        }
    }
}

impl From<crate::v2_8_0::types::LineageResultsDto> for super::types::LineageResultsDto {
    fn from(v: crate::v2_8_0::types::LineageResultsDto) -> Self {
        Self {
            errors: v.errors,
            links: v.links.map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ListenPortDto> for super::types::ListenPortDto {
    fn from(v: crate::v2_8_0::types::ListenPortDto) -> Self {
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

impl From<crate::v2_8_0::types::ListenPortsEntity> for super::types::ListenPortsEntity {
    fn from(v: crate::v2_8_0::types::ListenPortsEntity) -> Self {
        Self {
            listen_ports: v
                .listen_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ListingRequestDto> for super::types::ListingRequestDto {
    fn from(v: crate::v2_8_0::types::ListingRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ListingRequestEntity> for super::types::ListingRequestEntity {
    fn from(v: crate::v2_8_0::types::ListingRequestEntity) -> Self {
        Self {
            listing_request: Some(v.listing_request.into()),
        }
    }
}

impl From<crate::v2_8_0::types::LongParameter> for super::types::LongParameter {
    fn from(v: crate::v2_8_0::types::LongParameter) -> Self {
        Self { long: v.long }
    }
}

impl From<crate::v2_8_0::types::MultiProcessorUseCase> for super::types::MultiProcessorUseCase {
    fn from(v: crate::v2_8_0::types::MultiProcessorUseCase) -> Self {
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

impl From<crate::v2_8_0::types::NarCoordinateDto> for super::types::NarCoordinateDto {
    fn from(v: crate::v2_8_0::types::NarCoordinateDto) -> Self {
        Self {
            artifact: v.artifact,
            group: v.group,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::NarDetailsEntity> for super::types::NarDetailsEntity {
    fn from(v: crate::v2_8_0::types::NarDetailsEntity) -> Self {
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

impl From<crate::v2_8_0::types::NarSummariesEntity> for super::types::NarSummariesEntity {
    fn from(v: crate::v2_8_0::types::NarSummariesEntity) -> Self {
        Self {
            current_time: v.current_time,
            nar_summaries: v
                .nar_summaries
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::NarSummaryDto> for super::types::NarSummaryDto {
    fn from(v: crate::v2_8_0::types::NarSummaryDto) -> Self {
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

impl From<crate::v2_8_0::types::NarSummaryEntity> for super::types::NarSummaryEntity {
    fn from(v: crate::v2_8_0::types::NarSummaryEntity) -> Self {
        Self {
            nar_summary: Some(v.nar_summary.into()),
        }
    }
}

impl From<crate::v2_8_0::types::NodeConnectionStatisticsSnapshotDto>
    for super::types::NodeConnectionStatisticsSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeConnectionStatisticsSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            statistics_snapshot: v.statistics_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeConnectionStatusSnapshotDto>
    for super::types::NodeConnectionStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeConnectionStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeCountersSnapshotDto> for super::types::NodeCountersSnapshotDto {
    fn from(v: crate::v2_8_0::types::NodeCountersSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeDto> for super::types::NodeDto {
    fn from(v: crate::v2_8_0::types::NodeDto) -> Self {
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

impl From<crate::v2_8_0::types::NodeEntity> for super::types::NodeEntity {
    fn from(v: crate::v2_8_0::types::NodeEntity) -> Self {
        Self {
            node: Some(v.node.into()),
        }
    }
}

impl From<crate::v2_8_0::types::NodeEventDto> for super::types::NodeEventDto {
    fn from(v: crate::v2_8_0::types::NodeEventDto) -> Self {
        Self {
            category: v.category,
            message: v.message,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_8_0::types::NodePortStatusSnapshotDto>
    for super::types::NodePortStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodePortStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeProcessGroupStatusSnapshotDto>
    for super::types::NodeProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeProcessGroupStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeProcessorStatusSnapshotDto>
    for super::types::NodeProcessorStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeProcessorStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeRemoteProcessGroupStatusSnapshotDto>
    for super::types::NodeRemoteProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeRemoteProcessGroupStatusSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            status_snapshot: v.status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeReplayLastEventSnapshotDto>
    for super::types::NodeReplayLastEventSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeReplayLastEventSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::NodeSearchResultDto> for super::types::NodeSearchResultDto {
    fn from(v: crate::v2_8_0::types::NodeSearchResultDto) -> Self {
        Self {
            address: v.address,
            id: v.id,
        }
    }
}

impl From<crate::v2_8_0::types::NodeStatusSnapshotsDto> for super::types::NodeStatusSnapshotsDto {
    fn from(v: crate::v2_8_0::types::NodeStatusSnapshotsDto) -> Self {
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

impl From<crate::v2_8_0::types::NodeSystemDiagnosticsSnapshotDto>
    for super::types::NodeSystemDiagnosticsSnapshotDto
{
    fn from(v: crate::v2_8_0::types::NodeSystemDiagnosticsSnapshotDto) -> Self {
        Self {
            address: v.address,
            api_port: v.api_port,
            node_id: v.node_id,
            snapshot: v.snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::OutputPortsEntity> for super::types::OutputPortsEntity {
    fn from(v: crate::v2_8_0::types::OutputPortsEntity) -> Self {
        Self {
            output_ports: v
                .output_ports
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextDto> for super::types::ParameterContextDto {
    fn from(v: crate::v2_8_0::types::ParameterContextDto) -> Self {
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

impl From<crate::v2_8_0::types::ParameterContextEntity> for super::types::ParameterContextEntity {
    fn from(v: crate::v2_8_0::types::ParameterContextEntity) -> Self {
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

impl From<crate::v2_8_0::types::ParameterContextHandlingStrategy>
    for super::types::ParameterContextHandlingStrategy
{
    fn from(v: crate::v2_8_0::types::ParameterContextHandlingStrategy) -> Self {
        let s = serde_json::to_string(&v).expect("serialize enum");
        serde_json::from_str(&s).expect("deserialize enum")
    }
}

impl From<crate::v2_8_0::types::ParameterContextReferenceDto>
    for super::types::ParameterContextReferenceDto
{
    fn from(v: crate::v2_8_0::types::ParameterContextReferenceDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextReferenceEntity>
    for super::types::ParameterContextReferenceEntity
{
    fn from(v: crate::v2_8_0::types::ParameterContextReferenceEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextUpdateEntity>
    for super::types::ParameterContextUpdateEntity
{
    fn from(v: crate::v2_8_0::types::ParameterContextUpdateEntity) -> Self {
        Self {
            parameter_context: v.parameter_context.map(Into::into),
            parameter_context_revision: v.parameter_context_revision.map(Into::into),
            referencing_components: v
                .referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextUpdateRequestDto>
    for super::types::ParameterContextUpdateRequestDto
{
    fn from(v: crate::v2_8_0::types::ParameterContextUpdateRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ParameterContextUpdateRequestEntity>
    for super::types::ParameterContextUpdateRequestEntity
{
    fn from(v: crate::v2_8_0::types::ParameterContextUpdateRequestEntity) -> Self {
        Self {
            parameter_context_revision: v.parameter_context_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextUpdateStepDto>
    for super::types::ParameterContextUpdateStepDto
{
    fn from(v: crate::v2_8_0::types::ParameterContextUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextValidationRequestDto>
    for super::types::ParameterContextValidationRequestDto
{
    fn from(v: crate::v2_8_0::types::ParameterContextValidationRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ParameterContextValidationRequestEntity>
    for super::types::ParameterContextValidationRequestEntity
{
    fn from(v: crate::v2_8_0::types::ParameterContextValidationRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextValidationStepDto>
    for super::types::ParameterContextValidationStepDto
{
    fn from(v: crate::v2_8_0::types::ParameterContextValidationStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterContextsEntity> for super::types::ParameterContextsEntity {
    fn from(v: crate::v2_8_0::types::ParameterContextsEntity) -> Self {
        Self {
            current_time: v.current_time,
            parameter_contexts: v
                .parameter_contexts
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterDto> for super::types::ParameterDto {
    fn from(v: crate::v2_8_0::types::ParameterDto) -> Self {
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

impl From<crate::v2_8_0::types::ParameterEntity> for super::types::ParameterEntity {
    fn from(v: crate::v2_8_0::types::ParameterEntity) -> Self {
        Self {
            can_write: v.can_write,
            parameter: v.parameter.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterGroupConfigurationEntity>
    for super::types::ParameterGroupConfigurationEntity
{
    fn from(v: crate::v2_8_0::types::ParameterGroupConfigurationEntity) -> Self {
        Self {
            group_name: v.group_name,
            parameter_context_name: v.parameter_context_name,
            parameter_sensitivities: v.parameter_sensitivities.map(|m| {
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            v.map(|v| {
                                serde_json::to_value(&v)
                                    .ok()
                                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                                    .unwrap_or_default()
                            }),
                        )
                    })
                    .collect()
            }),
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderApplyParametersRequestDto>
    for super::types::ParameterProviderApplyParametersRequestDto
{
    fn from(v: crate::v2_8_0::types::ParameterProviderApplyParametersRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ParameterProviderApplyParametersRequestEntity>
    for super::types::ParameterProviderApplyParametersRequestEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderApplyParametersRequestEntity) -> Self {
        Self {
            request: Some(v.request.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderApplyParametersUpdateStepDto>
    for super::types::ParameterProviderApplyParametersUpdateStepDto
{
    fn from(v: crate::v2_8_0::types::ParameterProviderApplyParametersUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderConfigurationDto>
    for super::types::ParameterProviderConfigurationDto
{
    fn from(v: crate::v2_8_0::types::ParameterProviderConfigurationDto) -> Self {
        Self {
            parameter_group_name: v.parameter_group_name,
            parameter_provider_id: v.parameter_provider_id,
            parameter_provider_name: v.parameter_provider_name,
            synchronized: v.synchronized,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderConfigurationEntity>
    for super::types::ParameterProviderConfigurationEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderConfigurationEntity) -> Self {
        Self {
            component: v.component.map(Into::into),
            id: v.id,
            permissions: v.permissions.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderDefinition>
    for super::types::ParameterProviderDefinition
{
    fn from(v: crate::v2_8_0::types::ParameterProviderDefinition) -> Self {
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

impl From<crate::v2_8_0::types::ParameterProviderDto> for super::types::ParameterProviderDto {
    fn from(v: crate::v2_8_0::types::ParameterProviderDto) -> Self {
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderEntity> for super::types::ParameterProviderEntity {
    fn from(v: crate::v2_8_0::types::ParameterProviderEntity) -> Self {
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

impl From<crate::v2_8_0::types::ParameterProviderParameterApplicationEntity>
    for super::types::ParameterProviderParameterApplicationEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderParameterApplicationEntity) -> Self {
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

impl From<crate::v2_8_0::types::ParameterProviderParameterFetchEntity>
    for super::types::ParameterProviderParameterFetchEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderParameterFetchEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderReference>
    for super::types::ParameterProviderReference
{
    fn from(v: crate::v2_8_0::types::ParameterProviderReference) -> Self {
        Self {
            bundle: v.bundle.map(Into::into),
            identifier: v.identifier,
            name: v.name,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderReferencingComponentDto>
    for super::types::ParameterProviderReferencingComponentDto
{
    fn from(v: crate::v2_8_0::types::ParameterProviderReferencingComponentDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderReferencingComponentEntity>
    for super::types::ParameterProviderReferencingComponentEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderReferencingComponentEntity) -> Self {
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

impl From<crate::v2_8_0::types::ParameterProviderReferencingComponentsEntity>
    for super::types::ParameterProviderReferencingComponentsEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderReferencingComponentsEntity) -> Self {
        Self {
            parameter_provider_referencing_components: v
                .parameter_provider_referencing_components
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProviderTypesEntity>
    for super::types::ParameterProviderTypesEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProviderTypesEntity) -> Self {
        Self {
            parameter_provider_types: v
                .parameter_provider_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterProvidersEntity>
    for super::types::ParameterProvidersEntity
{
    fn from(v: crate::v2_8_0::types::ParameterProvidersEntity) -> Self {
        Self {
            current_time: v.current_time,
            parameter_providers: v
                .parameter_providers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ParameterStatusDto> for super::types::ParameterStatusDto {
    fn from(v: crate::v2_8_0::types::ParameterStatusDto) -> Self {
        Self {
            parameter: v.parameter.map(Into::into),
            status: v.status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::PasteRequestEntity> for super::types::PasteRequestEntity {
    fn from(v: crate::v2_8_0::types::PasteRequestEntity) -> Self {
        Self {
            copy_response: v.copy_response.map(Into::into),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::PasteResponseEntity> for super::types::PasteResponseEntity {
    fn from(v: crate::v2_8_0::types::PasteResponseEntity) -> Self {
        Self {
            flow: v.flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::PeerDto> for super::types::PeerDto {
    fn from(v: crate::v2_8_0::types::PeerDto) -> Self {
        Self {
            flow_file_count: v.flow_file_count,
            hostname: v.hostname,
            port: v.port,
            secure: v.secure,
        }
    }
}

impl From<crate::v2_8_0::types::PeersEntity> for super::types::PeersEntity {
    fn from(v: crate::v2_8_0::types::PeersEntity) -> Self {
        Self {
            peers: v.peers.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::PermissionsDto> for super::types::PermissionsDto {
    fn from(v: crate::v2_8_0::types::PermissionsDto) -> Self {
        Self {
            can_read: v.can_read,
            can_write: v.can_write,
        }
    }
}

impl From<crate::v2_8_0::types::PortDto> for super::types::PortDto {
    fn from(v: crate::v2_8_0::types::PortDto) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            id: v.id,
            name: v.name,
            parent_group_id: v.parent_group_id,
            port_function: v.port_function.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            transmitting: v.transmitting,
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_errors: v.validation_errors,
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::PortEntity> for super::types::PortEntity {
    fn from(v: crate::v2_8_0::types::PortEntity) -> Self {
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

impl From<crate::v2_8_0::types::PortRunStatusEntity> for super::types::PortRunStatusEntity {
    fn from(v: crate::v2_8_0::types::PortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::PortStatusDto> for super::types::PortStatusDto {
    fn from(v: crate::v2_8_0::types::PortStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stats_last_refreshed: v.stats_last_refreshed,
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_8_0::types::PortStatusEntity> for super::types::PortStatusEntity {
    fn from(v: crate::v2_8_0::types::PortStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            port_status: v.port_status.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::PortStatusSnapshotDto> for super::types::PortStatusSnapshotDto {
    fn from(v: crate::v2_8_0::types::PortStatusSnapshotDto) -> Self {
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
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            transmitting: v.transmitting,
        }
    }
}

impl From<crate::v2_8_0::types::PortStatusSnapshotEntity>
    for super::types::PortStatusSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::PortStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            port_status_snapshot: v.port_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::Position> for super::types::Position {
    fn from(v: crate::v2_8_0::types::Position) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_8_0::types::PositionDto> for super::types::PositionDto {
    fn from(v: crate::v2_8_0::types::PositionDto) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<crate::v2_8_0::types::PreviousValueDto> for super::types::PreviousValueDto {
    fn from(v: crate::v2_8_0::types::PreviousValueDto) -> Self {
        Self {
            previous_value: v.previous_value,
            timestamp: v.timestamp,
            user_identity: v.user_identity,
        }
    }
}

impl From<crate::v2_8_0::types::PrioritizerTypesEntity> for super::types::PrioritizerTypesEntity {
    fn from(v: crate::v2_8_0::types::PrioritizerTypesEntity) -> Self {
        Self {
            prioritizer_types: v
                .prioritizer_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupDto> for super::types::ProcessGroupDto {
    fn from(v: crate::v2_8_0::types::ProcessGroupDto) -> Self {
        Self {
            active_remote_port_count: v.active_remote_port_count,
            comments: v.comments,
            contents: v.contents.map(Into::into),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            disabled_count: v.disabled_count,
            execution_engine: v.execution_engine.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flowfile_concurrency: v.flowfile_concurrency.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flowfile_outbound_policy: v.flowfile_outbound_policy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            stateless_group_scheduled_state: v.stateless_group_scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stopped_count: v.stopped_count,
            sync_failure_count: v.sync_failure_count,
            up_to_date_count: v.up_to_date_count,
            version_control_information: v.version_control_information.map(Into::into),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupEntity> for super::types::ProcessGroupEntity {
    fn from(v: crate::v2_8_0::types::ProcessGroupEntity) -> Self {
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
            process_group_update_strategy: v.process_group_update_strategy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupFlowDto> for super::types::ProcessGroupFlowDto {
    fn from(v: crate::v2_8_0::types::ProcessGroupFlowDto) -> Self {
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

impl From<crate::v2_8_0::types::ProcessGroupFlowEntity> for super::types::ProcessGroupFlowEntity {
    fn from(v: crate::v2_8_0::types::ProcessGroupFlowEntity) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            process_group_flow: v.process_group_flow.map(Into::into),
            revision: v.revision.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupImportEntity>
    for super::types::ProcessGroupImportEntity
{
    fn from(v: crate::v2_8_0::types::ProcessGroupImportEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupNameDto> for super::types::ProcessGroupNameDto {
    fn from(v: crate::v2_8_0::types::ProcessGroupNameDto) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupReplaceRequestDto>
    for super::types::ProcessGroupReplaceRequestDto
{
    fn from(v: crate::v2_8_0::types::ProcessGroupReplaceRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ProcessGroupReplaceRequestEntity>
    for super::types::ProcessGroupReplaceRequestEntity
{
    fn from(v: crate::v2_8_0::types::ProcessGroupReplaceRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
            versioned_flow_snapshot: v.versioned_flow_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupStatusDto> for super::types::ProcessGroupStatusDto {
    fn from(v: crate::v2_8_0::types::ProcessGroupStatusDto) -> Self {
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

impl From<crate::v2_8_0::types::ProcessGroupStatusEntity>
    for super::types::ProcessGroupStatusEntity
{
    fn from(v: crate::v2_8_0::types::ProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            process_group_status: v.process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupStatusSnapshotDto>
    for super::types::ProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ProcessGroupStatusSnapshotDto) -> Self {
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
            versioned_flow_state: v.versioned_flow_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            written: v.written,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupStatusSnapshotEntity>
    for super::types::ProcessGroupStatusSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::ProcessGroupStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            process_group_status_snapshot: v.process_group_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessGroupUploadEntity>
    for super::types::ProcessGroupUploadEntity
{
    fn from(v: crate::v2_8_0::types::ProcessGroupUploadEntity) -> Self {
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

impl From<crate::v2_8_0::types::ProcessGroupsEntity> for super::types::ProcessGroupsEntity {
    fn from(v: crate::v2_8_0::types::ProcessGroupsEntity) -> Self {
        Self {
            process_groups: v
                .process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessingPerformanceStatusDto>
    for super::types::ProcessingPerformanceStatusDto
{
    fn from(v: crate::v2_8_0::types::ProcessingPerformanceStatusDto) -> Self {
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

impl From<crate::v2_8_0::types::ProcessorConfigDto> for super::types::ProcessorConfigDto {
    fn from(v: crate::v2_8_0::types::ProcessorConfigDto) -> Self {
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

impl From<crate::v2_8_0::types::ProcessorConfiguration> for super::types::ProcessorConfiguration {
    fn from(v: crate::v2_8_0::types::ProcessorConfiguration) -> Self {
        Self {
            configuration: v.configuration,
            processor_class_name: v.processor_class_name,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorDefinition> for super::types::ProcessorDefinition {
    fn from(v: crate::v2_8_0::types::ProcessorDefinition) -> Self {
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
            input_requirement: v.input_requirement.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::ProcessorDto> for super::types::ProcessorDto {
    fn from(v: crate::v2_8_0::types::ProcessorDto) -> Self {
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
            physical_state: v.physical_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            relationships: v
                .relationships
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            restricted: v.restricted,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            style: v.style,
            supports_batching: v.supports_batching,
            supports_parallel_processing: v.supports_parallel_processing,
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorEntity> for super::types::ProcessorEntity {
    fn from(v: crate::v2_8_0::types::ProcessorEntity) -> Self {
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
            physical_state: v.physical_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            revision: v.revision.map(Into::into),
            status: v.status.map(Into::into),
            uri: v.uri,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorRunStatusDetailsDto>
    for super::types::ProcessorRunStatusDetailsDto
{
    fn from(v: crate::v2_8_0::types::ProcessorRunStatusDetailsDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            id: v.id,
            name: v.name,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_errors: v.validation_errors,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorRunStatusDetailsEntity>
    for super::types::ProcessorRunStatusDetailsEntity
{
    fn from(v: crate::v2_8_0::types::ProcessorRunStatusDetailsEntity) -> Self {
        Self {
            permissions: v.permissions.map(Into::into),
            revision: v.revision.map(Into::into),
            run_status_details: v.run_status_details.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorRunStatusEntity>
    for super::types::ProcessorRunStatusEntity
{
    fn from(v: crate::v2_8_0::types::ProcessorRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorStatusDto> for super::types::ProcessorStatusDto {
    fn from(v: crate::v2_8_0::types::ProcessorStatusDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            group_id: v.group_id,
            id: v.id,
            name: v.name,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stats_last_refreshed: v.stats_last_refreshed,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorStatusEntity> for super::types::ProcessorStatusEntity {
    fn from(v: crate::v2_8_0::types::ProcessorStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            processor_status: v.processor_status.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorStatusSnapshotDto>
    for super::types::ProcessorStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ProcessorStatusSnapshotDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            bytes_in: v.bytes_in,
            bytes_out: v.bytes_out,
            bytes_read: v.bytes_read,
            bytes_written: v.bytes_written,
            execution_node: v.execution_node.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            flow_files_in: v.flow_files_in,
            flow_files_out: v.flow_files_out,
            group_id: v.group_id,
            id: v.id,
            input: v.input,
            name: v.name,
            output: v.output,
            processing_performance_status: v.processing_performance_status.map(Into::into),
            read: v.read,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::ProcessorStatusSnapshotEntity>
    for super::types::ProcessorStatusSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::ProcessorStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            processor_status_snapshot: v.processor_status_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorTypesEntity> for super::types::ProcessorTypesEntity {
    fn from(v: crate::v2_8_0::types::ProcessorTypesEntity) -> Self {
        Self {
            processor_types: v
                .processor_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorsEntity> for super::types::ProcessorsEntity {
    fn from(v: crate::v2_8_0::types::ProcessorsEntity) -> Self {
        Self {
            processors: v
                .processors
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ProcessorsRunStatusDetailsEntity>
    for super::types::ProcessorsRunStatusDetailsEntity
{
    fn from(v: crate::v2_8_0::types::ProcessorsRunStatusDetailsEntity) -> Self {
        Self {
            run_status_details: v
                .run_status_details
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::PropertyAllowableValue> for super::types::PropertyAllowableValue {
    fn from(v: crate::v2_8_0::types::PropertyAllowableValue) -> Self {
        Self {
            description: v.description,
            display_name: v.display_name,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::PropertyDependency> for super::types::PropertyDependency {
    fn from(v: crate::v2_8_0::types::PropertyDependency) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_display_name: v.property_display_name,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_8_0::types::PropertyDependencyDto> for super::types::PropertyDependencyDto {
    fn from(v: crate::v2_8_0::types::PropertyDependencyDto) -> Self {
        Self {
            dependent_values: v.dependent_values,
            property_name: v.property_name,
        }
    }
}

impl From<crate::v2_8_0::types::PropertyDescriptor> for super::types::PropertyDescriptor {
    fn from(v: crate::v2_8_0::types::PropertyDescriptor) -> Self {
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
            expression_language_scope: v.expression_language_scope.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::PropertyDescriptorDto> for super::types::PropertyDescriptorDto {
    fn from(v: crate::v2_8_0::types::PropertyDescriptorDto) -> Self {
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

impl From<crate::v2_8_0::types::PropertyDescriptorEntity>
    for super::types::PropertyDescriptorEntity
{
    fn from(v: crate::v2_8_0::types::PropertyDescriptorEntity) -> Self {
        Self {
            property_descriptor: Some(v.property_descriptor.into()),
        }
    }
}

impl From<crate::v2_8_0::types::PropertyHistoryDto> for super::types::PropertyHistoryDto {
    fn from(v: crate::v2_8_0::types::PropertyHistoryDto) -> Self {
        Self {
            previous_values: v
                .previous_values
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::PropertyListenPortDefinition>
    for super::types::PropertyListenPortDefinition
{
    fn from(v: crate::v2_8_0::types::PropertyListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::PropertyResourceDefinition>
    for super::types::PropertyResourceDefinition
{
    fn from(v: crate::v2_8_0::types::PropertyResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resource_types: v.resource_types.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceDto> for super::types::ProvenanceDto {
    fn from(v: crate::v2_8_0::types::ProvenanceDto) -> Self {
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

impl From<crate::v2_8_0::types::ProvenanceEntity> for super::types::ProvenanceEntity {
    fn from(v: crate::v2_8_0::types::ProvenanceEntity) -> Self {
        Self {
            provenance: Some(v.provenance.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceEventDto> for super::types::ProvenanceEventDto {
    fn from(v: crate::v2_8_0::types::ProvenanceEventDto) -> Self {
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
            event_timestamp: v.event_timestamp,
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

impl From<crate::v2_8_0::types::ProvenanceEventEntity> for super::types::ProvenanceEventEntity {
    fn from(v: crate::v2_8_0::types::ProvenanceEventEntity) -> Self {
        Self {
            provenance_event: Some(v.provenance_event.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceLinkDto> for super::types::ProvenanceLinkDto {
    fn from(v: crate::v2_8_0::types::ProvenanceLinkDto) -> Self {
        Self {
            flow_file_uuid: v.flow_file_uuid,
            millis: v.millis,
            source_id: v.source_id,
            target_id: v.target_id,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceNodeDto> for super::types::ProvenanceNodeDto {
    fn from(v: crate::v2_8_0::types::ProvenanceNodeDto) -> Self {
        Self {
            child_uuids: v.child_uuids,
            cluster_node_identifier: v.cluster_node_identifier,
            component_type: v.component_type,
            event_type: v.event_type,
            flow_file_uuid: v.flow_file_uuid,
            id: v.id,
            millis: v.millis,
            parent_uuids: v.parent_uuids,
            timestamp: v.timestamp,
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceOptionsDto> for super::types::ProvenanceOptionsDto {
    fn from(v: crate::v2_8_0::types::ProvenanceOptionsDto) -> Self {
        Self {
            searchable_fields: v
                .searchable_fields
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceOptionsEntity> for super::types::ProvenanceOptionsEntity {
    fn from(v: crate::v2_8_0::types::ProvenanceOptionsEntity) -> Self {
        Self {
            provenance_options: Some(v.provenance_options.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceRequestDto> for super::types::ProvenanceRequestDto {
    fn from(v: crate::v2_8_0::types::ProvenanceRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::ProvenanceResultsDto> for super::types::ProvenanceResultsDto {
    fn from(v: crate::v2_8_0::types::ProvenanceResultsDto) -> Self {
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

impl From<crate::v2_8_0::types::ProvenanceSearchValueDto>
    for super::types::ProvenanceSearchValueDto
{
    fn from(v: crate::v2_8_0::types::ProvenanceSearchValueDto) -> Self {
        Self {
            inverse: v.inverse,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::ProvenanceSearchableFieldDto>
    for super::types::ProvenanceSearchableFieldDto
{
    fn from(v: crate::v2_8_0::types::ProvenanceSearchableFieldDto) -> Self {
        Self {
            field: v.field,
            id: v.id,
            label: v.label,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::QueueSizeDto> for super::types::QueueSizeDto {
    fn from(v: crate::v2_8_0::types::QueueSizeDto) -> Self {
        Self {
            byte_count: v.byte_count,
            object_count: v.object_count,
        }
    }
}

impl From<crate::v2_8_0::types::RegisteredFlow> for super::types::RegisteredFlow {
    fn from(v: crate::v2_8_0::types::RegisteredFlow) -> Self {
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

impl From<crate::v2_8_0::types::RegisteredFlowSnapshot> for super::types::RegisteredFlowSnapshot {
    fn from(v: crate::v2_8_0::types::RegisteredFlowSnapshot) -> Self {
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

impl From<crate::v2_8_0::types::RegisteredFlowSnapshotMetadata>
    for super::types::RegisteredFlowSnapshotMetadata
{
    fn from(v: crate::v2_8_0::types::RegisteredFlowSnapshotMetadata) -> Self {
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

impl From<crate::v2_8_0::types::RegisteredFlowVersionInfo>
    for super::types::RegisteredFlowVersionInfo
{
    fn from(v: crate::v2_8_0::types::RegisteredFlowVersionInfo) -> Self {
        Self { version: v.version }
    }
}

impl From<crate::v2_8_0::types::Relationship> for super::types::Relationship {
    fn from(v: crate::v2_8_0::types::Relationship) -> Self {
        Self {
            description: v.description,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::RelationshipDto> for super::types::RelationshipDto {
    fn from(v: crate::v2_8_0::types::RelationshipDto) -> Self {
        Self {
            auto_terminate: v.auto_terminate,
            description: v.description,
            name: v.name,
            retry: v.retry,
        }
    }
}

impl From<crate::v2_8_0::types::RemotePortRunStatusEntity>
    for super::types::RemotePortRunStatusEntity
{
    fn from(v: crate::v2_8_0::types::RemotePortRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::RemoteProcessGroupContentsDto>
    for super::types::RemoteProcessGroupContentsDto
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupContentsDto) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupDto> for super::types::RemoteProcessGroupDto {
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupDto) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupEntity>
    for super::types::RemoteProcessGroupEntity
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupEntity) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupPortDto>
    for super::types::RemoteProcessGroupPortDto
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupPortDto) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupPortEntity>
    for super::types::RemoteProcessGroupPortEntity
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupPortEntity) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupStatusDto>
    for super::types::RemoteProcessGroupStatusDto
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupStatusDto) -> Self {
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
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::RemoteProcessGroupStatusEntity>
    for super::types::RemoteProcessGroupStatusEntity
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupStatusEntity) -> Self {
        Self {
            can_read: v.can_read,
            remote_process_group_status: v.remote_process_group_status.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::RemoteProcessGroupStatusSnapshotDto>
    for super::types::RemoteProcessGroupStatusSnapshotDto
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupStatusSnapshotDto) -> Self {
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

impl From<crate::v2_8_0::types::RemoteProcessGroupStatusSnapshotEntity>
    for super::types::RemoteProcessGroupStatusSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupStatusSnapshotEntity) -> Self {
        Self {
            can_read: v.can_read,
            id: v.id,
            remote_process_group_status_snapshot: v
                .remote_process_group_status_snapshot
                .map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::RemoteProcessGroupsEntity>
    for super::types::RemoteProcessGroupsEntity
{
    fn from(v: crate::v2_8_0::types::RemoteProcessGroupsEntity) -> Self {
        Self {
            remote_process_groups: v
                .remote_process_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ReplayLastEventRequestEntity>
    for super::types::ReplayLastEventRequestEntity
{
    fn from(v: crate::v2_8_0::types::ReplayLastEventRequestEntity) -> Self {
        Self {
            component_id: v.component_id,
            nodes: v.nodes.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ReplayLastEventResponseEntity>
    for super::types::ReplayLastEventResponseEntity
{
    fn from(v: crate::v2_8_0::types::ReplayLastEventResponseEntity) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            component_id: v.component_id,
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            nodes: v.nodes.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ReplayLastEventSnapshotDto>
    for super::types::ReplayLastEventSnapshotDto
{
    fn from(v: crate::v2_8_0::types::ReplayLastEventSnapshotDto) -> Self {
        Self {
            event_available: v.event_available,
            events_replayed: v.events_replayed,
            failure_explanation: v.failure_explanation,
        }
    }
}

impl From<crate::v2_8_0::types::ReportingTaskDefinition> for super::types::ReportingTaskDefinition {
    fn from(v: crate::v2_8_0::types::ReportingTaskDefinition) -> Self {
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

impl From<crate::v2_8_0::types::ReportingTaskDto> for super::types::ReportingTaskDto {
    fn from(v: crate::v2_8_0::types::ReportingTaskDto) -> Self {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            supports_sensitive_dynamic_properties: v.supports_sensitive_dynamic_properties,
            r#type: v.r#type,
            validation_errors: v.validation_errors,
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            versioned_component_id: v.versioned_component_id,
        }
    }
}

impl From<crate::v2_8_0::types::ReportingTaskEntity> for super::types::ReportingTaskEntity {
    fn from(v: crate::v2_8_0::types::ReportingTaskEntity) -> Self {
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

impl From<crate::v2_8_0::types::ReportingTaskRunStatusEntity>
    for super::types::ReportingTaskRunStatusEntity
{
    fn from(v: crate::v2_8_0::types::ReportingTaskRunStatusEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            revision: v.revision.map(Into::into),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ReportingTaskStatusDto> for super::types::ReportingTaskStatusDto {
    fn from(v: crate::v2_8_0::types::ReportingTaskStatusDto) -> Self {
        Self {
            active_thread_count: v.active_thread_count,
            run_status: v.run_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            validation_status: v.validation_status.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::ReportingTaskTypesEntity>
    for super::types::ReportingTaskTypesEntity
{
    fn from(v: crate::v2_8_0::types::ReportingTaskTypesEntity) -> Self {
        Self {
            reporting_task_types: v
                .reporting_task_types
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::ReportingTasksEntity> for super::types::ReportingTasksEntity {
    fn from(v: crate::v2_8_0::types::ReportingTasksEntity) -> Self {
        Self {
            current_time: v.current_time,
            reporting_tasks: v
                .reporting_tasks
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::RequiredPermissionDto> for super::types::RequiredPermissionDto {
    fn from(v: crate::v2_8_0::types::RequiredPermissionDto) -> Self {
        Self {
            id: v.id,
            label: v.label,
        }
    }
}

impl From<crate::v2_8_0::types::ResourceClaimDetailsDto> for super::types::ResourceClaimDetailsDto {
    fn from(v: crate::v2_8_0::types::ResourceClaimDetailsDto) -> Self {
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

impl From<crate::v2_8_0::types::ResourceDto> for super::types::ResourceDto {
    fn from(v: crate::v2_8_0::types::ResourceDto) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::ResourcesEntity> for super::types::ResourcesEntity {
    fn from(v: crate::v2_8_0::types::ResourcesEntity) -> Self {
        Self {
            resources: v
                .resources
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::Restriction> for super::types::Restriction {
    fn from(v: crate::v2_8_0::types::Restriction) -> Self {
        Self {
            explanation: v.explanation,
            required_permission: v.required_permission,
        }
    }
}

impl From<crate::v2_8_0::types::RevisionDto> for super::types::RevisionDto {
    fn from(v: crate::v2_8_0::types::RevisionDto) -> Self {
        Self {
            client_id: v.client_id,
            last_modifier: v.last_modifier,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::RunStatusDetailsRequestEntity>
    for super::types::RunStatusDetailsRequestEntity
{
    fn from(v: crate::v2_8_0::types::RunStatusDetailsRequestEntity) -> Self {
        Self {
            processor_ids: v.processor_ids,
        }
    }
}

impl From<crate::v2_8_0::types::RuntimeManifest> for super::types::RuntimeManifest {
    fn from(v: crate::v2_8_0::types::RuntimeManifest) -> Self {
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

impl From<crate::v2_8_0::types::RuntimeManifestEntity> for super::types::RuntimeManifestEntity {
    fn from(v: crate::v2_8_0::types::RuntimeManifestEntity) -> Self {
        Self {
            runtime_manifest: Some(v.runtime_manifest.into()),
        }
    }
}

impl From<crate::v2_8_0::types::ScheduleComponentsEntity>
    for super::types::ScheduleComponentsEntity
{
    fn from(v: crate::v2_8_0::types::ScheduleComponentsEntity) -> Self {
        Self {
            components: v
                .components
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::SchedulingDefaults> for super::types::SchedulingDefaults {
    fn from(v: crate::v2_8_0::types::SchedulingDefaults) -> Self {
        Self {
            default_concurrent_tasks_by_scheduling_strategy: v
                .default_concurrent_tasks_by_scheduling_strategy,
            default_max_concurrent_tasks: v.default_max_concurrent_tasks,
            default_run_duration_nanos: v.default_run_duration_nanos,
            default_scheduling_period_millis: v.default_scheduling_period_millis,
            default_scheduling_periods_by_scheduling_strategy: v
                .default_scheduling_periods_by_scheduling_strategy,
            default_scheduling_strategy: v.default_scheduling_strategy.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            penalization_period_millis: v.penalization_period_millis,
            yield_duration_millis: v.yield_duration_millis,
        }
    }
}

impl From<crate::v2_8_0::types::SearchResultGroupDto> for super::types::SearchResultGroupDto {
    fn from(v: crate::v2_8_0::types::SearchResultGroupDto) -> Self {
        Self {
            id: Some(v.id),
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::SearchResultsDto> for super::types::SearchResultsDto {
    fn from(v: crate::v2_8_0::types::SearchResultsDto) -> Self {
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

impl From<crate::v2_8_0::types::SearchResultsEntity> for super::types::SearchResultsEntity {
    fn from(v: crate::v2_8_0::types::SearchResultsEntity) -> Self {
        Self {
            search_results_d_t_o: Some(v.search_results_d_t_o.into()),
        }
    }
}

impl From<crate::v2_8_0::types::SnippetDto> for super::types::SnippetDto {
    fn from(v: crate::v2_8_0::types::SnippetDto) -> Self {
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

impl From<crate::v2_8_0::types::SnippetEntity> for super::types::SnippetEntity {
    fn from(v: crate::v2_8_0::types::SnippetEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            snippet: v.snippet.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::StartVersionControlRequestEntity>
    for super::types::StartVersionControlRequestEntity
{
    fn from(v: crate::v2_8_0::types::StartVersionControlRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            versioned_flow: v.versioned_flow.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::StateEntryDto> for super::types::StateEntryDto {
    fn from(v: crate::v2_8_0::types::StateEntryDto) -> Self {
        Self {
            cluster_node_address: v.cluster_node_address,
            cluster_node_id: v.cluster_node_id,
            key: v.key,
            value: v.value,
        }
    }
}

impl From<crate::v2_8_0::types::StateMapDto> for super::types::StateMapDto {
    fn from(v: crate::v2_8_0::types::StateMapDto) -> Self {
        Self {
            scope: v.scope,
            state: v.state.map(|v| v.into_iter().map(|v| v.into()).collect()),
            total_entry_count: v.total_entry_count,
        }
    }
}

impl From<crate::v2_8_0::types::Stateful> for super::types::Stateful {
    fn from(v: crate::v2_8_0::types::Stateful) -> Self {
        Self {
            description: v.description,
            scopes: v.scopes.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::StatusDescriptorDto> for super::types::StatusDescriptorDto {
    fn from(v: crate::v2_8_0::types::StatusDescriptorDto) -> Self {
        Self {
            description: v.description,
            field: v.field,
            formatter: v.formatter,
            label: v.label,
        }
    }
}

impl From<crate::v2_8_0::types::StatusHistoryDto> for super::types::StatusHistoryDto {
    fn from(v: crate::v2_8_0::types::StatusHistoryDto) -> Self {
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

impl From<crate::v2_8_0::types::StatusHistoryEntity> for super::types::StatusHistoryEntity {
    fn from(v: crate::v2_8_0::types::StatusHistoryEntity) -> Self {
        Self {
            can_read: v.can_read,
            status_history: v.status_history.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::StatusSnapshotDto> for super::types::StatusSnapshotDto {
    fn from(v: crate::v2_8_0::types::StatusSnapshotDto) -> Self {
        Self {
            status_metrics: v.status_metrics,
            timestamp: v.timestamp,
        }
    }
}

impl From<crate::v2_8_0::types::StorageUsageDto> for super::types::StorageUsageDto {
    fn from(v: crate::v2_8_0::types::StorageUsageDto) -> Self {
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

impl From<crate::v2_8_0::types::StreamingOutput> for super::types::StreamingOutput {
    fn from(v: crate::v2_8_0::types::StreamingOutput) -> Self {
        Self {}
    }
}

impl From<crate::v2_8_0::types::SubmitReplayRequestEntity>
    for super::types::SubmitReplayRequestEntity
{
    fn from(v: crate::v2_8_0::types::SubmitReplayRequestEntity) -> Self {
        Self {
            cluster_node_id: v.cluster_node_id,
            event_id: v.event_id,
        }
    }
}

impl From<crate::v2_8_0::types::SupportedMimeTypesDto> for super::types::SupportedMimeTypesDto {
    fn from(v: crate::v2_8_0::types::SupportedMimeTypesDto) -> Self {
        Self {
            display_name: v.display_name,
            mime_types: v.mime_types,
        }
    }
}

impl From<crate::v2_8_0::types::SystemDiagnosticsDto> for super::types::SystemDiagnosticsDto {
    fn from(v: crate::v2_8_0::types::SystemDiagnosticsDto) -> Self {
        Self {
            aggregate_snapshot: v.aggregate_snapshot.map(Into::into),
            node_snapshots: v
                .node_snapshots
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::SystemDiagnosticsEntity> for super::types::SystemDiagnosticsEntity {
    fn from(v: crate::v2_8_0::types::SystemDiagnosticsEntity) -> Self {
        Self {
            system_diagnostics: Some(v.system_diagnostics.into()),
        }
    }
}

impl From<crate::v2_8_0::types::SystemDiagnosticsSnapshotDto>
    for super::types::SystemDiagnosticsSnapshotDto
{
    fn from(v: crate::v2_8_0::types::SystemDiagnosticsSnapshotDto) -> Self {
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

impl From<crate::v2_8_0::types::SystemResourceConsideration>
    for super::types::SystemResourceConsideration
{
    fn from(v: crate::v2_8_0::types::SystemResourceConsideration) -> Self {
        Self {
            description: v.description,
            resource: v.resource,
        }
    }
}

impl From<crate::v2_8_0::types::TenantDto> for super::types::TenantDto {
    fn from(v: crate::v2_8_0::types::TenantDto) -> Self {
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

impl From<crate::v2_8_0::types::TenantEntity> for super::types::TenantEntity {
    fn from(v: crate::v2_8_0::types::TenantEntity) -> Self {
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

impl From<crate::v2_8_0::types::TenantsEntity> for super::types::TenantsEntity {
    fn from(v: crate::v2_8_0::types::TenantsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::TransactionResultEntity> for super::types::TransactionResultEntity {
    fn from(v: crate::v2_8_0::types::TransactionResultEntity) -> Self {
        Self {
            flow_file_sent: v.flow_file_sent,
            message: v.message,
            response_code: v.response_code,
        }
    }
}

impl From<crate::v2_8_0::types::UpdateControllerServiceReferenceRequestEntity>
    for super::types::UpdateControllerServiceReferenceRequestEntity
{
    fn from(v: crate::v2_8_0::types::UpdateControllerServiceReferenceRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            id: v.id,
            referencing_component_revisions: v
                .referencing_component_revisions
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            ui_only: v.ui_only,
        }
    }
}

impl From<crate::v2_8_0::types::UseCase> for super::types::UseCase {
    fn from(v: crate::v2_8_0::types::UseCase) -> Self {
        Self {
            configuration: v.configuration,
            description: v.description,
            input_requirement: v.input_requirement.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            keywords: v.keywords,
            notes: v.notes,
        }
    }
}

impl From<crate::v2_8_0::types::UserDto> for super::types::UserDto {
    fn from(v: crate::v2_8_0::types::UserDto) -> Self {
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

impl From<crate::v2_8_0::types::UserEntity> for super::types::UserEntity {
    fn from(v: crate::v2_8_0::types::UserEntity) -> Self {
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

impl From<crate::v2_8_0::types::UserGroupDto> for super::types::UserGroupDto {
    fn from(v: crate::v2_8_0::types::UserGroupDto) -> Self {
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

impl From<crate::v2_8_0::types::UserGroupEntity> for super::types::UserGroupEntity {
    fn from(v: crate::v2_8_0::types::UserGroupEntity) -> Self {
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

impl From<crate::v2_8_0::types::UserGroupsEntity> for super::types::UserGroupsEntity {
    fn from(v: crate::v2_8_0::types::UserGroupsEntity) -> Self {
        Self {
            user_groups: v
                .user_groups
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::UsersEntity> for super::types::UsersEntity {
    fn from(v: crate::v2_8_0::types::UsersEntity) -> Self {
        Self {
            generated: v.generated,
            users: v.users.map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::VerifyConfigRequestDto> for super::types::VerifyConfigRequestDto {
    fn from(v: crate::v2_8_0::types::VerifyConfigRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::VerifyConfigRequestEntity>
    for super::types::VerifyConfigRequestEntity
{
    fn from(v: crate::v2_8_0::types::VerifyConfigRequestEntity) -> Self {
        Self {
            request: Some(v.request.into()),
        }
    }
}

impl From<crate::v2_8_0::types::VerifyConfigUpdateStepDto>
    for super::types::VerifyConfigUpdateStepDto
{
    fn from(v: crate::v2_8_0::types::VerifyConfigUpdateStepDto) -> Self {
        Self {
            complete: v.complete,
            description: v.description,
            failure_reason: v.failure_reason,
        }
    }
}

impl From<crate::v2_8_0::types::VersionControlComponentMappingEntity>
    for super::types::VersionControlComponentMappingEntity
{
    fn from(v: crate::v2_8_0::types::VersionControlComponentMappingEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            version_control_component_mapping: v.version_control_component_mapping,
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionControlInformationDto>
    for super::types::VersionControlInformationDto
{
    fn from(v: crate::v2_8_0::types::VersionControlInformationDto) -> Self {
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
            state: v.state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            state_explanation: v.state_explanation,
            storage_location: v.storage_location,
            version: v.version,
        }
    }
}

impl From<crate::v2_8_0::types::VersionControlInformationEntity>
    for super::types::VersionControlInformationEntity
{
    fn from(v: crate::v2_8_0::types::VersionControlInformationEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            process_group_revision: v.process_group_revision.map(Into::into),
            version_control_information: v.version_control_information.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionInfoDto> for super::types::VersionInfoDto {
    fn from(v: crate::v2_8_0::types::VersionInfoDto) -> Self {
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

impl From<crate::v2_8_0::types::VersionedAsset> for super::types::VersionedAsset {
    fn from(v: crate::v2_8_0::types::VersionedAsset) -> Self {
        Self {
            identifier: v.identifier,
            name: v.name,
        }
    }
}

impl From<crate::v2_8_0::types::VersionedConnection> for super::types::VersionedConnection {
    fn from(v: crate::v2_8_0::types::VersionedConnection) -> Self {
        Self {
            back_pressure_data_size_threshold: v.back_pressure_data_size_threshold,
            back_pressure_object_threshold: v.back_pressure_object_threshold,
            bends: v.bends.map(|v| v.into_iter().map(|v| v.into()).collect()),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::VersionedControllerService>
    for super::types::VersionedControllerService
{
    fn from(v: crate::v2_8_0::types::VersionedControllerService) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFlowCoordinates>
    for super::types::VersionedFlowCoordinates
{
    fn from(v: crate::v2_8_0::types::VersionedFlowCoordinates) -> Self {
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

impl From<crate::v2_8_0::types::VersionedFlowDto> for super::types::VersionedFlowDto {
    fn from(v: crate::v2_8_0::types::VersionedFlowDto) -> Self {
        Self {
            action: v.action.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::VersionedFlowEntity> for super::types::VersionedFlowEntity {
    fn from(v: crate::v2_8_0::types::VersionedFlowEntity) -> Self {
        Self {
            versioned_flow: Some(v.versioned_flow.into()),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFlowSnapshotEntity>
    for super::types::VersionedFlowSnapshotEntity
{
    fn from(v: crate::v2_8_0::types::VersionedFlowSnapshotEntity) -> Self {
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

impl From<crate::v2_8_0::types::VersionedFlowSnapshotMetadataEntity>
    for super::types::VersionedFlowSnapshotMetadataEntity
{
    fn from(v: crate::v2_8_0::types::VersionedFlowSnapshotMetadataEntity) -> Self {
        Self {
            registry_id: v.registry_id,
            versioned_flow_snapshot_metadata: v.versioned_flow_snapshot_metadata.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFlowSnapshotMetadataSetEntity>
    for super::types::VersionedFlowSnapshotMetadataSetEntity
{
    fn from(v: crate::v2_8_0::types::VersionedFlowSnapshotMetadataSetEntity) -> Self {
        Self {
            versioned_flow_snapshot_metadata_set: v
                .versioned_flow_snapshot_metadata_set
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFlowUpdateRequestDto>
    for super::types::VersionedFlowUpdateRequestDto
{
    fn from(v: crate::v2_8_0::types::VersionedFlowUpdateRequestDto) -> Self {
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

impl From<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity>
    for super::types::VersionedFlowUpdateRequestEntity
{
    fn from(v: crate::v2_8_0::types::VersionedFlowUpdateRequestEntity) -> Self {
        Self {
            process_group_revision: v.process_group_revision.map(Into::into),
            request: v.request.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFlowsEntity> for super::types::VersionedFlowsEntity {
    fn from(v: crate::v2_8_0::types::VersionedFlowsEntity) -> Self {
        Self {
            versioned_flows: v
                .versioned_flows
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedFunnel> for super::types::VersionedFunnel {
    fn from(v: crate::v2_8_0::types::VersionedFunnel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedLabel> for super::types::VersionedLabel {
    fn from(v: crate::v2_8_0::types::VersionedLabel) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::VersionedListenPortDefinition>
    for super::types::VersionedListenPortDefinition
{
    fn from(v: crate::v2_8_0::types::VersionedListenPortDefinition) -> Self {
        Self {
            application_protocols: v.application_protocols,
            transport_protocol: v.transport_protocol.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedParameter> for super::types::VersionedParameter {
    fn from(v: crate::v2_8_0::types::VersionedParameter) -> Self {
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

impl From<crate::v2_8_0::types::VersionedParameterContext>
    for super::types::VersionedParameterContext
{
    fn from(v: crate::v2_8_0::types::VersionedParameterContext) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::VersionedPort> for super::types::VersionedPort {
    fn from(v: crate::v2_8_0::types::VersionedPort) -> Self {
        Self {
            allow_remote_access: v.allow_remote_access,
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            port_function: v.port_function.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            position: v.position.map(Into::into),
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            r#type: v.r#type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedProcessGroup> for super::types::VersionedProcessGroup {
    fn from(v: crate::v2_8_0::types::VersionedProcessGroup) -> Self {
        Self {
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            connections: v
                .connections
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            controller_services: v
                .controller_services
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            default_back_pressure_data_size_threshold: v.default_back_pressure_data_size_threshold,
            default_back_pressure_object_threshold: v.default_back_pressure_object_threshold,
            default_flow_file_expiration: v.default_flow_file_expiration,
            execution_engine: v.execution_engine.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            stateless_flow_timeout: v.stateless_flow_timeout,
            versioned_flow_coordinates: v.versioned_flow_coordinates.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedProcessor> for super::types::VersionedProcessor {
    fn from(v: crate::v2_8_0::types::VersionedProcessor) -> Self {
        Self {
            annotation_data: v.annotation_data,
            auto_terminated_relationships: v.auto_terminated_relationships,
            backoff_mechanism: v.backoff_mechanism,
            bulletin_level: v.bulletin_level,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            style: v.style,
            r#type: v.r#type,
            yield_duration: v.yield_duration,
        }
    }
}

impl From<crate::v2_8_0::types::VersionedPropertyDescriptor>
    for super::types::VersionedPropertyDescriptor
{
    fn from(v: crate::v2_8_0::types::VersionedPropertyDescriptor) -> Self {
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

impl From<crate::v2_8_0::types::VersionedRemoteGroupPort>
    for super::types::VersionedRemoteGroupPort
{
    fn from(v: crate::v2_8_0::types::VersionedRemoteGroupPort) -> Self {
        Self {
            batch_size: v.batch_size.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            concurrently_schedulable_task_count: v.concurrently_schedulable_task_count,
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            remote_group_id: v.remote_group_id,
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            target_id: v.target_id,
            use_compression: v.use_compression,
        }
    }
}

impl From<crate::v2_8_0::types::VersionedRemoteProcessGroup>
    for super::types::VersionedRemoteProcessGroup
{
    fn from(v: crate::v2_8_0::types::VersionedRemoteProcessGroup) -> Self {
        Self {
            comments: v.comments,
            communications_timeout: v.communications_timeout,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
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

impl From<crate::v2_8_0::types::VersionedReportingTask> for super::types::VersionedReportingTask {
    fn from(v: crate::v2_8_0::types::VersionedReportingTask) -> Self {
        Self {
            annotation_data: v.annotation_data,
            bundle: v.bundle.map(Into::into),
            comments: v.comments,
            component_type: v.component_type.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            group_identifier: v.group_identifier,
            identifier: v.identifier,
            instance_identifier: v.instance_identifier,
            name: v.name,
            position: v.position.map(Into::into),
            properties: v.properties,
            property_descriptors: v
                .property_descriptors
                .map(|m| m.into_iter().map(|(k, v)| (k, v.map(Into::into))).collect()),
            scheduled_state: v.scheduled_state.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            scheduling_period: v.scheduling_period,
            scheduling_strategy: v.scheduling_strategy,
            r#type: v.r#type,
        }
    }
}

impl From<crate::v2_8_0::types::VersionedReportingTaskImportRequestEntity>
    for super::types::VersionedReportingTaskImportRequestEntity
{
    fn from(v: crate::v2_8_0::types::VersionedReportingTaskImportRequestEntity) -> Self {
        Self {
            disconnected_node_acknowledged: v.disconnected_node_acknowledged,
            reporting_task_snapshot: v.reporting_task_snapshot.map(Into::into),
        }
    }
}

impl From<crate::v2_8_0::types::VersionedReportingTaskImportResponseEntity>
    for super::types::VersionedReportingTaskImportResponseEntity
{
    fn from(v: crate::v2_8_0::types::VersionedReportingTaskImportResponseEntity) -> Self {
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

impl From<crate::v2_8_0::types::VersionedReportingTaskSnapshot>
    for super::types::VersionedReportingTaskSnapshot
{
    fn from(v: crate::v2_8_0::types::VersionedReportingTaskSnapshot) -> Self {
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

impl From<crate::v2_8_0::types::VersionedResourceDefinition>
    for super::types::VersionedResourceDefinition
{
    fn from(v: crate::v2_8_0::types::VersionedResourceDefinition) -> Self {
        Self {
            cardinality: v.cardinality.map(|v| {
                serde_json::to_value(&v)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default()
            }),
            resource_types: v.resource_types.map(|v| {
                v.into_iter()
                    .map(|v| {
                        serde_json::to_value(&v)
                            .ok()
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect()
            }),
        }
    }
}

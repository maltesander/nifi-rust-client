// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerApi;
use crate::dynamic::traits::ControllerBulletinsApi;
use crate::dynamic::traits::ControllerConfigApi;
use crate::dynamic::traits::ControllerContentApi;
use crate::dynamic::traits::ControllerDescriptorsApi;
use crate::dynamic::traits::ControllerDetailsApi;
use crate::dynamic::traits::ControllerRunStatusApi;
use crate::dynamic::traits::ControllerStateApi;
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
impl<'a> ControllerApiDispatch<'a> {
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
impl ControllerApi for ControllerApiDispatch<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ControllerBulletinsApi + 'b {
        ControllerBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn config<'b>(&'b self, id: &'b str) -> impl ControllerConfigApi + 'b {
        ControllerConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn content<'b>(&'b self, id: &'b str) -> impl ControllerContentApi + 'b {
        ControllerContentApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ControllerDescriptorsApi + 'b {
        ControllerDescriptorsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn details<'b>(&'b self, id: &'b str) -> impl ControllerDetailsApi + 'b {
        ControllerDetailsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl ControllerRunStatusApi + 'b {
        ControllerRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl ControllerStateApi + 'b {
        ControllerStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    async fn create_bulletin(
        &self,
        body: &types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_bulletin(body).await,
            Self::V2_7_2(api) => api.create_bulletin(body).await,
            Self::V2_8_0(api) => api.create_bulletin(body).await,
        }
    }
    async fn create_controller_service(
        &self,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_controller_service(body).await,
            Self::V2_7_2(api) => api.create_controller_service(body).await,
            Self::V2_8_0(api) => api.create_controller_service(body).await,
        }
    }
    async fn create_flow_analysis_rule(
        &self,
        body: &types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_flow_analysis_rule(body).await,
            Self::V2_7_2(api) => api.create_flow_analysis_rule(body).await,
            Self::V2_8_0(api) => api.create_flow_analysis_rule(body).await,
        }
    }
    async fn create_flow_registry_client(
        &self,
        body: &types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_flow_registry_client(body).await,
            Self::V2_7_2(api) => api.create_flow_registry_client(body).await,
            Self::V2_8_0(api) => api.create_flow_registry_client(body).await,
        }
    }
    async fn create_parameter_provider(
        &self,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_parameter_provider(body).await,
            Self::V2_7_2(api) => api.create_parameter_provider(body).await,
            Self::V2_8_0(api) => api.create_parameter_provider(body).await,
        }
    }
    async fn create_reporting_task(
        &self,
        body: &types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_reporting_task(body).await,
            Self::V2_7_2(api) => api.create_reporting_task(body).await,
            Self::V2_8_0(api) => api.create_reporting_task(body).await,
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
    async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_registry_client_types().await,
            Self::V2_7_2(api) => api.get_registry_client_types().await,
            Self::V2_8_0(api) => api.get_registry_client_types().await,
        }
    }
    async fn import_reporting_task_snapshot(
        &self,
        body: &types::VersionedReportingTaskImportRequestEntity,
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
    async fn update_controller_config(
        &self,
        body: &types::ControllerConfigurationEntity,
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
        body: &types::FlowAnalysisRuleEntity,
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
        body: &types::FlowRegistryClientEntity,
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
        body: &types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_node(id, body).await,
            Self::V2_7_2(api) => api.update_node(id, body).await,
            Self::V2_8_0(api) => api.update_node(id, body).await,
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
/// Sub-resource dispatch struct for [ControllerBulletinsApi].
pub struct ControllerBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerBulletinsApi for ControllerBulletinsApiDispatch<'_> {
    async fn clear_flow_analysis_rule_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_flow_analysis_rule_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_flow_analysis_rule_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_flow_analysis_rule_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_flow_analysis_rule_bulletins".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn clear_parameter_provider_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_parameter_provider_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_parameter_provider_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_parameter_provider_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_parameter_provider_bulletins".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn clear_registry_client_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_registry_client_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_registry_client_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_registry_client_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_registry_client_bulletins".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerConfigApi].
pub struct ControllerConfigApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerConfigApi for ControllerConfigApiDispatch<'_> {
    async fn analyze_flow_analysis_rule_configuration(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_flow_analysis_rule_configuration".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn analyze_flow_registry_client_configuration(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_flow_registry_client_configuration".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_flow_registry_client_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_flow_registry_client_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_flow_registry_client_configuration".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_flow_analysis_rule_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_registry_client_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .delete_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .delete_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_registry_client_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flow_analysis_rule_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_registry_client_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_registry_client_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_flow_analysis_rule_config_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn submit_registry_client_config_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_registry_client_config_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_registry_client_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .submit_registry_client_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_registry_client_config_verification_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerContentApi].
pub struct ControllerContentApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerContentApi for ControllerContentApiDispatch<'_> {
    async fn download_nar(&self) -> Result<(), NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerContentApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_nar().await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerContentApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_nar().await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerContentApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_nar().await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "download_nar".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerDescriptorsApi].
pub struct ControllerDescriptorsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerDescriptorsApi for ControllerDescriptorsApiDispatch<'_> {
    async fn get_flow_analysis_rule_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flow_analysis_rule_property_descriptor".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_property_descriptor".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerDetailsApi].
pub struct ControllerDetailsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerDetailsApi for ControllerDetailsApiDispatch<'_> {
    async fn get_nar_details(&self) -> Result<types::NarDetailsEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_nar_details().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_nar_details().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_nar_details().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_nar_details".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerRunStatusApi].
pub struct ControllerRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerRunStatusApi for ControllerRunStatusApiDispatch<'_> {
    async fn update_run_status(
        &self,
        body: &types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_6_0::types::FlowAnalysisRuleRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_8_0::types::FlowAnalysisRuleRunStatusEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_run_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [ControllerStateApi].
pub struct ControllerStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl ControllerStateApi for ControllerStateApiDispatch<'_> {
    async fn clear_state(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state(&crate::v2_6_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state(&crate::v2_7_2::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .clear_state(&crate::v2_8_0::types::ComponentStateEntity::try_from(
                        body.clone(),
                    )?)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_state".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_flow_analysis_rule_state(&self) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flow_analysis_rule_state".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

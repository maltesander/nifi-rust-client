// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerConfigApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerContentApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerDescriptorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerDetailsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ControllerApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ControllerApi for V2_8_0ControllerApi<'_> {
    type ControllerBulletinsApi<'b>
        = crate::dynamic::dispatch::ControllerBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ControllerBulletinsApi<'b> {
        crate::dynamic::dispatch::ControllerBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerConfigApi<'b>
        = crate::dynamic::dispatch::ControllerConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ControllerConfigApi<'b> {
        crate::dynamic::dispatch::ControllerConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerContentApi<'b>
        = crate::dynamic::dispatch::ControllerContentApiDispatch<'b>
    where
        Self: 'b;
    fn content<'b>(&'b self, id: &'b str) -> Self::ControllerContentApi<'b> {
        crate::dynamic::dispatch::ControllerContentApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerDescriptorsApi<'b>
        = crate::dynamic::dispatch::ControllerDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ControllerDescriptorsApi<'b> {
        crate::dynamic::dispatch::ControllerDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerDetailsApi<'b>
        = crate::dynamic::dispatch::ControllerDetailsApiDispatch<'b>
    where
        Self: 'b;
    fn details<'b>(&'b self, id: &'b str) -> Self::ControllerDetailsApi<'b> {
        crate::dynamic::dispatch::ControllerDetailsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerRunStatusApi<'b>
        = crate::dynamic::dispatch::ControllerRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ControllerRunStatusApi<'b> {
        crate::dynamic::dispatch::ControllerRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type ControllerStateApi<'b>
        = crate::dynamic::dispatch::ControllerStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ControllerStateApi<'b> {
        crate::dynamic::dispatch::ControllerStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn create_bulletin(
        &self,
        body: &types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_bulletin(&crate::v2_8_0::types::BulletinEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn create_controller_service(
        &self,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_controller_service(&crate::v2_8_0::types::ControllerServiceEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn create_flow_analysis_rule(
        &self,
        body: &types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_flow_analysis_rule(&crate::v2_8_0::types::FlowAnalysisRuleEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn create_flow_registry_client(
        &self,
        body: &types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_flow_registry_client(&crate::v2_8_0::types::FlowRegistryClientEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn create_parameter_provider(
        &self,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_parameter_provider(&crate::v2_8_0::types::ParameterProviderEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn create_reporting_task(
        &self,
        body: &types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_reporting_task(&crate::v2_8_0::types::ReportingTaskEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .delete_flow_registry_client(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_history(&self, end_date: &str) -> Result<types::HistoryDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.delete_history(end_date).await?.into())
    }
    async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .delete_nar(id, disconnected_node_acknowledged, force)
            .await?
            .into())
    }
    async fn delete_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.delete_node(id).await?.into())
    }
    async fn get_cluster(&self) -> Result<types::ClusterDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_cluster().await?.into())
    }
    async fn get_controller_config(
        &self,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_controller_config().await?.into())
    }
    async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_analysis_rule(id).await?.into())
    }
    async fn get_flow_analysis_rules(&self) -> Result<types::FlowAnalysisRulesEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_analysis_rules().await?.into())
    }
    async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_registry_client(id).await?.into())
    }
    async fn get_flow_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_registry_clients().await?.into())
    }
    async fn get_nar_summaries(&self) -> Result<types::NarSummariesEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_nar_summaries().await?.into())
    }
    async fn get_nar_summary(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_nar_summary(id).await?.into())
    }
    async fn get_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_node(id).await?.into())
    }
    async fn get_node_status_history(&self) -> Result<types::ComponentHistoryDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_node_status_history().await?.into())
    }
    async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_registry_client_types().await?.into())
    }
    async fn import_reporting_task_snapshot(
        &self,
        body: &types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<types::VersionedReportingTaskImportResponseEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .import_reporting_task_snapshot(
                &crate::v2_8_0::types::VersionedReportingTaskImportRequestEntity::try_from(
                    body.clone(),
                )?,
            )
            .await?
            .into())
    }
    async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .remove_flow_analysis_rule(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_controller_config(
        &self,
        body: &types::ControllerConfigurationEntity,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_controller_config(
                &crate::v2_8_0::types::ControllerConfigurationEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: &types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_flow_analysis_rule(
                id,
                &crate::v2_8_0::types::FlowAnalysisRuleEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn update_flow_registry_client(
        &self,
        id: &str,
        body: &types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_flow_registry_client(
                id,
                &crate::v2_8_0::types::FlowRegistryClientEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn update_node(
        &self,
        id: &str,
        body: &types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_node(
                id,
                &crate::v2_8_0::types::NodeEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        let api = crate::v2_8_0::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.upload_nar(filename, data).await?.into())
    }
}

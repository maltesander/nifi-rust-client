// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ControllerApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ControllerApi for V2_7_2ControllerApi<'_> {
    async fn analyze_flow_analysis_rule_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_flow_analysis_rule_configuration(
                &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn analyze_flow_registry_client_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_flow_registry_client_configuration(
                &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_flow_analysis_rule_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_flow_analysis_rule_bulletins(
                &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_parameter_provider_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_parameter_provider_bulletins(
                &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_registry_client_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_registry_client_bulletins(
                &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_state(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerStateApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_state(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_bulletin(
        &self,
        body: types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_bulletin(&crate::v2_7_2::types::BulletinEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_controller_service(
        &self,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_controller_service(&crate::v2_7_2::types::ControllerServiceEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_flow_analysis_rule(
        &self,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_flow_analysis_rule(&crate::v2_7_2::types::FlowAnalysisRuleEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_flow_registry_client(
        &self,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_flow_registry_client(&crate::v2_7_2::types::FlowRegistryClientEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_parameter_provider(
        &self,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_parameter_provider(&crate::v2_7_2::types::ParameterProviderEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn create_reporting_task(
        &self,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .create_reporting_task(&crate::v2_7_2::types::ReportingTaskEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn delete_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .delete_flow_analysis_rule_verification_request(request_id)
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
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .delete_flow_registry_client(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_history(&self, end_date: &str) -> Result<types::HistoryDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
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
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .delete_nar(id, disconnected_node_acknowledged, force)
            .await?
            .into())
    }
    async fn delete_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.delete_node(id).await?.into())
    }
    async fn delete_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .delete_registry_client_verification_request(request_id)
            .await?
            .into())
    }
    async fn download_nar(&self, id: &str) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerContentApi {
            client: self.client,
            id,
        };
        api.download_nar().await
    }
    async fn get_cluster(&self) -> Result<types::ClusterDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_cluster().await?.into())
    }
    async fn get_controller_config(
        &self,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_controller_config().await?.into())
    }
    async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_analysis_rule(id).await?.into())
    }
    async fn get_flow_analysis_rule_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
            .await?
            .into())
    }
    async fn get_flow_analysis_rule_state(
        &self,
        id: &str,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_flow_analysis_rule_state().await?.into())
    }
    async fn get_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_flow_analysis_rule_verification_request(request_id)
            .await?
            .into())
    }
    async fn get_flow_analysis_rules(&self) -> Result<types::FlowAnalysisRulesEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_analysis_rules().await?.into())
    }
    async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_registry_client(id).await?.into())
    }
    async fn get_flow_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_flow_registry_clients().await?.into())
    }
    async fn get_nar_details(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerDetailsApi {
            client: self.client,
            id,
        };
        Ok(api.get_nar_details().await?.into())
    }
    async fn get_nar_summaries(&self) -> Result<types::NarSummariesEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_nar_summaries().await?.into())
    }
    async fn get_nar_summary(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_nar_summary(id).await?.into())
    }
    async fn get_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_node(id).await?.into())
    }
    async fn get_node_status_history(&self) -> Result<types::ComponentHistoryDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_node_status_history().await?.into())
    }
    async fn get_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_property_descriptor(property_name, sensitive)
            .await?
            .into())
    }
    async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.get_registry_client_types().await?.into())
    }
    async fn get_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_registry_client_verification_request(request_id)
            .await?
            .into())
    }
    async fn import_reporting_task_snapshot(
        &self,
        body: types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<types::VersionedReportingTaskImportResponseEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .import_reporting_task_snapshot(
                &crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity::try_from(body)?,
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
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .remove_flow_analysis_rule(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_flow_analysis_rule_config_verification_request(
                &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn submit_registry_client_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_registry_client_config_verification_request(
                &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_controller_config(
        &self,
        body: types::ControllerConfigurationEntity,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_controller_config(
                &crate::v2_7_2::types::ControllerConfigurationEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_flow_analysis_rule(
                id,
                &crate::v2_7_2::types::FlowAnalysisRuleEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_flow_registry_client(
        &self,
        id: &str,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_flow_registry_client(
                id,
                &crate::v2_7_2::types::FlowRegistryClientEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_node(
        &self,
        id: &str,
        body: types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api
            .update_node(id, &crate::v2_7_2::types::NodeEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn update_run_status(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status(
                &crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        let api = crate::v2_7_2::api::controller::ControllerApi {
            client: self.client,
        };
        Ok(api.upload_nar(filename, data).await?.into())
    }
}

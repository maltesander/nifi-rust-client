// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ReportingTasksApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0ReportingTasksApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ReportingTasksApi for V2_6_0ReportingTasksApi<'_> {
    async fn analyze_configuration_3(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_configuration_3(
                &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_state_4(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_state_4(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn delete_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
            client: self.client,
            id,
        };
        Ok(api.delete_verification_request_3(request_id).await?.into())
    }
    async fn get_property_descriptor_4(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_property_descriptor_4(property_name, sensitive)
            .await?
            .into())
    }
    async fn get_reporting_task(&self, id: &str) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api.get_reporting_task(id).await?.into())
    }
    async fn get_state_4(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_state_4().await?.into())
    }
    async fn get_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
            client: self.client,
            id,
        };
        Ok(api.get_verification_request_3(request_id).await?.into())
    }
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api
            .remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn submit_config_verification_request_2(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_config_verification_request_2(
                &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_reporting_task(
        &self,
        id: &str,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api
            .update_reporting_task(
                id,
                &crate::v2_6_0::types::ReportingTaskEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_run_status_5(
        &self,
        id: &str,
        body: types::ReportingTaskRunStatusEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_6_0::api::reportingtasks::ReportingTasksRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status_5(
                &crate::v2_6_0::types::ReportingTaskRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

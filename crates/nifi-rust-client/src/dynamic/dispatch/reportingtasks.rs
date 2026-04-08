// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ReportingTasksApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ReportingTasks API. Use via the [`ReportingTasksApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ReportingTasksApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ReportingTasksApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ReportingTasksApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ReportingTasksApi<'a>),
}
impl ReportingTasksApi for ReportingTasksApiDispatch<'_> {
    async fn analyze_configuration_3(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.analyze_configuration_3(id, body).await,
            Self::V2_7_2(api) => api.analyze_configuration_3(id, body).await,
            Self::V2_8_0(api) => api.analyze_configuration_3(id, body).await,
        }
    }
    async fn clear_bulletins_7(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_7(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_7(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_7(id, body).await,
        }
    }
    async fn clear_state_4(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_state_4(id, body).await,
            Self::V2_7_2(api) => api.clear_state_4(id, body).await,
            Self::V2_8_0(api) => api.clear_state_4(id, body).await,
        }
    }
    async fn delete_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_verification_request_3(id, request_id).await,
            Self::V2_7_2(api) => api.delete_verification_request_3(id, request_id).await,
            Self::V2_8_0(api) => api.delete_verification_request_3(id, request_id).await,
        }
    }
    async fn get_property_descriptor_4(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_property_descriptor_4(id, property_name, sensitive)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_property_descriptor_4(id, property_name, sensitive)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_property_descriptor_4(id, property_name, sensitive)
                    .await
            }
        }
    }
    async fn get_reporting_task(&self, id: &str) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_reporting_task(id).await,
            Self::V2_7_2(api) => api.get_reporting_task(id).await,
            Self::V2_8_0(api) => api.get_reporting_task(id).await,
        }
    }
    async fn get_state_4(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_state_4(id).await,
            Self::V2_7_2(api) => api.get_state_4(id).await,
            Self::V2_8_0(api) => api.get_state_4(id).await,
        }
    }
    async fn get_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_verification_request_3(id, request_id).await,
            Self::V2_7_2(api) => api.get_verification_request_3(id, request_id).await,
            Self::V2_8_0(api) => api.get_verification_request_3(id, request_id).await,
        }
    }
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn submit_config_verification_request_2(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_config_verification_request_2(id, body).await,
            Self::V2_7_2(api) => api.submit_config_verification_request_2(id, body).await,
            Self::V2_8_0(api) => api.submit_config_verification_request_2(id, body).await,
        }
    }
    async fn update_reporting_task(
        &self,
        id: &str,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_reporting_task(id, body).await,
            Self::V2_7_2(api) => api.update_reporting_task(id, body).await,
            Self::V2_8_0(api) => api.update_reporting_task(id, body).await,
        }
    }
    async fn update_run_status_5(
        &self,
        id: &str,
        body: types::ReportingTaskRunStatusEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_run_status_5(id, body).await,
            Self::V2_7_2(api) => api.update_run_status_5(id, body).await,
            Self::V2_8_0(api) => api.update_run_status_5(id, body).await,
        }
    }
}

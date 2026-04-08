// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessorsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Processors API. Use via the [`ProcessorsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ProcessorsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ProcessorsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ProcessorsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ProcessorsApi<'a>),
}
impl ProcessorsApi for ProcessorsApiDispatch<'_> {
    async fn analyze_configuration_2(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.analyze_configuration_2(id, body).await,
            Self::V2_7_2(api) => api.analyze_configuration_2(id, body).await,
            Self::V2_8_0(api) => api.analyze_configuration_2(id, body).await,
        }
    }
    async fn clear_bulletins_5(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_5(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_5(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_5(id, body).await,
        }
    }
    async fn clear_state_3(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_state_3(id, body).await,
            Self::V2_7_2(api) => api.clear_state_3(id, body).await,
            Self::V2_8_0(api) => api.clear_state_3(id, body).await,
        }
    }
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn delete_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_verification_request_2(id, request_id).await,
            Self::V2_7_2(api) => api.delete_verification_request_2(id, request_id).await,
            Self::V2_8_0(api) => api.delete_verification_request_2(id, request_id).await,
        }
    }
    async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor(id).await,
            Self::V2_7_2(api) => api.get_processor(id).await,
            Self::V2_8_0(api) => api.get_processor(id).await,
        }
    }
    async fn get_processor_diagnostics(
        &self,
        id: &str,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor_diagnostics(id).await,
            Self::V2_7_2(api) => api.get_processor_diagnostics(id).await,
            Self::V2_8_0(api) => api.get_processor_diagnostics(id).await,
        }
    }
    async fn get_processor_run_status_details(
        &self,
        body: types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_processor_run_status_details(body).await,
            Self::V2_7_2(api) => api.get_processor_run_status_details(body).await,
            Self::V2_8_0(api) => api.get_processor_run_status_details(body).await,
        }
    }
    async fn get_property_descriptor_3(
        &self,
        id: &str,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_property_descriptor_3(id, client_id, property_name, sensitive)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_property_descriptor_3(id, client_id, property_name, sensitive)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_property_descriptor_3(id, client_id, property_name, sensitive)
                    .await
            }
        }
    }
    async fn get_state_2(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_state_2(id).await,
            Self::V2_7_2(api) => api.get_state_2(id).await,
            Self::V2_8_0(api) => api.get_state_2(id).await,
        }
    }
    async fn get_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_verification_request_2(id, request_id).await,
            Self::V2_7_2(api) => api.get_verification_request_2(id, request_id).await,
            Self::V2_8_0(api) => api.get_verification_request_2(id, request_id).await,
        }
    }
    async fn submit_processor_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_processor_verification_request(id, body).await,
            Self::V2_7_2(api) => api.submit_processor_verification_request(id, body).await,
            Self::V2_8_0(api) => api.submit_processor_verification_request(id, body).await,
        }
    }
    async fn terminate_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.terminate_processor(id).await,
            Self::V2_7_2(api) => api.terminate_processor(id).await,
            Self::V2_8_0(api) => api.terminate_processor(id).await,
        }
    }
    async fn update_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_processor(id, body).await,
            Self::V2_7_2(api) => api.update_processor(id, body).await,
            Self::V2_8_0(api) => api.update_processor(id, body).await,
        }
    }
    async fn update_run_status_4(
        &self,
        id: &str,
        body: types::ProcessorRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_run_status_4(id, body).await,
            Self::V2_7_2(api) => api.update_run_status_4(id, body).await,
            Self::V2_8_0(api) => api.update_run_status_4(id, body).await,
        }
    }
}

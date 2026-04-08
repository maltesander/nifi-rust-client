// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerServicesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Controller Services API. Use via the [`ControllerServicesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ControllerServicesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ControllerServicesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ControllerServicesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ControllerServicesApi<'a>),
}
impl ControllerServicesApi for ControllerServicesApiDispatch<'_> {
    async fn analyze_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.analyze_configuration(id, body).await,
            Self::V2_7_2(api) => api.analyze_configuration(id, body).await,
            Self::V2_8_0(api) => api.analyze_configuration(id, body).await,
        }
    }
    async fn clear_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins(id, body).await,
        }
    }
    async fn clear_state_1(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_state_1(id, body).await,
            Self::V2_7_2(api) => api.clear_state_1(id, body).await,
            Self::V2_8_0(api) => api.clear_state_1(id, body).await,
        }
    }
    async fn delete_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_verification_request(id, request_id).await,
            Self::V2_7_2(api) => api.delete_verification_request(id, request_id).await,
            Self::V2_8_0(api) => api.delete_verification_request(id, request_id).await,
        }
    }
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_controller_service(id, ui_only).await,
            Self::V2_7_2(api) => api.get_controller_service(id, ui_only).await,
            Self::V2_8_0(api) => api.get_controller_service(id, ui_only).await,
        }
    }
    async fn get_controller_service_references(
        &self,
        id: &str,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_controller_service_references(id).await,
            Self::V2_7_2(api) => api.get_controller_service_references(id).await,
            Self::V2_8_0(api) => api.get_controller_service_references(id).await,
        }
    }
    async fn get_property_descriptor_1(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_property_descriptor_1(id, property_name, sensitive)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_property_descriptor_1(id, property_name, sensitive)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_property_descriptor_1(id, property_name, sensitive)
                    .await
            }
        }
    }
    async fn get_state(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_state(id).await,
            Self::V2_7_2(api) => api.get_state(id).await,
            Self::V2_8_0(api) => api.get_state(id).await,
        }
    }
    async fn get_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_verification_request(id, request_id).await,
            Self::V2_7_2(api) => api.get_verification_request(id, request_id).await,
            Self::V2_8_0(api) => api.get_verification_request(id, request_id).await,
        }
    }
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.remove_controller_service(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn submit_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_config_verification_request(id, body).await,
            Self::V2_7_2(api) => api.submit_config_verification_request(id, body).await,
            Self::V2_8_0(api) => api.submit_config_verification_request(id, body).await,
        }
    }
    async fn update_controller_service(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_controller_service(id, body).await,
            Self::V2_7_2(api) => api.update_controller_service(id, body).await,
            Self::V2_8_0(api) => api.update_controller_service(id, body).await,
        }
    }
    async fn update_controller_service_references(
        &self,
        id: &str,
        body: types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_controller_service_references(id, body).await,
            Self::V2_7_2(api) => api.update_controller_service_references(id, body).await,
            Self::V2_8_0(api) => api.update_controller_service_references(id, body).await,
        }
    }
    async fn update_run_status_1(
        &self,
        id: &str,
        body: types::ControllerServiceRunStatusEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_run_status_1(id, body).await,
            Self::V2_7_2(api) => api.update_run_status_1(id, body).await,
            Self::V2_8_0(api) => api.update_run_status_1(id, body).await,
        }
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerServicesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ControllerServicesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ControllerServicesApi for V2_7_2ControllerServicesApi<'_> {
    async fn analyze_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_configuration(
                &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesBulletinsApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_bulletins(&crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn clear_state_1(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_state_1(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn delete_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
            client: self.client,
            id,
        };
        Ok(api.delete_verification_request(request_id).await?.into())
    }
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api.get_controller_service(id, ui_only).await?.into())
    }
    async fn get_controller_service_references(
        &self,
        id: &str,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
            client: self.client,
            id,
        };
        Ok(api.get_controller_service_references().await?.into())
    }
    async fn get_property_descriptor_1(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_property_descriptor_1(property_name, sensitive)
            .await?
            .into())
    }
    async fn get_state(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_state().await?.into())
    }
    async fn get_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
            client: self.client,
            id,
        };
        Ok(api.get_verification_request(request_id).await?.into())
    }
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api
            .remove_controller_service(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn submit_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_config_verification_request(
                &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_controller_service(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api
            .update_controller_service(
                id,
                &crate::v2_7_2::types::ControllerServiceEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_controller_service_references(
        &self,
        id: &str,
        body: types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_controller_service_references(
                &crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                    body,
                )?,
            )
            .await?
            .into())
    }
    async fn update_run_status_1(
        &self,
        id: &str,
        body: types::ControllerServiceRunStatusEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_7_2::api::controller_services::ControllerServicesRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status_1(
                &crate::v2_7_2::types::ControllerServiceRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

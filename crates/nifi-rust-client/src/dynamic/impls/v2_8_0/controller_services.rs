// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ControllerServicesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesConfigApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesDescriptorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesReferencesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ControllerServicesStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ControllerServicesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ControllerServicesApi for V2_8_0ControllerServicesApi<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ControllerServicesBulletinsApi + 'b {
        crate::dynamic::dispatch::ControllerServicesBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn config<'b>(&'b self, id: &'b str) -> impl ControllerServicesConfigApi + 'b {
        crate::dynamic::dispatch::ControllerServicesConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ControllerServicesDescriptorsApi + 'b {
        crate::dynamic::dispatch::ControllerServicesDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn references<'b>(&'b self, id: &'b str) -> impl ControllerServicesReferencesApi + 'b {
        crate::dynamic::dispatch::ControllerServicesReferencesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl ControllerServicesRunStatusApi + 'b {
        crate::dynamic::dispatch::ControllerServicesRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl ControllerServicesStateApi + 'b {
        crate::dynamic::dispatch::ControllerServicesStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api.get_controller_service(id, ui_only).await?.into())
    }
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api
            .remove_controller_service(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_controller_service(
        &self,
        id: &str,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api
            .update_controller_service(
                id,
                &crate::v2_8_0::types::ControllerServiceEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

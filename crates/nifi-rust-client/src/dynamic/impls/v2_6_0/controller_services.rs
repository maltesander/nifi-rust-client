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
pub(crate) struct V2_6_0ControllerServicesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ControllerServicesApi for V2_6_0ControllerServicesApi<'_> {
    type ControllerServicesBulletinsApi<'b>
        = crate::dynamic::dispatch::ControllerServicesBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ControllerServicesBulletinsApi<'b> {
        crate::dynamic::dispatch::ControllerServicesBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    type ControllerServicesConfigApi<'b>
        = crate::dynamic::dispatch::ControllerServicesConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ControllerServicesConfigApi<'b> {
        crate::dynamic::dispatch::ControllerServicesConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    type ControllerServicesDescriptorsApi<'b>
        = crate::dynamic::dispatch::ControllerServicesDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ControllerServicesDescriptorsApi<'b> {
        crate::dynamic::dispatch::ControllerServicesDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    type ControllerServicesReferencesApi<'b>
        = crate::dynamic::dispatch::ControllerServicesReferencesApiDispatch<'b>
    where
        Self: 'b;
    fn references<'b>(&'b self, id: &'b str) -> Self::ControllerServicesReferencesApi<'b> {
        crate::dynamic::dispatch::ControllerServicesReferencesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    type ControllerServicesRunStatusApi<'b>
        = crate::dynamic::dispatch::ControllerServicesRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ControllerServicesRunStatusApi<'b> {
        crate::dynamic::dispatch::ControllerServicesRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    type ControllerServicesStateApi<'b>
        = crate::dynamic::dispatch::ControllerServicesStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ControllerServicesStateApi<'b> {
        crate::dynamic::dispatch::ControllerServicesStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_6_0,
        }
    }
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
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
        let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
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
        let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
            client: self.client,
        };
        Ok(api
            .update_controller_service(
                id,
                &crate::v2_6_0::types::ControllerServiceEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

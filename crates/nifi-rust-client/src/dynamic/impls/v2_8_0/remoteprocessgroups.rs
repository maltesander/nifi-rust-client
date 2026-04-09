// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::RemoteProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::RemoteProcessGroupsBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::RemoteProcessGroupsInputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::RemoteProcessGroupsOutputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::RemoteProcessGroupsRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::traits::RemoteProcessGroupsStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0RemoteProcessGroupsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl RemoteProcessGroupsApi for V2_8_0RemoteProcessGroupsApi<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsBulletinsApi + 'b {
        crate::dynamic::dispatch::RemoteProcessGroupsBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn input_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsInputPortsApi + 'b {
        crate::dynamic::dispatch::RemoteProcessGroupsInputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn output_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsOutputPortsApi + 'b {
        crate::dynamic::dispatch::RemoteProcessGroupsOutputPortsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsRunStatusApi + 'b {
        crate::dynamic::dispatch::RemoteProcessGroupsRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsStateApi + 'b {
        crate::dynamic::dispatch::RemoteProcessGroupsStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_remote_process_group(id).await?.into())
    }
    async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .remove_remote_process_group(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_remote_process_group(
        &self,
        id: &str,
        body: &types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .update_remote_process_group(
                id,
                &crate::v2_8_0::types::RemoteProcessGroupEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

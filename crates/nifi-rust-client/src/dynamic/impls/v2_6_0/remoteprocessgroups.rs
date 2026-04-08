// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::RemoteProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0RemoteProcessGroupsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl RemoteProcessGroupsApi for V2_6_0RemoteProcessGroupsApi<'_> {
    async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
            client: self.client,
        };
        Ok(api.get_remote_process_group(id).await?.into())
    }
    async fn get_state_3(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_state_3().await?.into())
    }
    async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
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
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
            client: self.client,
        };
        Ok(api
            .update_remote_process_group(
                id,
                &crate::v2_6_0::types::RemoteProcessGroupEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_input_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_input_port(
                port_id,
                &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_input_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_input_port_run_status(
                port_id,
                &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_output_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_output_port(
                port_id,
                &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_output_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_output_port_run_status(
                port_id,
                &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_run_status(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_run_status(
                &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn update_remote_process_group_run_statuses(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_remote_process_group_run_statuses(
                &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

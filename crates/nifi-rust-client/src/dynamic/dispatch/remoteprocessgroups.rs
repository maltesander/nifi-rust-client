// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::RemoteProcessGroupsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the RemoteProcessGroups API. Use via the [`RemoteProcessGroupsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum RemoteProcessGroupsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0RemoteProcessGroupsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2RemoteProcessGroupsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0RemoteProcessGroupsApi<'a>),
}
impl RemoteProcessGroupsApi for RemoteProcessGroupsApiDispatch<'_> {
    async fn clear_bulletins_6(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.clear_bulletins_6(id, body).await,
            Self::V2_7_2(api) => api.clear_bulletins_6(id, body).await,
            Self::V2_8_0(api) => api.clear_bulletins_6(id, body).await,
        }
    }
    async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_remote_process_group(id).await,
            Self::V2_7_2(api) => api.get_remote_process_group(id).await,
            Self::V2_8_0(api) => api.get_remote_process_group(id).await,
        }
    }
    async fn get_state_3(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_state_3(id).await,
            Self::V2_7_2(api) => api.get_state_3(id).await,
            Self::V2_8_0(api) => api.get_state_3(id).await,
        }
    }
    async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_remote_process_group(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_7_2(api) => {
                api.remove_remote_process_group(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
            Self::V2_8_0(api) => {
                api.remove_remote_process_group(
                    id,
                    version,
                    client_id,
                    disconnected_node_acknowledged,
                )
                .await
            }
        }
    }
    async fn update_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_remote_process_group(id, body).await,
            Self::V2_7_2(api) => api.update_remote_process_group(id, body).await,
            Self::V2_8_0(api) => api.update_remote_process_group(id, body).await,
        }
    }
    async fn update_remote_process_group_input_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.update_remote_process_group_input_port(id, port_id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.update_remote_process_group_input_port(id, port_id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.update_remote_process_group_input_port(id, port_id, body)
                    .await
            }
        }
    }
    async fn update_remote_process_group_input_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.update_remote_process_group_input_port_run_status(id, port_id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.update_remote_process_group_input_port_run_status(id, port_id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.update_remote_process_group_input_port_run_status(id, port_id, body)
                    .await
            }
        }
    }
    async fn update_remote_process_group_output_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.update_remote_process_group_output_port(id, port_id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.update_remote_process_group_output_port(id, port_id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.update_remote_process_group_output_port(id, port_id, body)
                    .await
            }
        }
    }
    async fn update_remote_process_group_output_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.update_remote_process_group_output_port_run_status(id, port_id, body)
                    .await
            }
            Self::V2_7_2(api) => {
                api.update_remote_process_group_output_port_run_status(id, port_id, body)
                    .await
            }
            Self::V2_8_0(api) => {
                api.update_remote_process_group_output_port_run_status(id, port_id, body)
                    .await
            }
        }
    }
    async fn update_remote_process_group_run_status(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_remote_process_group_run_status(id, body).await,
            Self::V2_7_2(api) => api.update_remote_process_group_run_status(id, body).await,
            Self::V2_8_0(api) => api.update_remote_process_group_run_status(id, body).await,
        }
    }
    async fn update_remote_process_group_run_statuses(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_remote_process_group_run_statuses(id, body).await,
            Self::V2_7_2(api) => api.update_remote_process_group_run_statuses(id, body).await,
            Self::V2_8_0(api) => api.update_remote_process_group_run_statuses(id, body).await,
        }
    }
}

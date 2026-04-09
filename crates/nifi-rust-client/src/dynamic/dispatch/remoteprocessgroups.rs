// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::RemoteProcessGroupsApi;
use crate::dynamic::traits::RemoteProcessGroupsBulletinsApi;
use crate::dynamic::traits::RemoteProcessGroupsInputPortsApi;
use crate::dynamic::traits::RemoteProcessGroupsOutputPortsApi;
use crate::dynamic::traits::RemoteProcessGroupsRunStatusApi;
use crate::dynamic::traits::RemoteProcessGroupsStateApi;
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
impl<'a> RemoteProcessGroupsApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl RemoteProcessGroupsApi for RemoteProcessGroupsApiDispatch<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsBulletinsApi + 'b {
        RemoteProcessGroupsBulletinsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn input_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsInputPortsApi + 'b {
        RemoteProcessGroupsInputPortsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn output_ports<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsOutputPortsApi + 'b {
        RemoteProcessGroupsOutputPortsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsRunStatusApi + 'b {
        RemoteProcessGroupsRunStatusApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl RemoteProcessGroupsStateApi + 'b {
        RemoteProcessGroupsStateApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
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
        body: &types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_remote_process_group(id, body).await,
            Self::V2_7_2(api) => api.update_remote_process_group(id, body).await,
            Self::V2_8_0(api) => api.update_remote_process_group(id, body).await,
        }
    }
}
/// Sub-resource dispatch struct for [RemoteProcessGroupsBulletinsApi].
pub struct RemoteProcessGroupsBulletinsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl RemoteProcessGroupsBulletinsApi for RemoteProcessGroupsBulletinsApiDispatch<'_> {
    async fn clear_bulletins_6(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_6".to_string(),
                version: "2.6.0".to_string(),
            }),
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsBulletinsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .clear_bulletins_6(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsBulletinsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .clear_bulletins_6(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_6".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [RemoteProcessGroupsInputPortsApi].
pub struct RemoteProcessGroupsInputPortsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl RemoteProcessGroupsInputPortsApi for RemoteProcessGroupsInputPortsApiDispatch<'_> {
    async fn update_remote_process_group_input_port(
        &self,
        port_id: &str,
        body: &types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_7_2::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_8_0::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_input_port".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn update_remote_process_group_input_port_run_status(
        &self,
        port_id: &str,
        body: &types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_input_port_run_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [RemoteProcessGroupsOutputPortsApi].
pub struct RemoteProcessGroupsOutputPortsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl RemoteProcessGroupsOutputPortsApi for RemoteProcessGroupsOutputPortsApiDispatch<'_> {
    async fn update_remote_process_group_output_port(
        &self,
        port_id: &str,
        body: &types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_7_2::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_8_0::types::RemoteProcessGroupPortEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_output_port".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn update_remote_process_group_output_port_run_status(
        &self,
        port_id: &str,
        body: &types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_output_port_run_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [RemoteProcessGroupsRunStatusApi].
pub struct RemoteProcessGroupsRunStatusApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl RemoteProcessGroupsRunStatusApi for RemoteProcessGroupsRunStatusApiDispatch<'_> {
    async fn update_remote_process_group_run_status(
        &self,
        body: &types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_run_status".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn update_remote_process_group_run_statuses(
        &self,
        body: &types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id: &self.id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body.clone())?,
                    )
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "update_remote_process_group_run_statuses".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [RemoteProcessGroupsStateApi].
pub struct RemoteProcessGroupsStateApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl RemoteProcessGroupsStateApi for RemoteProcessGroupsStateApiDispatch<'_> {
    async fn get_state_3(&self) -> Result<types::ComponentStateDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_3().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_3().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_state_3().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_state_3".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

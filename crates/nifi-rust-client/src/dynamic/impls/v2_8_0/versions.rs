// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::VersionsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::VersionsDownloadApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0VersionsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl VersionsApi for V2_8_0VersionsApi<'_> {
    fn download<'b>(&'b self, id: &'b str) -> impl VersionsDownloadApi + 'b {
        crate::dynamic::dispatch::VersionsDownloadApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn create_version_control_request(
        &self,
        body: &types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        api.create_version_control_request(
            &crate::v2_8_0::types::CreateActiveRequestEntity::try_from(body.clone())?,
        )
        .await
    }
    async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .delete_revert_request(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .delete_update_request_1(id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        api.delete_version_control_request(id, disconnected_node_acknowledged)
            .await
    }
    async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api.get_revert_request(id).await?.into())
    }
    async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api.get_update_request(id).await?.into())
    }
    async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api.get_version_information(id).await?.into())
    }
    async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: &types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .initiate_revert_flow_version(
                id,
                &crate::v2_8_0::types::VersionControlInformationEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn initiate_version_control_update(
        &self,
        id: &str,
        body: &types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .initiate_version_control_update(
                id,
                &crate::v2_8_0::types::VersionControlInformationEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn save_to_flow_registry(
        &self,
        id: &str,
        body: &types::StartVersionControlRequestEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .save_to_flow_registry(
                id,
                &crate::v2_8_0::types::StartVersionControlRequestEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .stop_version_control(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_flow_version(
        &self,
        id: &str,
        body: &types::VersionedFlowSnapshotEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .update_flow_version(
                id,
                &crate::v2_8_0::types::VersionedFlowSnapshotEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn update_version_control_request(
        &self,
        id: &str,
        body: &types::VersionControlComponentMappingEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        let api = crate::v2_8_0::api::versions::VersionsApi {
            client: self.client,
        };
        Ok(api
            .update_version_control_request(
                id,
                &crate::v2_8_0::types::VersionControlComponentMappingEntity::try_from(
                    body.clone(),
                )?,
            )
            .await?
            .into())
    }
}

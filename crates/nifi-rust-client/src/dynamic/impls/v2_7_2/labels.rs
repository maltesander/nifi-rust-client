// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::LabelsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2LabelsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl LabelsApi for V2_7_2LabelsApi<'_> {
    async fn get_label(&self, id: &str) -> Result<types::LabelEntity, NifiError> {
        let api = crate::v2_7_2::api::labels::LabelsApi {
            client: self.client,
        };
        Ok(api.get_label(id).await?.into())
    }
    async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::LabelEntity, NifiError> {
        let api = crate::v2_7_2::api::labels::LabelsApi {
            client: self.client,
        };
        Ok(api
            .remove_label(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_label(
        &self,
        id: &str,
        body: &types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        let api = crate::v2_7_2::api::labels::LabelsApi {
            client: self.client,
        };
        Ok(api
            .update_label(
                id,
                &crate::v2_7_2::types::LabelEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

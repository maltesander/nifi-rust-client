// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ConnectionsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ConnectionsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ConnectionsApi for V2_8_0ConnectionsApi<'_> {
    async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ConnectionEntity, NifiError> {
        let api = crate::v2_8_0::api::connections::ConnectionsApi {
            client: self.client,
        };
        Ok(api
            .delete_connection(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_connection(&self, id: &str) -> Result<types::ConnectionEntity, NifiError> {
        let api = crate::v2_8_0::api::connections::ConnectionsApi {
            client: self.client,
        };
        Ok(api.get_connection(id).await?.into())
    }
    async fn update_connection(
        &self,
        id: &str,
        body: &types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        let api = crate::v2_8_0::api::connections::ConnectionsApi {
            client: self.client,
        };
        Ok(api
            .update_connection(
                id,
                &crate::v2_8_0::types::ConnectionEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

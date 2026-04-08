// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FunnelsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2FunnelsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl FunnelsApi for V2_7_2FunnelsApi<'_> {
    async fn get_funnel(&self, id: &str) -> Result<types::FunnelEntity, NifiError> {
        let api = crate::v2_7_2::api::funnels::FunnelsApi {
            client: self.client,
        };
        Ok(api.get_funnel(id).await?.into())
    }
    async fn remove_funnel(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FunnelEntity, NifiError> {
        let api = crate::v2_7_2::api::funnels::FunnelsApi {
            client: self.client,
        };
        Ok(api
            .remove_funnel(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        let api = crate::v2_7_2::api::funnels::FunnelsApi {
            client: self.client,
        };
        Ok(api
            .update_funnel(id, &crate::v2_7_2::types::FunnelEntity::try_from(body)?)
            .await?
            .into())
    }
}

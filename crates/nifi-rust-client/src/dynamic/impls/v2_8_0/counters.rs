// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::CountersApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0CountersApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl CountersApi for V2_8_0CountersApi<'_> {
    async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::CountersDto, NifiError> {
        let api = crate::v2_8_0::api::counters::CountersApi {
            client: self.client,
        };
        Ok(api.get_counters(nodewise, cluster_node_id).await?.into())
    }
    async fn update_all_counters(&self) -> Result<types::CountersDto, NifiError> {
        let api = crate::v2_8_0::api::counters::CountersApi {
            client: self.client,
        };
        Ok(api.update_all_counters().await?.into())
    }
    async fn update_counter(&self, id: &str) -> Result<types::CounterDto, NifiError> {
        let api = crate::v2_8_0::api::counters::CountersApi {
            client: self.client,
        };
        Ok(api.update_counter(id).await?.into())
    }
}

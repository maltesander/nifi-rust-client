// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct CountersApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> CountersApi<'a> {
    /// Gets the current counters for this NiFi
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/counters`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    pub async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::CountersDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_8_0::types::CountersEntity =
            self.client.get_with_query("/counters", &query).await?;
        Ok(e.counters)
    }
    /// Updates all counters. This will reset all counter values to 0
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/counters`.
    pub async fn update_all_counters(
        &self,
    ) -> Result<crate::v2_8_0::types::CountersDto, NifiError> {
        let e: crate::v2_8_0::types::CountersEntity = self.client.put_no_body("/counters").await?;
        Ok(e.counters)
    }
    /// Updates the specified counter. This will reset the counter value to 0
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/counters/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the counter.
    pub async fn update_counter(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::CounterDto, NifiError> {
        let e: crate::v2_8_0::types::CounterEntity =
            self.client.put_no_body(&format!("/counters/{id}")).await?;
        Ok(e.counter)
    }
}

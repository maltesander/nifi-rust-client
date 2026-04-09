// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Counters API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait CountersApi {
    /// Gets the current counters for this NiFi
    async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_7_2::types::CountersDto, NifiError>;
    /// Updates all counters. This will reset all counter values to 0
    async fn update_all_counters(&self) -> Result<crate::v2_7_2::types::CountersDto, NifiError>;
    /// Updates the specified counter. This will reset the counter value to 0
    async fn update_counter(&self, id: &str)
    -> Result<crate::v2_7_2::types::CounterDto, NifiError>;
}

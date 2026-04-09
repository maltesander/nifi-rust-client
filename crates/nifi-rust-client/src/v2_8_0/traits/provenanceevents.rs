// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `content` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceEventsContentApi {
    /// Gets the input content for a provenance event
    async fn get_input_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError>;
    /// Gets the output content for a provenance event
    async fn get_output_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError>;
}
/// The ProvenanceEvents API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceEventsApi {
    type ProvenanceEventsContentApi<'b>: ProvenanceEventsContentApi
    where
        Self: 'b;
    fn content<'b>(&'b self, id: &'b str) -> Self::ProvenanceEventsContentApi<'b>;
    /// Replays content from a provenance event
    async fn submit_replay_latest_event(
        &self,
        body: &crate::v2_8_0::types::ReplayLastEventRequestEntity,
    ) -> Result<crate::v2_8_0::types::ReplayLastEventResponseEntity, NifiError>;
    /// Retrieves the latest cached Provenance Events for the specified component
    async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<crate::v2_8_0::types::LatestProvenanceEventsDto, NifiError>;
    /// Replays content from a provenance event
    async fn submit_replay(
        &self,
        body: &crate::v2_8_0::types::SubmitReplayRequestEntity,
    ) -> Result<crate::v2_8_0::types::ProvenanceEventDto, NifiError>;
    /// Gets a provenance event
    async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::ProvenanceEventDto, NifiError>;
}

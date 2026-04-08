// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProvenanceEventsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the ProvenanceEvents API. Use via the [`ProvenanceEventsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ProvenanceEventsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ProvenanceEventsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ProvenanceEventsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ProvenanceEventsApi<'a>),
}
impl ProvenanceEventsApi for ProvenanceEventsApiDispatch<'_> {
    async fn get_input_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_input_content(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_input_content(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_input_content(id, cluster_node_id).await,
        }
    }
    async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<types::LatestProvenanceEventsDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_latest_provenance_events(component_id, limit).await,
            Self::V2_7_2(api) => api.get_latest_provenance_events(component_id, limit).await,
            Self::V2_8_0(api) => api.get_latest_provenance_events(component_id, limit).await,
        }
    }
    async fn get_output_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_output_content(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_output_content(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_output_content(id, cluster_node_id).await,
        }
    }
    async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_provenance_event(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_provenance_event(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_provenance_event(id, cluster_node_id).await,
        }
    }
    async fn submit_replay(
        &self,
        body: types::SubmitReplayRequestEntity,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_replay(body).await,
            Self::V2_7_2(api) => api.submit_replay(body).await,
            Self::V2_8_0(api) => api.submit_replay(body).await,
        }
    }
    async fn submit_replay_latest_event(
        &self,
        body: types::ReplayLastEventRequestEntity,
    ) -> Result<types::ReplayLastEventResponseEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_replay_latest_event(body).await,
            Self::V2_7_2(api) => api.submit_replay_latest_event(body).await,
            Self::V2_8_0(api) => api.submit_replay_latest_event(body).await,
        }
    }
}

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsDto {
    pub component_id: Option<String>,
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsEntity {
    pub latest_provenance_events: LatestProvenanceEventsDto,
}
/// The provenance events that matched the search criteria.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventDto {
    /// The alternate identifier uri for the fileflow for the event.
    pub alternate_identifier_uri: Option<String>,
    /// The attributes of the flowfile for the event.
    pub attributes: Option<Vec<AttributeDto>>,
    /// The child uuids for the event.
    pub child_uuids: Option<Vec<String>>,
    /// The label for the node where the event originated.
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the event originated.
    pub cluster_node_id: Option<String>,
    /// The id of the component that generated the event.
    pub component_id: Option<String>,
    /// The name of the component that generated the event.
    pub component_name: Option<String>,
    /// The type of the component that generated the event.
    pub component_type: Option<String>,
    /// Whether the input and output content claim is the same.
    pub content_equal: Option<bool>,
    /// The event details.
    pub details: Option<String>,
    /// The event duration in milliseconds.
    pub event_duration: Option<i64>,
    /// The event id. This is a one up number thats unique per node.
    pub event_id: Option<i64>,
    /// The timestamp of the event.
    pub event_time: Option<String>,
    /// Event Timestamp formatted using ISO8601
    pub event_timestamp: Option<String>,
    /// The type of the event.
    pub event_type: Option<String>,
    /// The size of the flowfile for the event.
    pub file_size: Option<String>,
    /// The size of the flowfile in bytes for the event.
    pub file_size_bytes: Option<i64>,
    /// The uuid of the flowfile for the event.
    pub flow_file_uuid: Option<String>,
    /// The id of the group that the component resides in. If the component is no longer in the flow, the group id will not be set.
    pub group_id: Option<String>,
    /// The event uuid.
    pub id: Option<String>,
    /// Whether the input content is still available.
    pub input_content_available: Option<bool>,
    /// The container in which the input content claim lives.
    pub input_content_claim_container: Option<String>,
    /// The file size of the input content claim formatted.
    pub input_content_claim_file_size: Option<String>,
    /// The file size of the intput content claim in bytes.
    pub input_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the input content claim.
    pub input_content_claim_identifier: Option<String>,
    /// The offset into the input content claim where the flowfiles content begins.
    pub input_content_claim_offset: Option<i64>,
    /// The section in which the input content claim lives.
    pub input_content_claim_section: Option<String>,
    /// The duration since the lineage began, in milliseconds.
    pub lineage_duration: Option<i64>,
    /// Whether the output content is still available.
    pub output_content_available: Option<bool>,
    /// The container in which the output content claim lives.
    pub output_content_claim_container: Option<String>,
    /// The file size of the output content claim formatted.
    pub output_content_claim_file_size: Option<String>,
    /// The file size of the output content claim in bytes.
    pub output_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the output content claim.
    pub output_content_claim_identifier: Option<String>,
    /// The offset into the output content claim where the flowfiles content begins.
    pub output_content_claim_offset: Option<i64>,
    /// The section in which the output content claim lives.
    pub output_content_claim_section: Option<String>,
    /// The parent uuids for the event.
    pub parent_uuids: Option<Vec<String>>,
    /// The relationship to which the flowfile was routed if the event is of type ROUTE.
    pub relationship: Option<String>,
    /// Whether or not replay is available.
    pub replay_available: Option<bool>,
    /// Explanation as to why replay is unavailable.
    pub replay_explanation: Option<String>,
    /// The identifier of the queue/connection from which the flowfile was pulled to genereate this event. May be null if the queue/connection is unknown or the flowfile was generated from this event.
    pub source_connection_identifier: Option<String>,
    /// The source system flowfile id.
    pub source_system_flow_file_id: Option<String>,
    /// The source/destination system uri if the event was a RECEIVE/SEND.
    pub transit_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventEntity {
    pub provenance_event: ProvenanceEventDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReplayLastEventRequestEntityNodes {
    #[default]
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "PRIMARY")]
    Primary,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventRequestEntity {
    /// The UUID of the component whose last event should be replayed.
    pub component_id: Option<String>,
    /// Which nodes are to replay their last provenance event.
    pub nodes: Option<ReplayLastEventRequestEntityNodes>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReplayLastEventResponseEntityNodes {
    #[default]
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "PRIMARY")]
    Primary,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventResponseEntity {
    pub aggregate_snapshot: Option<ReplayLastEventSnapshotDto>,
    /// The UUID of the component whose last event should be replayed.
    pub component_id: Option<String>,
    /// The node-wise results
    pub node_snapshots: Option<Vec<NodeReplayLastEventSnapshotDto>>,
    /// Which nodes were requested to replay their last provenance event.
    pub nodes: Option<ReplayLastEventResponseEntityNodes>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReplayRequestEntity {
    /// The identifier of the node where to submit the replay request.
    pub cluster_node_id: Option<String>,
    /// The event identifier
    pub event_id: Option<i64>,
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_provenance_events: Option<LatestProvenanceEventsDto>,
}
/// The provenance events that matched the search criteria.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventDto {
    /// The alternate identifier uri for the fileflow for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_identifier_uri: Option<String>,
    /// The attributes of the flowfile for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeDto>>,
    /// The child uuids for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_uuids: Option<Vec<String>>,
    /// The label for the node where the event originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the event originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The id of the component that generated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component that generated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of the component that generated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// Whether the input and output content claim is the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_equal: Option<bool>,
    /// The event details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The event duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_duration: Option<i64>,
    /// The event id. This is a one up number thats unique per node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// The timestamp of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// Event Timestamp formatted using ISO8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,
    /// The type of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The size of the flowfile for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    /// The size of the flowfile in bytes for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<i64>,
    /// The uuid of the flowfile for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The id of the group that the component resides in. If the component is no longer in the flow, the group id will not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The event uuid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the input content is still available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_available: Option<bool>,
    /// The container in which the input content claim lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_container: Option<String>,
    /// The file size of the input content claim formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size: Option<String>,
    /// The file size of the intput content claim in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the input content claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_identifier: Option<String>,
    /// The offset into the input content claim where the flowfiles content begins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_offset: Option<i64>,
    /// The section in which the input content claim lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_section: Option<String>,
    /// The duration since the lineage began, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// Whether the output content is still available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_available: Option<bool>,
    /// The container in which the output content claim lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_container: Option<String>,
    /// The file size of the output content claim formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size: Option<String>,
    /// The file size of the output content claim in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the output content claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_identifier: Option<String>,
    /// The offset into the output content claim where the flowfiles content begins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_offset: Option<i64>,
    /// The section in which the output content claim lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_section: Option<String>,
    /// The parent uuids for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuids: Option<Vec<String>>,
    /// The relationship to which the flowfile was routed if the event is of type ROUTE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// Whether or not replay is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_available: Option<bool>,
    /// Explanation as to why replay is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_explanation: Option<String>,
    /// The identifier of the queue/connection from which the flowfile was pulled to genereate this event. May be null if the queue/connection is unknown or the flowfile was generated from this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection_identifier: Option<String>,
    /// The source system flowfile id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_system_flow_file_id: Option<String>,
    /// The source/destination system uri if the event was a RECEIVE/SEND.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_event: Option<ProvenanceEventDto>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// Which nodes are to replay their last provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ReplayLastEventSnapshotDto>,
    /// The UUID of the component whose last event should be replayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The node-wise results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeReplayLastEventSnapshotDto>>,
    /// Which nodes were requested to replay their last provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<ReplayLastEventResponseEntityNodes>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReplayRequestEntity {
    /// The identifier of the node where to submit the replay request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The event identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
}

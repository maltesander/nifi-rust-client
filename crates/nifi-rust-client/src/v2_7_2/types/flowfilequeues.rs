#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileDto {
    /// The FlowFile attributes.
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// The label for the node where this FlowFile resides.
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    pub cluster_node_id: Option<String>,
    /// The container in which the content claim lives.
    pub content_claim_container: Option<String>,
    /// The file size of the content claim formatted.
    pub content_claim_file_size: Option<String>,
    /// The file size of the content claim in bytes.
    pub content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the content claim.
    pub content_claim_identifier: Option<String>,
    /// The offset into the content claim where the flowfile's content begins.
    pub content_claim_offset: Option<i64>,
    /// The section in which the content claim lives.
    pub content_claim_section: Option<String>,
    /// The FlowFile filename.
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    pub queued_duration: Option<i64>,
    /// The FlowFile file size.
    pub size: Option<i64>,
    /// The URI that can be used to access this FlowFile.
    pub uri: Option<String>,
    /// The FlowFile UUID.
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileEntity {
    pub flow_file: FlowFileDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestDto {
    /// Whether the destination of the connection is running
    pub destination_running: Option<bool>,
    /// The reason, if any, that this listing request failed.
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    pub finished: Option<bool>,
    /// The FlowFile summaries. The summaries will be populated once the request has completed.
    pub flow_file_summaries: Option<Vec<FlowFileSummaryDto>>,
    /// The id for this listing request.
    pub id: Option<String>,
    /// The last time this listing request was updated.
    pub last_updated: Option<String>,
    /// The maximum number of FlowFileSummary objects to return
    pub max_results: Option<i32>,
    /// The current percent complete.
    pub percent_completed: Option<i32>,
    pub queue_size: Option<QueueSizeDto>,
    /// Whether the source of the connection is running
    pub source_running: Option<bool>,
    /// The current state of the listing request.
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    pub submission_time: Option<String>,
    /// The URI for future requests to this listing request.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestEntity {
    pub listing_request: ListingRequestDto,
}

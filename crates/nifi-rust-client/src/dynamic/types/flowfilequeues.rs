// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileDto {
    /// The FlowFile attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// The label for the node where this FlowFile resides.
    #[serde(rename = "clusterNodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The container in which the content claim lives.
    #[serde(rename = "contentClaimContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_container: Option<String>,
    /// The file size of the content claim formatted.
    #[serde(rename = "contentClaimFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size: Option<String>,
    /// The file size of the content claim in bytes.
    #[serde(rename = "contentClaimFileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the content claim.
    #[serde(rename = "contentClaimIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_identifier: Option<String>,
    /// The offset into the content claim where the flowfile's content begins.
    #[serde(rename = "contentClaimOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_offset: Option<i64>,
    /// The section in which the content claim lives.
    #[serde(rename = "contentClaimSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_section: Option<String>,
    /// The FlowFile filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    #[serde(rename = "lineageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    #[serde(rename = "penaltyExpiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    #[serde(rename = "queuedDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_duration: Option<i64>,
    /// The FlowFile file size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The URI that can be used to access this FlowFile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The FlowFile UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileEntity {
    pub flow_file: Option<FlowFileDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestDto {
    /// Whether the destination of the connection is running
    #[serde(rename = "destinationRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_running: Option<bool>,
    /// The reason, if any, that this listing request failed.
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The FlowFile summaries. The summaries will be populated once the request has completed.
    #[serde(rename = "flowFileSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_summaries: Option<Vec<FlowFileSummaryDto>>,
    /// The id for this listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The last time this listing request was updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The maximum number of FlowFileSummary objects to return
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The current percent complete.
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "queueSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<QueueSizeDto>,
    /// Whether the source of the connection is running
    #[serde(rename = "sourceRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_running: Option<bool>,
    /// The current state of the listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for future requests to this listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestEntity {
    pub listing_request: Option<ListingRequestDto>,
}

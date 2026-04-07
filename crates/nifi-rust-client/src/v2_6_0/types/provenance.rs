// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageDto {
    /// When the lineage query will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Whether the lineage query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id of this lineage query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The percent complete for the lineage query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<LineageRequestDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<LineageResultsDto>,
    /// When the lineage query was submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for this lineage query for later retrieval and deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage: Option<LineageDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceDto {
    /// The timestamp when the query will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id of the provenance query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The current percent complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ProvenanceRequestDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<ProvenanceResultsDto>,
    /// The timestamp when the query was submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for this query. Used for obtaining/deleting the request at a later time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsDto {
    /// The available searchable field for the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<ProvenanceSearchableFieldDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_options: Option<ProvenanceOptionsDto>,
}

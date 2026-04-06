#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageDto {
    /// When the lineage query will expire.
    pub expiration: Option<String>,
    /// Whether the lineage query has finished.
    pub finished: Option<bool>,
    /// The id of this lineage query.
    pub id: Option<String>,
    /// The percent complete for the lineage query.
    pub percent_completed: Option<i32>,
    pub request: Option<LineageRequestDto>,
    pub results: Option<LineageResultsDto>,
    /// When the lineage query was submitted.
    pub submission_time: Option<String>,
    /// The URI for this lineage query for later retrieval and deletion.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEntity {
    pub lineage: LineageDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceDto {
    /// The timestamp when the query will expire.
    pub expiration: Option<String>,
    /// Whether the query has finished.
    pub finished: Option<bool>,
    /// The id of the provenance query.
    pub id: Option<String>,
    /// The current percent complete.
    pub percent_completed: Option<i32>,
    pub request: Option<ProvenanceRequestDto>,
    pub results: Option<ProvenanceResultsDto>,
    /// The timestamp when the query was submitted.
    pub submission_time: Option<String>,
    /// The URI for this query. Used for obtaining/deleting the request at a later time
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEntity {
    pub provenance: ProvenanceDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsDto {
    /// The available searchable field for the NiFi.
    pub searchable_fields: Option<Vec<ProvenanceSearchableFieldDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsEntity {
    pub provenance_options: ProvenanceOptionsDto,
}

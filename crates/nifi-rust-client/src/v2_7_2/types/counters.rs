// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterDto {
    /// The context of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// The id of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The value count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_count: Option<i64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<CounterDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<CountersSnapshotDto>,
    /// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeCountersSnapshotDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<CountersDto>,
}

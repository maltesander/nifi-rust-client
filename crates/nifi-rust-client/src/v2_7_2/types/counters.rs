#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterDto {
    /// The context of the counter.
    pub context: Option<String>,
    /// The id of the counter.
    pub id: Option<String>,
    /// The name of the counter.
    pub name: Option<String>,
    /// The value of the counter.
    pub value: Option<String>,
    /// The value count.
    pub value_count: Option<i64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterEntity {
    pub counter: CounterDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersDto {
    pub aggregate_snapshot: Option<CountersSnapshotDto>,
    /// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    pub node_snapshots: Option<Vec<NodeCountersSnapshotDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersEntity {
    pub counters: CountersDto,
}

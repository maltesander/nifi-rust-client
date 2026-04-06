// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerDto {
    /// The number of active remote ports contained in the NiFi.
    pub active_remote_port_count: Option<i32>,
    /// The comments for the NiFi.
    pub comments: Option<String>,
    /// The number of disabled components in the NiFi.
    pub disabled_count: Option<i32>,
    /// The id of the NiFi.
    pub id: Option<String>,
    /// The number of inactive remote ports contained in the NiFi.
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports contained in the NiFi.
    pub input_port_count: Option<i32>,
    /// The input ports available to send data to for the NiFi.
    pub input_ports: Option<Vec<PortDto>>,
    /// If clustered, the id of the Cluster Manager, otherwise the id of the NiFi.
    pub instance_id: Option<String>,
    /// The number of invalid components in the NiFi.
    pub invalid_count: Option<i32>,
    /// The name of the NiFi.
    pub name: Option<String>,
    /// The number of output ports in the NiFi.
    pub output_port_count: Option<i32>,
    /// The output ports available to received data from the NiFi.
    pub output_ports: Option<Vec<PortDto>>,
    /// The HTTP(S) Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    pub remote_site_http_listening_port: Option<i32>,
    /// The Socket Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    pub remote_site_listening_port: Option<i32>,
    /// The number of running components in the NiFi.
    pub running_count: Option<i32>,
    /// Indicates whether or not Site-to-Site communications with this instance is secure (2-way authentication).
    pub site_to_site_secure: Option<bool>,
    /// The number of stopped components in the NiFi.
    pub stopped_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerEntity {
    pub controller: ControllerDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeersEntity {
    pub peers: Option<Vec<PeerDto>>,
}

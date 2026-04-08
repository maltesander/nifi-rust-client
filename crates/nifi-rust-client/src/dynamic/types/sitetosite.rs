// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerDto {
    /// The number of active remote ports contained in the NiFi.
    #[serde(rename = "activeRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The comments for the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of disabled components in the NiFi.
    #[serde(rename = "disabledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The id of the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote ports contained in the NiFi.
    #[serde(rename = "inactiveRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports contained in the NiFi.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The input ports available to send data to for the NiFi.
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortDto>>,
    /// If clustered, the id of the Cluster Manager, otherwise the id of the NiFi.
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// The number of invalid components in the NiFi.
    #[serde(rename = "invalidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The name of the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of output ports in the NiFi.
    #[serde(rename = "outputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    /// The output ports available to received data from the NiFi.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortDto>>,
    /// The HTTP(S) Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteHttpListeningPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_site_http_listening_port: Option<i32>,
    /// The Socket Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteListeningPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_site_listening_port: Option<i32>,
    /// The number of running components in the NiFi.
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// Indicates whether or not Site-to-Site communications with this instance is secure (2-way authentication).
    #[serde(rename = "siteToSiteSecure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_to_site_secure: Option<bool>,
    /// The number of stopped components in the NiFi.
    #[serde(rename = "stoppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerEntity {
    pub controller: Option<ControllerDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeersEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<PeerDto>>,
}

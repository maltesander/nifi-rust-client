#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum RemotePortRunStatusEntityState {
    #[default]
    #[serde(rename = "TRANSMITTING")]
    Transmitting,
    #[serde(rename = "STOPPED")]
    Stopped,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemotePortRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    /// The run status of the RemotePort.
    pub state: Option<RemotePortRunStatusEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub remote_process_group_port: Option<RemoteProcessGroupPortDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}

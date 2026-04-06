// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentsEntity {
    pub controller_service_referencing_components:
        Option<Vec<ControllerServiceReferencingComponentEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ControllerServiceRunStatusEntityState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    /// The run status of the ControllerService.
    pub state: Option<ControllerServiceRunStatusEntityState>,
    /// Indicates whether or not responses should only include fields necessary for rendering the NiFi User Interface.
    pub ui_only: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum UpdateControllerServiceReferenceRequestEntityState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateControllerServiceReferenceRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The identifier of the Controller Service.
    pub id: Option<String>,
    /// The revisions for all referencing components.
    pub referencing_component_revisions:
        Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The new state of the references for the controller service.
    pub state: Option<UpdateControllerServiceReferenceRequestEntityState>,
    /// Indicates whether or not the response should only include fields necessary for rendering the NiFi User Interface.
    pub ui_only: Option<bool>,
}

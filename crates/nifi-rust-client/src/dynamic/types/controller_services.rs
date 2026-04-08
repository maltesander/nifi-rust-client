// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentsEntity {
    #[serde(rename = "controllerServiceReferencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_referencing_components:
        Option<Vec<ControllerServiceReferencingComponentEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the ControllerService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Indicates whether or not responses should only include fields necessary for rendering the NiFi User Interface.
    /// As such, when this value is set to true, some fields may be returned as null values, and the selected fields may change at any time without notice.
    /// As a result, this value should not be set to true by any client other than the UI.
    #[serde(rename = "uiOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_only: Option<bool>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateControllerServiceReferenceRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The identifier of the Controller Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The revisions for all referencing components.
    #[serde(rename = "referencingComponentRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_component_revisions:
        Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The new state of the references for the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Indicates whether or not the response should only include fields necessary for rendering the NiFi User Interface.
    /// As such, when this value is set to true, some fields may be returned as null values, and the selected fields may change at any time without notice.
    /// As a result, this value should not be set to true by any client other than the UI.
    #[serde(rename = "uiOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_only: Option<bool>,
}

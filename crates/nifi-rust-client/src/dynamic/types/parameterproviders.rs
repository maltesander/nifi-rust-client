// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
/// The Apply Parameters Request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestDto {
    /// Whether or not the request is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The Parameter Contexts updated by this Parameter Provider. This may not be populated until the request has successfully completed.
    #[serde(rename = "parameterContextUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_updates: Option<Vec<ParameterContextUpdateEntity>>,
    #[serde(rename = "parameterProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider: Option<ParameterProviderDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterProviderApplyParametersUpdateStepDto>>,
    /// The URI for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestEntity {
    pub request: Option<ParameterProviderApplyParametersRequestDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterApplicationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Configuration for the fetched Parameter Groups
    #[serde(rename = "parameterGroupConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterFetchEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentsEntity {
    #[serde(rename = "parameterProviderReferencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_referencing_components:
        Option<Vec<ParameterProviderReferencingComponentEntity>>,
}

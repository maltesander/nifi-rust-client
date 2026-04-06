// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// The Apply Parameters Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestDto {
    /// Whether or not the request is completed
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    pub last_updated: Option<String>,
    /// The Parameter Contexts updated by this Parameter Provider. This may not be populated until the request has successfully completed.
    pub parameter_context_updates: Option<Vec<ParameterContextUpdateEntity>>,
    pub parameter_provider: Option<ParameterProviderDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update.
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request
    pub request_id: Option<String>,
    /// A description of the current state of the request
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    pub update_steps: Option<Vec<ParameterProviderApplyParametersUpdateStepDto>>,
    /// The URI for the request
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestEntity {
    pub request: ParameterProviderApplyParametersRequestDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterApplicationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    pub id: Option<String>,
    /// Configuration for the fetched Parameter Groups
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterFetchEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    pub id: Option<String>,
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentsEntity {
    pub parameter_provider_referencing_components:
        Option<Vec<ParameterProviderReferencingComponentEntity>>,
}

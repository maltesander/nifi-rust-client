// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    /// Whether the system requires login through an external Identity Provider
    pub external_login_required: Option<bool>,
    /// Whether the system is configured to support login operations
    pub login_supported: Option<bool>,
    /// Location for initiating login processing
    pub login_uri: Option<String>,
    /// Location for initiating logout processing
    pub logout_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    pub authentication_configuration: AuthenticationConfigurationDto,
}

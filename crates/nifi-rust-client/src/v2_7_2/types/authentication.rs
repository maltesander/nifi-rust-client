// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    /// Whether the system requires login through an external Identity Provider Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_required: Option<bool>,
    /// Whether the system is configured to support login operations Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_supported: Option<bool>,
    /// Location for initiating login processing Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_uri: Option<String>,
    /// Location for initiating logout processing Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfigurationDto>,
}

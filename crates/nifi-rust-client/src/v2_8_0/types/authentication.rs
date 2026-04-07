// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    /// Whether the system requires login through an external Identity Provider Read-only — this field is ignored when serializing requests to NiFi.
    #[serde(skip_serializing)]
    pub external_login_required: Option<bool>,
    /// Whether the system is configured to support login operations Read-only — this field is ignored when serializing requests to NiFi.
    #[serde(skip_serializing)]
    pub login_supported: Option<bool>,
    /// Location for initiating login processing Read-only — this field is ignored when serializing requests to NiFi.
    #[serde(skip_serializing)]
    pub login_uri: Option<String>,
    /// Location for initiating logout processing Read-only — this field is ignored when serializing requests to NiFi.
    #[serde(skip_serializing)]
    pub logout_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    pub authentication_configuration: AuthenticationConfigurationDto,
}

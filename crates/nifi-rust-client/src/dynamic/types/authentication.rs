// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    /// Whether the system requires login through an external Identity Provider
    #[serde(rename = "externalLoginRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_required: Option<bool>,
    /// Whether the system is configured to support login operations
    #[serde(rename = "loginSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_supported: Option<bool>,
    /// Location for initiating login processing
    #[serde(rename = "loginUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_uri: Option<String>,
    /// Location for initiating logout processing
    #[serde(rename = "logoutUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    pub authentication_configuration: Option<AuthenticationConfigurationDto>,
}

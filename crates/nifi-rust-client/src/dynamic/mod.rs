// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

mod conversions;
pub mod types;
use crate::{NifiClient, NifiError};
/// Represents a detected NiFi server version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectedVersion {
    V2_6_0,
    V2_7_2,
    V2_8_0,
}
impl std::fmt::Display for DetectedVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectedVersion::V2_6_0 => write!(f, "2.6.0"),
            DetectedVersion::V2_7_2 => write!(f, "2.7.2"),
            DetectedVersion::V2_8_0 => write!(f, "2.8.0"),
        }
    }
}
/// Match a version string by major.minor (ignoring patch).
fn version_from_str(version: &str) -> Result<DetectedVersion, NifiError> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() < 2 {
        return Err(NifiError::UnsupportedVersion {
            detected: version.to_string(),
        });
    }
    let major_minor = format!("{}.{}", parts[0], parts[1]);
    match major_minor.as_str() {
        "2.6" => Ok(DetectedVersion::V2_6_0),
        "2.7" => Ok(DetectedVersion::V2_7_2),
        "2.8" => Ok(DetectedVersion::V2_8_0),
        _ => Err(NifiError::UnsupportedVersion {
            detected: version.to_string(),
        }),
    }
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AboutResponse {
    about: AboutInner,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AboutInner {
    version: String,
}
/// A dynamic NiFi client that detects the server version at connect time
/// and dispatches API calls to the correct version's generated code.
#[derive(Debug)]
pub struct DynamicClient {
    client: NifiClient,
    version: DetectedVersion,
}
impl DynamicClient {
    /// Wrap an existing `NifiClient` and detect the NiFi server version via GET /flow/about.
    pub async fn from_client(client: NifiClient) -> Result<Self, NifiError> {
        let resp: AboutResponse = client.get("/flow/about").await?;
        let version = version_from_str(&resp.about.version)?;
        Ok(Self { client, version })
    }
    /// Returns the detected NiFi server version.
    pub fn detected_version(&self) -> DetectedVersion {
        self.version
    }
    /// Returns a reference to the underlying `NifiClient`.
    pub fn inner(&self) -> &NifiClient {
        &self.client
    }
    /// Authenticate with the NiFi instance.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {
        self.client.login(username, password).await
    }
    /// Log out from the NiFi instance.
    pub async fn logout(&self) -> Result<(), NifiError> {
        self.client.logout().await
    }
    /// Access the [Access API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn access_api(&self) -> DynamicAccessApi<'_> {
        DynamicAccessApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Authentication API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn authentication_api(&self) -> DynamicAuthenticationApi<'_> {
        DynamicAuthenticationApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Connections API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn connections_api(&self) -> DynamicConnectionsApi<'_> {
        DynamicConnectionsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Controller API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn controller_api(&self) -> DynamicControllerApi<'_> {
        DynamicControllerApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Controller Services API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn controller_services_api(&self) -> DynamicControllerServicesApi<'_> {
        DynamicControllerServicesApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Counters API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn counters_api(&self) -> DynamicCountersApi<'_> {
        DynamicCountersApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [DataTransfer API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn datatransfer_api(&self) -> DynamicDataTransferApi<'_> {
        DynamicDataTransferApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Flow API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn flow_api(&self) -> DynamicFlowApi<'_> {
        DynamicFlowApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [FlowFileQueues API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn flowfilequeues_api(&self) -> DynamicFlowFileQueuesApi<'_> {
        DynamicFlowFileQueuesApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Funnels API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn funnels_api(&self) -> DynamicFunnelsApi<'_> {
        DynamicFunnelsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [InputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn inputports_api(&self) -> DynamicInputPortsApi<'_> {
        DynamicInputPortsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Labels API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn labels_api(&self) -> DynamicLabelsApi<'_> {
        DynamicLabelsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [OutputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn outputports_api(&self) -> DynamicOutputPortsApi<'_> {
        DynamicOutputPortsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [ParameterContexts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn parametercontexts_api(&self) -> DynamicParameterContextsApi<'_> {
        DynamicParameterContextsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [ParameterProviders API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn parameterproviders_api(&self) -> DynamicParameterProvidersApi<'_> {
        DynamicParameterProvidersApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Policies API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn policies_api(&self) -> DynamicPoliciesApi<'_> {
        DynamicPoliciesApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [ProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn processgroups_api(&self) -> DynamicProcessGroupsApi<'_> {
        DynamicProcessGroupsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Processors API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn processors_api(&self) -> DynamicProcessorsApi<'_> {
        DynamicProcessorsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Provenance API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn provenance_api(&self) -> DynamicProvenanceApi<'_> {
        DynamicProvenanceApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [ProvenanceEvents API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn provenanceevents_api(&self) -> DynamicProvenanceEventsApi<'_> {
        DynamicProvenanceEventsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [RemoteProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn remoteprocessgroups_api(&self) -> DynamicRemoteProcessGroupsApi<'_> {
        DynamicRemoteProcessGroupsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [ReportingTasks API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn reportingtasks_api(&self) -> DynamicReportingTasksApi<'_> {
        DynamicReportingTasksApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Resources API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn resources_api(&self) -> DynamicResourcesApi<'_> {
        DynamicResourcesApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [SiteToSite API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn sitetosite_api(&self) -> DynamicSiteToSiteApi<'_> {
        DynamicSiteToSiteApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Snippets API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn snippets_api(&self) -> DynamicSnippetsApi<'_> {
        DynamicSnippetsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [SystemDiagnostics API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn systemdiagnostics_api(&self) -> DynamicSystemDiagnosticsApi<'_> {
        DynamicSystemDiagnosticsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Tenants API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn tenants_api(&self) -> DynamicTenantsApi<'_> {
        DynamicTenantsApi {
            client: &self.client,
            version: self.version,
        }
    }
    /// Access the [Versions API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn versions_api(&self) -> DynamicVersionsApi<'_> {
        DynamicVersionsApi {
            client: &self.client,
            version: self.version,
        }
    }
}
/// Dynamic dispatch wrapper for the Access API.
pub struct DynamicAccessApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicAccessApi<'a> {
    /// Performs a logout for other providers that have been issued a JWT.
    pub async fn log_out(&self) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out().await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out().await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out().await
            }
        }
    }
    /// Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.
    pub async fn log_out_complete(&self) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out_complete().await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out_complete().await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::access::AccessApi {
                    client: self.client,
                };
                api.log_out_complete().await
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Authentication API.
pub struct DynamicAuthenticationApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicAuthenticationApi<'a> {
    /// Retrieves the authentication configuration endpoint and status information
    pub async fn get_authentication_configuration(
        &self,
    ) -> Result<types::AuthenticationConfigurationDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::authentication::AuthenticationApi {
                    client: self.client,
                };
                Ok(api.get_authentication_configuration().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::authentication::AuthenticationApi {
                    client: self.client,
                };
                Ok(api.get_authentication_configuration().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::authentication::AuthenticationApi {
                    client: self.client,
                };
                Ok(api.get_authentication_configuration().await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Connections API.
pub struct DynamicConnectionsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicConnectionsApi<'a> {
    /// Deletes a connection
    pub async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_connection(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Gets a connection
    pub async fn get_connection(&self, id: &str) -> Result<types::ConnectionEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api.get_connection(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api.get_connection(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api.get_connection(id).await?.into())
            }
        }
    }
    /// Updates a connection
    pub async fn update_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_connection(id, &crate::v2_6_0::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_connection(id, &crate::v2_7_2::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::connections::ConnectionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_connection(id, &crate::v2_8_0::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Controller API.
pub struct DynamicControllerApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicControllerApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    pub async fn analyze_flow_analysis_rule_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_flow_analysis_rule_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn analyze_flow_registry_client_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "analyze_flow_registry_client_configuration".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_flow_registry_client_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_flow_registry_client_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a flow analysis rule
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_flow_analysis_rule_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_flow_analysis_rule_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_flow_analysis_rule_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_flow_analysis_rule_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a parameter provider
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_parameter_provider_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_parameter_provider_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_parameter_provider_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_parameter_provider_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a registry client
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_registry_client_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_registry_client_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_registry_client_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_registry_client_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears the state for a flow analysis rule
    pub async fn clear_state(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state(&crate::v2_8_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new bulletin
    pub async fn create_bulletin(
        &self,
        body: types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_bulletin(&crate::v2_6_0::types::BulletinEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_bulletin(&crate::v2_7_2::types::BulletinEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_bulletin(&crate::v2_8_0::types::BulletinEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new controller service
    pub async fn create_controller_service(
        &self,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_controller_service(
                        &crate::v2_6_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_controller_service(
                        &crate::v2_7_2::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_controller_service(
                        &crate::v2_8_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new flow analysis rule
    pub async fn create_flow_analysis_rule(
        &self,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_analysis_rule(
                        &crate::v2_6_0::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_analysis_rule(
                        &crate::v2_7_2::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_analysis_rule(
                        &crate::v2_8_0::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new flow registry client
    pub async fn create_flow_registry_client(
        &self,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_registry_client(
                        &crate::v2_6_0::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_registry_client(
                        &crate::v2_7_2::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_flow_registry_client(
                        &crate::v2_8_0::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new parameter provider
    pub async fn create_parameter_provider(
        &self,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_provider(
                        &crate::v2_6_0::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_provider(
                        &crate::v2_7_2::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_provider(
                        &crate::v2_8_0::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new reporting task
    pub async fn create_reporting_task(
        &self,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_reporting_task(&crate::v2_6_0::types::ReportingTaskEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_reporting_task(&crate::v2_7_2::types::ReportingTaskEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .create_reporting_task(&crate::v2_8_0::types::ReportingTaskEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    pub async fn delete_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .delete_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes a flow registry client
    pub async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_flow_registry_client(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_flow_registry_client(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_flow_registry_client(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Purges history
    pub async fn delete_history(&self, end_date: &str) -> Result<types::HistoryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_history(end_date).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_history(end_date).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_history(end_date).await?.into())
            }
        }
    }
    /// Deletes an installed NAR
    pub async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_nar(id, disconnected_node_acknowledged, force)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_nar(id, disconnected_node_acknowledged, force)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .delete_nar(id, disconnected_node_acknowledged, force)
                    .await?
                    .into())
            }
        }
    }
    /// Removes a node from the cluster
    pub async fn delete_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_node(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_node(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.delete_node(id).await?.into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn delete_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_registry_client_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .delete_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .delete_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the content of the NAR with the given id
    pub async fn download_nar(&self, id: &str) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerContentApi {
                    client: self.client,
                    id,
                };
                api.download_nar().await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerContentApi {
                    client: self.client,
                    id,
                };
                api.download_nar().await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerContentApi {
                    client: self.client,
                    id,
                };
                api.download_nar().await
            }
        }
    }
    /// Gets the contents of the cluster
    pub async fn get_cluster(&self) -> Result<types::ClusterDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_cluster().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_cluster().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_cluster().await?.into())
            }
        }
    }
    /// Retrieves the configuration for this NiFi Controller
    pub async fn get_controller_config(
        &self,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_controller_config().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_controller_config().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_controller_config().await?.into())
            }
        }
    }
    /// Gets a flow analysis rule
    pub async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rule(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rule(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rule(id).await?.into())
            }
        }
    }
    /// Gets a flow analysis rule property descriptor
    pub async fn get_flow_analysis_rule_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the state for a flow analysis rule
    pub async fn get_flow_analysis_rule_state(
        &self,
        id: &str,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flow_analysis_rule_state().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    pub async fn get_flow_analysis_rule_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_analysis_rule_verification_request(request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all flow analysis rules
    pub async fn get_flow_analysis_rules(
        &self,
    ) -> Result<types::FlowAnalysisRulesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rules().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rules().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_analysis_rules().await?.into())
            }
        }
    }
    /// Gets a flow registry client
    pub async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_client(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_client(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_client(id).await?.into())
            }
        }
    }
    /// Gets the listing of available flow registry clients
    pub async fn get_flow_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_clients().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_clients().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_flow_registry_clients().await?.into())
            }
        }
    }
    /// Retrieves the component types available from the installed NARs
    pub async fn get_nar_details(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_nar_details().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_nar_details().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDetailsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_nar_details().await?.into())
            }
        }
    }
    /// Retrieves summary information for installed NARs
    pub async fn get_nar_summaries(&self) -> Result<types::NarSummariesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summaries().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summaries().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summaries().await?.into())
            }
        }
    }
    /// Retrieves the summary information for the NAR with the given identifier
    pub async fn get_nar_summary(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summary(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summary(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_nar_summary(id).await?.into())
            }
        }
    }
    /// Gets a node in the cluster
    pub async fn get_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node(id).await?.into())
            }
        }
    }
    /// Gets status history for the node
    pub async fn get_node_status_history(&self) -> Result<types::ComponentHistoryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node_status_history().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node_status_history().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_node_status_history().await?.into())
            }
        }
    }
    /// Gets a flow registry client property descriptor
    pub async fn get_property_descriptor(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor(property_name, sensitive)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the types of flow  that this NiFi supports
    pub async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_registry_client_types().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_registry_client_types().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.get_registry_client_types().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn get_registry_client_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_registry_client_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_registry_client_verification_request(request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Imports a reporting task snapshot
    pub async fn import_reporting_task_snapshot(
        &self,
        body: types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<types::VersionedReportingTaskImportResponseEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .import_reporting_task_snapshot(
                        &crate::v2_6_0::types::VersionedReportingTaskImportRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .import_reporting_task_snapshot(
                        &crate::v2_7_2::types::VersionedReportingTaskImportRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .import_reporting_task_snapshot(
                        &crate::v2_8_0::types::VersionedReportingTaskImportRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Deletes a flow analysis rule
    pub async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .remove_flow_analysis_rule(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .remove_flow_analysis_rule(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .remove_flow_analysis_rule(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Performs verification of the Flow Analysis Rule's configuration
    pub async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_flow_analysis_rule_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Performs verification of the Registry Client's configuration
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn submit_registry_client_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "submit_registry_client_config_verification_request".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_registry_client_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_registry_client_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the configuration for this NiFi
    pub async fn update_controller_config(
        &self,
        body: types::ControllerConfigurationEntity,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_config(
                        &crate::v2_6_0::types::ControllerConfigurationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_config(
                        &crate::v2_7_2::types::ControllerConfigurationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_config(
                        &crate::v2_8_0::types::ControllerConfigurationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a flow analysis rule
    pub async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_analysis_rule(
                        id,
                        &crate::v2_6_0::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_analysis_rule(
                        id,
                        &crate::v2_7_2::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_analysis_rule(
                        id,
                        &crate::v2_8_0::types::FlowAnalysisRuleEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a flow registry client
    pub async fn update_flow_registry_client(
        &self,
        id: &str,
        body: types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_registry_client(
                        id,
                        &crate::v2_6_0::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_registry_client(
                        id,
                        &crate::v2_7_2::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_registry_client(
                        id,
                        &crate::v2_8_0::types::FlowRegistryClientEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a node in the cluster
    pub async fn update_node(
        &self,
        id: &str,
        body: types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_node(id, &crate::v2_6_0::types::NodeEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_node(id, &crate::v2_7_2::types::NodeEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api
                    .update_node(id, &crate::v2_8_0::types::NodeEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a flow analysis rule
    pub async fn update_run_status(
        &self,
        id: &str,
        body: types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_6_0::types::FlowAnalysisRuleRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_7_2::types::FlowAnalysisRuleRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status(
                        &crate::v2_8_0::types::FlowAnalysisRuleRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Uploads a NAR and requests for it to be installed
    pub async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.upload_nar(filename, data).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.upload_nar(filename, data).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller::ControllerApi {
                    client: self.client,
                };
                Ok(api.upload_nar(filename, data).await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Controller Services API.
pub struct DynamicControllerServicesApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicControllerServicesApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    pub async fn analyze_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a controller service
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears the state for a controller service
    pub async fn clear_state_1(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_1(&crate::v2_8_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    pub async fn delete_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request(request_id).await?.into())
            }
        }
    }
    /// Gets a controller service
    pub async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api.get_controller_service(id, ui_only).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api.get_controller_service(id, ui_only).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api.get_controller_service(id, ui_only).await?.into())
            }
        }
    }
    /// Gets a controller service
    pub async fn get_controller_service_references(
        &self,
        id: &str,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_controller_service_references().await?.into())
            }
        }
    }
    /// Gets a controller service property descriptor
    pub async fn get_property_descriptor_1(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .get_property_descriptor_1(property_name, sensitive)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the state for a controller service
    pub async fn get_state(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    pub async fn get_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request(request_id).await?.into())
            }
        }
    }
    /// Deletes a controller service
    pub async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_controller_service(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_controller_service(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_controller_service(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Performs verification of the Controller Service's configuration
    pub async fn submit_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a controller service
    pub async fn update_controller_service(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_service(
                        id,
                        &crate::v2_6_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_service(
                        id,
                        &crate::v2_7_2::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesApi {
                    client: self.client,
                };
                Ok(api
                    .update_controller_service(
                        id,
                        &crate::v2_8_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a controller services references
    pub async fn update_controller_service_references(
        &self,
        id: &str,
        body: types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_6_0::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_7_2::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::controller_services::ControllerServicesReferencesApi {
                        client: self.client,
                        id,
                    };
                Ok(
                    api
                        .update_controller_service_references(
                            &crate::v2_8_0::types::UpdateControllerServiceReferenceRequestEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
        }
    }
    /// Updates run status of a controller service
    pub async fn update_run_status_1(
        &self,
        id: &str,
        body: types::ControllerServiceRunStatusEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_6_0::types::ControllerServiceRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_7_2::types::ControllerServiceRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::controller_services::ControllerServicesRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_1(
                        &crate::v2_8_0::types::ControllerServiceRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Counters API.
pub struct DynamicCountersApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicCountersApi<'a> {
    /// Gets the current counters for this NiFi
    pub async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::CountersDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.get_counters(nodewise, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.get_counters(nodewise, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.get_counters(nodewise, cluster_node_id).await?.into())
            }
        }
    }
    /// Updates all counters. This will reset all counter values to 0
    pub async fn update_all_counters(&self) -> Result<types::CountersDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_all_counters().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_all_counters().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_all_counters().await?.into())
            }
        }
    }
    /// Updates the specified counter. This will reset the counter value to 0
    pub async fn update_counter(&self, id: &str) -> Result<types::CounterDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_counter(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_counter(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::counters::CountersApi {
                    client: self.client,
                };
                Ok(api.update_counter(id).await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the DataTransfer API.
pub struct DynamicDataTransferApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicDataTransferApi<'a> {
    /// Commit or cancel the specified transaction
    pub async fn commit_input_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
        }
    }
    /// Commit or cancel the specified transaction
    pub async fn commit_output_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
        }
    }
    /// Create a transaction to the specified output port or input port
    pub async fn create_port_transaction(
        &self,
        port_id: &str,
        port_type: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
        }
    }
    /// Extend transaction TTL
    pub async fn extend_input_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
        }
    }
    /// Extend transaction TTL
    pub async fn extend_output_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
        }
    }
    /// Transfer flow files to the input port
    pub async fn receive_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
        }
    }
    /// Transfer flow files from the output port
    pub async fn transfer_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Flow API.
pub struct DynamicFlowApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicFlowApi<'a> {
    /// Enable or disable Controller Services in the specified Process Group.
    pub async fn activate_controller_services(
        &self,
        id: &str,
        body: types::ActivateControllerServicesEntity,
    ) -> Result<types::ActivateControllerServicesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_6_0::types::ActivateControllerServicesEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_7_2::types::ActivateControllerServicesEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .activate_controller_services(
                        &crate::v2_8_0::types::ActivateControllerServicesEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for components in the specified Process Group.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_1(
        &self,
        id: &str,
        body: types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<types::ClearBulletinsForGroupResultsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_1".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_1(
                        &crate::v2_7_2::types::ClearBulletinsForGroupRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_1(
                        &crate::v2_8_0::types::ClearBulletinsForGroupRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Download a snapshot of the given reporting tasks and any controller services they use
    pub async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.download_reporting_task_snapshot(reporting_task_id)
                    .await
            }
        }
    }
    /// Generates a client id.
    pub async fn generate_client_id(&self) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.generate_client_id().await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                api.generate_client_id().await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.generate_client_id().await
            }
        }
    }
    /// Retrieves details about this NiFi to put in the About dialog
    pub async fn get_about_info(&self) -> Result<types::AboutDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_about_info().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_about_info().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_about_info().await?.into())
            }
        }
    }
    /// Gets an action
    pub async fn get_action(&self, id: &str) -> Result<types::ActionEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_action(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_action(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_action(id).await?.into())
            }
        }
    }
    /// Retrieves the additional details for the specified component type.
    pub async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::AdditionalDetailsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_additional_details(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_additional_details(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_additional_details(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Returns all flow analysis results currently in effect
    pub async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_all_flow_analysis_results().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_all_flow_analysis_results().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_all_flow_analysis_results().await?.into())
            }
        }
    }
    /// Retrieves the banners for this NiFi
    pub async fn get_banners(&self) -> Result<types::BannerDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_banners().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_banners().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_banners().await?.into())
            }
        }
    }
    /// Gets the branches from the specified registry for the current user
    pub async fn get_branches(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryBranchesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_branches().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_branches().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_branches().await?.into())
            }
        }
    }
    /// Gets the breadcrumbs for a process group
    pub async fn get_breadcrumbs(
        &self,
        id: &str,
    ) -> Result<types::FlowBreadcrumbEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBreadcrumbsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_breadcrumbs().await?.into())
            }
        }
    }
    /// Gets the buckets from the specified registry for the current user
    pub async fn get_buckets(
        &self,
        id: &str,
        branch: Option<&str>,
    ) -> Result<types::FlowRegistryBucketsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_buckets(branch).await?.into())
            }
        }
    }
    /// Gets current bulletins
    pub async fn get_bulletin_board(
        &self,
        after: Option<&str>,
        source_name: Option<&str>,
        message: Option<&str>,
        source_id: Option<&str>,
        group_id: Option<&str>,
        limit: Option<&str>,
    ) -> Result<types::BulletinBoardDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_bulletin_board(after, source_name, message, source_id, group_id, limit)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves Controller level bulletins
    pub async fn get_bulletins(&self) -> Result<types::ControllerBulletinsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_bulletins().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_bulletins().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_bulletins().await?.into())
            }
        }
    }
    /// The cluster summary for this NiFi
    pub async fn get_cluster_summary(&self) -> Result<types::ClusterSummaryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_cluster_summary().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_cluster_summary().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_cluster_summary().await?.into())
            }
        }
    }
    /// Gets configuration history for a component
    pub async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<types::ComponentHistoryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_component_history(component_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_component_history(component_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_component_history(component_id).await?.into())
            }
        }
    }
    /// Gets statistics for a connection
    pub async fn get_connection_statistics(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatisticsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatisticsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_statistics(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets status for a connection
    pub async fn get_connection_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_connection_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the status history for a connection
    pub async fn get_connection_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connection_status_history().await?.into())
            }
        }
    }
    /// Retrieves the registered content viewers
    pub async fn get_content_viewers(&self) -> Result<types::ContentViewerEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_content_viewers().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_content_viewers().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_content_viewers().await?.into())
            }
        }
    }
    /// Retrieves the Controller Service Definition for the specified component type.
    pub async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ControllerServiceDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the types of controller services that this NiFi supports
    pub async fn get_controller_service_types(
        &self,
        service_type: Option<&str>,
        service_bundle_group: Option<&str>,
        service_bundle_artifact: Option<&str>,
        service_bundle_version: Option<&str>,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        type_filter: Option<&str>,
    ) -> Result<types::ControllerServiceTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_types(
                        service_type,
                        service_bundle_group,
                        service_bundle_artifact,
                        service_bundle_version,
                        bundle_group_filter,
                        bundle_artifact_filter,
                        type_filter,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_types(
                        service_type,
                        service_bundle_group,
                        service_bundle_artifact,
                        service_bundle_version,
                        bundle_group_filter,
                        bundle_artifact_filter,
                        type_filter,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_service_types(
                        service_type,
                        service_bundle_group,
                        service_bundle_artifact,
                        service_bundle_version,
                        bundle_group_filter,
                        bundle_artifact_filter,
                        type_filter,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets controller services for reporting tasks
    pub async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_services_from_controller(
                        ui_only,
                        include_referencing_components,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_services_from_controller(
                        ui_only,
                        include_referencing_components,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_controller_services_from_controller(
                        ui_only,
                        include_referencing_components,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets all controller services
    pub async fn get_controller_services_from_group(
        &self,
        id: &str,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_controller_services_from_group(
                        include_ancestor_groups,
                        include_descendant_groups,
                        include_referencing_components,
                        ui_only,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets the current status of this NiFi
    pub async fn get_controller_status(&self) -> Result<types::ControllerStatusDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_controller_status().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_controller_status().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_controller_status().await?.into())
            }
        }
    }
    /// Retrieves the user identity of the user making the request
    pub async fn get_current_user(&self) -> Result<types::CurrentUserEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_current_user().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_current_user().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_current_user().await?.into())
            }
        }
    }
    /// Gets the details of a flow from the specified registry and bucket for the specified flow for the current user
    pub async fn get_details(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_details(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
        }
    }
    /// Gets a process group
    pub async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ProcessGroupFlowEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow(id, ui_only).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow(id, ui_only).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow(id, ui_only).await?.into())
            }
        }
    }
    /// Returns flow analysis results produced by the analysis of a given process group
    pub async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_results(process_group_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_results(process_group_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_results(process_group_id)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the Flow Analysis Rule Definition for the specified component type.
    pub async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowAnalysisRuleDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the types of available Flow Analysis Rules
    pub async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::FlowAnalysisRuleTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_analysis_rule_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the configuration for this NiFi flow
    pub async fn get_flow_config(&self) -> Result<types::FlowConfigurationDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow_config().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow_config().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_flow_config().await?.into())
            }
        }
    }
    /// Gets all metrics for the flow from a particular node
    pub async fn get_flow_metrics(
        &self,
        producer: &str,
        included_registries: Option<types::IncludedRegistries>,
        sample_name: Option<&str>,
        sample_label_value: Option<&str>,
        root_field_name: Option<&str>,
        flow_metrics_reporting_strategy: Option<types::FlowMetricsReportingStrategy>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.get_flow_metrics(
                    producer,
                    included_registries
                        .map(crate::v2_6_0::types::IncludedRegistries::try_from)
                        .transpose()?,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                )
                .await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                api.get_flow_metrics(
                    producer,
                    included_registries
                        .map(crate::v2_7_2::types::IncludedRegistries::try_from)
                        .transpose()?,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                    flow_metrics_reporting_strategy
                        .map(crate::v2_7_2::types::FlowMetricsReportingStrategy::try_from)
                        .transpose()?,
                )
                .await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                api.get_flow_metrics(
                    producer,
                    included_registries
                        .map(crate::v2_8_0::types::IncludedRegistries::try_from)
                        .transpose()?,
                    sample_name,
                    sample_label_value,
                    root_field_name,
                    flow_metrics_reporting_strategy
                        .map(crate::v2_8_0::types::FlowMetricsReportingStrategy::try_from)
                        .transpose()?,
                )
                .await
            }
        }
    }
    /// Retrieves the Flow Registry Client Definition for the specified component type.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn get_flow_registry_client_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowRegistryClientDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flow_registry_client_definition".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_registry_client_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_flow_registry_client_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the flows from the specified registry and bucket for the current user
    pub async fn get_flows(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_flows(registry_id, bucket_id, branch).await?.into())
            }
        }
    }
    /// Gets status for an input port
    pub async fn get_input_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_input_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all listen ports configured on this NiFi that the current user has access to
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn get_listen_ports(&self) -> Result<types::ListenPortsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_listen_ports".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_listen_ports().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_listen_ports().await?.into())
            }
        }
    }
    /// Gets status for an output port
    pub async fn get_output_port_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_output_port_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all Parameter Contexts
    pub async fn get_parameter_contexts(
        &self,
    ) -> Result<types::ParameterContextsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_contexts().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_contexts().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_contexts().await?.into())
            }
        }
    }
    /// Retrieves the Parameter Provider Definition for the specified component type.
    pub async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ParameterProviderDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the types of parameter providers that this NiFi supports
    pub async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ParameterProviderTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_provider_types(
                        bundle_group_filter,
                        bundle_artifact_filter,
                        r#type,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets all parameter providers
    pub async fn get_parameter_providers(
        &self,
    ) -> Result<types::ParameterProvidersEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_providers().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_providers().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_parameter_providers().await?.into())
            }
        }
    }
    /// Retrieves the types of prioritizers that this NiFi supports
    pub async fn get_prioritizers(&self) -> Result<types::PrioritizerTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_prioritizers().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_prioritizers().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_prioritizers().await?.into())
            }
        }
    }
    /// Gets the status for a process group
    pub async fn get_process_group_status(
        &self,
        id: &str,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessGroupStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_process_group_status(recursive, nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets status history for a remote process group
    pub async fn get_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_group_status_history().await?.into())
            }
        }
    }
    /// Retrieves the Processor Definition for the specified component type.
    pub async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ProcessorDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Gets status for a processor
    pub async fn get_processor_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessorStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_processor_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets status history for a processor
    pub async fn get_processor_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_status_history().await?.into())
            }
        }
    }
    /// Retrieves the types of processors that this NiFi supports
    pub async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ProcessorTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the listing of available flow registry clients
    pub async fn get_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_registry_clients().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_registry_clients().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_registry_clients().await?.into())
            }
        }
    }
    /// Gets status for a remote process group
    pub async fn get_remote_process_group_status(
        &self,
        id: &str,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::RemoteProcessGroupStatusEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_remote_process_group_status(nodewise, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the status history
    pub async fn get_remote_process_group_status_history(
        &self,
        id: &str,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_group_status_history().await?.into())
            }
        }
    }
    /// Retrieves the Reporting Task Definition for the specified component type.
    pub async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ReportingTaskDefinition, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_definition(group, artifact, version, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Get a snapshot of the given reporting tasks and any controller services they use
    pub async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<types::VersionedReportingTaskSnapshot, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_snapshot(reporting_task_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_snapshot(reporting_task_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_snapshot(reporting_task_id)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the types of reporting tasks that this NiFi supports
    pub async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ReportingTaskTypesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, r#type)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all reporting tasks
    pub async fn get_reporting_tasks(&self) -> Result<types::ReportingTasksEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_reporting_tasks().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_reporting_tasks().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_reporting_tasks().await?.into())
            }
        }
    }
    /// Retrieves the runtime manifest for this NiFi instance.
    pub async fn get_runtime_manifest(&self) -> Result<types::RuntimeManifest, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_runtime_manifest().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_runtime_manifest().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.get_runtime_manifest().await?.into())
            }
        }
    }
    /// Gets the differences between two versions of the same versioned flow, the basis of the comparison will be the first version
    pub async fn get_version_differences(
        &self,
        id: &str,
        registry_id: &str,
        branch_id_a: &str,
        bucket_id_a: &str,
        flow_id_a: &str,
        version_a: &str,
        branch_id_b: &str,
        bucket_id_b: &str,
        flow_id_b: &str,
        version_b: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBranchesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_version_differences(
                        registry_id,
                        branch_id_a,
                        bucket_id_a,
                        flow_id_a,
                        version_a,
                        branch_id_b,
                        bucket_id_b,
                        flow_id_b,
                        version_b,
                        offset,
                        limit,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets the flow versions from the specified registry and bucket for the specified flow for the current user
    pub async fn get_versions(
        &self,
        id: &str,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowBucketsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_versions(registry_id, bucket_id, flow_id, branch)
                    .await?
                    .into())
            }
        }
    }
    /// Gets configuration history
    pub async fn query_history(
        &self,
        offset: &str,
        count: &str,
        sort_column: Option<&str>,
        sort_order: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        user_identity: Option<&str>,
        source_id: Option<&str>,
    ) -> Result<types::HistoryDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .query_history(
                        offset,
                        count,
                        sort_column,
                        sort_order,
                        start_date,
                        end_date,
                        user_identity,
                        source_id,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .query_history(
                        offset,
                        count,
                        sort_column,
                        sort_order,
                        start_date,
                        end_date,
                        user_identity,
                        source_id,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .query_history(
                        offset,
                        count,
                        sort_column,
                        sort_order,
                        start_date,
                        end_date,
                        user_identity,
                        source_id,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Schedule or unschedule components in the specified Process Group.
    pub async fn schedule_components(
        &self,
        id: &str,
        body: types::ScheduleComponentsEntity,
    ) -> Result<types::ScheduleComponentsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .schedule_components(
                        id,
                        &crate::v2_6_0::types::ScheduleComponentsEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .schedule_components(
                        id,
                        &crate::v2_7_2::types::ScheduleComponentsEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api
                    .schedule_components(
                        id,
                        &crate::v2_8_0::types::ScheduleComponentsEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Searches the cluster for a node with the specified address
    pub async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<types::ClusterSearchResultsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_cluster(q).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_cluster(q).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_cluster(q).await?.into())
            }
        }
    }
    /// Performs a search against this NiFi using the specified search term
    pub async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<types::SearchResultsDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_flow(q, a).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_flow(q, a).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flow::FlowApi {
                    client: self.client,
                };
                Ok(api.search_flow(q, a).await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the FlowFileQueues API.
pub struct DynamicFlowFileQueuesApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicFlowFileQueuesApi<'a> {
    /// Creates a request to drop the contents of the queue in this connection.
    pub async fn create_drop_request(&self, id: &str) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_drop_request().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_drop_request().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_drop_request().await?.into())
            }
        }
    }
    /// Lists the contents of the queue in this connection.
    pub async fn create_flow_file_listing(
        &self,
        id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
        }
    }
    /// Cancels and/or removes a request to list the contents of this connection.
    pub async fn delete_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
        }
    }
    /// Gets the content for a FlowFile in a Connection.
    pub async fn download_flow_file_content(
        &self,
        id: &str,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
        }
    }
    /// Gets the current status of a drop request for the specified connection.
    pub async fn get_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
        }
    }
    /// Gets a FlowFile from a Connection.
    pub async fn get_flow_file(
        &self,
        id: &str,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::FlowFileDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the current status of a listing request for the specified connection.
    pub async fn get_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
        }
    }
    /// Cancels and/or removes a request to drop the contents of this connection.
    pub async fn remove_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Funnels API.
pub struct DynamicFunnelsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicFunnelsApi<'a> {
    /// Gets a funnel
    pub async fn get_funnel(&self, id: &str) -> Result<types::FunnelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api.get_funnel(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api.get_funnel(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api.get_funnel(id).await?.into())
            }
        }
    }
    /// Deletes a funnel
    pub async fn remove_funnel(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_funnel(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Updates a funnel
    pub async fn update_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_funnel(id, &crate::v2_6_0::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_funnel(id, &crate::v2_7_2::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::funnels::FunnelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_funnel(id, &crate::v2_8_0::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the InputPorts API.
pub struct DynamicInputPortsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicInputPortsApi<'a> {
    /// Clears bulletins for an input port
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_2(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_2".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_2(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_2(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets an input port
    pub async fn get_input_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api.get_input_port(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api.get_input_port(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api.get_input_port(id).await?.into())
            }
        }
    }
    /// Deletes an input port
    pub async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_input_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Updates an input port
    pub async fn update_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_input_port(id, &crate::v2_6_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_input_port(id, &crate::v2_7_2::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_input_port(id, &crate::v2_8_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of an input-port
    pub async fn update_run_status_2(
        &self,
        id: &str,
        body: types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_6_0::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_7_2::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::inputports::InputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_2(&crate::v2_8_0::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Labels API.
pub struct DynamicLabelsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicLabelsApi<'a> {
    /// Gets a label
    pub async fn get_label(&self, id: &str) -> Result<types::LabelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api.get_label(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api.get_label(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api.get_label(id).await?.into())
            }
        }
    }
    /// Deletes a label
    pub async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::LabelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_label(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Updates a label
    pub async fn update_label(
        &self,
        id: &str,
        body: types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_label(id, &crate::v2_6_0::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_label(id, &crate::v2_7_2::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::labels::LabelsApi {
                    client: self.client,
                };
                Ok(api
                    .update_label(id, &crate::v2_8_0::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the OutputPorts API.
pub struct DynamicOutputPortsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicOutputPortsApi<'a> {
    /// Clears bulletins for an output port
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_3(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_3".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_3(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_3(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets an output port
    pub async fn get_output_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api.get_output_port(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api.get_output_port(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api.get_output_port(id).await?.into())
            }
        }
    }
    /// Deletes an output port
    pub async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_output_port(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Updates an output port
    pub async fn update_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_output_port(id, &crate::v2_6_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_output_port(id, &crate::v2_7_2::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsApi {
                    client: self.client,
                };
                Ok(api
                    .update_output_port(id, &crate::v2_8_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of an output-port
    pub async fn update_run_status_3(
        &self,
        id: &str,
        body: types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_6_0::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_7_2::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::outputports::OutputPortsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_3(&crate::v2_8_0::types::PortRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the ParameterContexts API.
pub struct DynamicParameterContextsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicParameterContextsApi<'a> {
    /// Creates a new Asset in the given Parameter Context
    pub async fn create_asset(
        &self,
        context_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::AssetDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.create_asset(filename, data).await?.into())
            }
        }
    }
    /// Create a Parameter Context
    pub async fn create_parameter_context(
        &self,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_context(
                        &crate::v2_6_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_context(
                        &crate::v2_7_2::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .create_parameter_context(
                        &crate::v2_8_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Deletes an Asset from the given Parameter Context
    pub async fn delete_asset(
        &self,
        context_id: &str,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AssetDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api
                    .delete_asset(asset_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Parameter Context with the given ID
    pub async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_parameter_context(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_parameter_context(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_parameter_context(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Update Request with the given ID
    pub async fn delete_update_request(
        &self,
        context_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_update_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Validation Request with the given ID
    pub async fn delete_validation_request(
        &self,
        context_id: &str,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .delete_validation_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Retrieves the content of the asset with the given id
    pub async fn get_asset_content(
        &self,
        context_id: &str,
        asset_id: &str,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                api.get_asset_content(asset_id).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                api.get_asset_content(asset_id).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                api.get_asset_content(asset_id).await
            }
        }
    }
    /// Lists the assets that belong to the Parameter Context with the given ID
    pub async fn get_assets(&self, context_id: &str) -> Result<types::AssetsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.get_assets().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.get_assets().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsAssetsApi {
                    client: self.client,
                    context_id,
                };
                Ok(api.get_assets().await?.into())
            }
        }
    }
    /// Returns the Parameter Context with the given ID
    pub async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_context(id, include_inherited_parameters)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_context(id, include_inherited_parameters)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .get_parameter_context(id, include_inherited_parameters)
                    .await?
                    .into())
            }
        }
    }
    /// Returns the Update Request with the given ID
    pub async fn get_parameter_context_update(
        &self,
        context_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_parameter_context_update(request_id).await?.into())
            }
        }
    }
    /// Returns the Validation Request with the given ID
    pub async fn get_validation_request(
        &self,
        context_id: &str,
        id: &str,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api.get_validation_request(id).await?.into())
            }
        }
    }
    /// Initiate the Update Request of a Parameter Context
    pub async fn submit_parameter_context_update(
        &self,
        context_id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_6_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_7_2::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsUpdateRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_parameter_context_update(
                        &crate::v2_8_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated
    pub async fn submit_validation_request(
        &self,
        context_id: &str,
        body: types::ParameterContextValidationRequestEntity,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_6_0::types::ParameterContextValidationRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_7_2::types::ParameterContextValidationRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parametercontexts::ParameterContextsValidationRequestsApi {
                        client: self.client,
                        context_id,
                    };
                Ok(api
                    .submit_validation_request(
                        &crate::v2_8_0::types::ParameterContextValidationRequestEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Modifies a Parameter Context
    pub async fn update_parameter_context(
        &self,
        id: &str,
        body: types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_context(
                        id,
                        &crate::v2_6_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_context(
                        id,
                        &crate::v2_7_2::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parametercontexts::ParameterContextsApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_context(
                        id,
                        &crate::v2_8_0::types::ParameterContextEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the ParameterProviders API.
pub struct DynamicParameterProvidersApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicParameterProvidersApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    pub async fn analyze_configuration_1(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_1(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a parameter provider
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_4(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_4".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_4(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_4(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears the state for a parameter provider
    pub async fn clear_state_2(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_2(&crate::v2_8_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Apply Parameters Request with the given ID
    pub async fn delete_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .delete_apply_parameters_request(request_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    pub async fn delete_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_1(request_id).await?.into())
            }
        }
    }
    /// Fetches and temporarily caches the parameters for a provider
    pub async fn fetch_parameters(
        &self,
        id: &str,
        body: types::ParameterProviderParameterFetchEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_6_0::types::ParameterProviderParameterFetchEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_7_2::types::ParameterProviderParameterFetchEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersParametersApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .fetch_parameters(
                        &crate::v2_8_0::types::ParameterProviderParameterFetchEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets a parameter provider
    pub async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api.get_parameter_provider(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api.get_parameter_provider(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api.get_parameter_provider(id).await?.into())
            }
        }
    }
    /// Returns the Apply Parameters Request with the given ID
    pub async fn get_parameter_provider_apply_parameters_request(
        &self,
        provider_id: &str,
        request_id: &str,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(api
                    .get_parameter_provider_apply_parameters_request(request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all references to a parameter provider
    pub async fn get_parameter_provider_references(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderReferencingComponentsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersReferencesApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_parameter_provider_references().await?.into())
            }
        }
    }
    /// Gets a parameter provider property descriptor
    pub async fn get_property_descriptor_2(
        &self,
        id: &str,
        property_name: &str,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::parameterproviders::ParameterProvidersDescriptorsApi {
                        client: self.client,
                        id,
                    };
                Ok(api.get_property_descriptor_2(property_name).await?.into())
            }
        }
    }
    /// Gets the state for a parameter provider
    pub async fn get_state_1(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_1().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_1().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_1().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    pub async fn get_verification_request_1(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_1(request_id).await?.into())
            }
        }
    }
    /// Deletes a parameter provider
    pub async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .remove_parameter_provider(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .remove_parameter_provider(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .remove_parameter_provider(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Initiate a request to apply the fetched parameters of a Parameter Provider
    pub async fn submit_apply_parameters(
        &self,
        provider_id: &str,
        body: types::ParameterProviderParameterApplicationEntity,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_6_0::types::ParameterProviderParameterApplicationEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_7_2::types::ParameterProviderParameterApplicationEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApplyParametersRequestsApi {
                    client: self.client,
                    provider_id,
                };
                Ok(
                    api
                        .submit_apply_parameters(
                            &crate::v2_8_0::types::ParameterProviderParameterApplicationEntity::try_from(
                                body,
                            )?,
                        )
                        .await?
                        .into(),
                )
            }
        }
    }
    /// Performs verification of the Parameter Provider's configuration
    pub async fn submit_config_verification_request_1(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_1(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a parameter provider
    pub async fn update_parameter_provider(
        &self,
        id: &str,
        body: types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_provider(
                        id,
                        &crate::v2_6_0::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_provider(
                        id,
                        &crate::v2_7_2::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::parameterproviders::ParameterProvidersApi {
                    client: self.client,
                };
                Ok(api
                    .update_parameter_provider(
                        id,
                        &crate::v2_8_0::types::ParameterProviderEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Policies API.
pub struct DynamicPoliciesApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicPoliciesApi<'a> {
    /// Creates an access policy
    pub async fn create_access_policy(
        &self,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .create_access_policy(&crate::v2_6_0::types::AccessPolicyEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .create_access_policy(&crate::v2_7_2::types::AccessPolicyEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .create_access_policy(&crate::v2_8_0::types::AccessPolicyEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
    /// Gets an access policy
    pub async fn get_access_policy(
        &self,
        id: &str,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api.get_access_policy(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api.get_access_policy(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api.get_access_policy(id).await?.into())
            }
        }
    }
    /// Gets an access policy for the specified action and resource
    pub async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .get_access_policy_for_resource(action, resource)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .get_access_policy_for_resource(action, resource)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .get_access_policy_for_resource(action, resource)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes an access policy
    pub async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Updates a access policy
    pub async fn update_access_policy(
        &self,
        id: &str,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .update_access_policy(
                        id,
                        &crate::v2_6_0::types::AccessPolicyEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .update_access_policy(
                        id,
                        &crate::v2_7_2::types::AccessPolicyEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::policies::PoliciesApi {
                    client: self.client,
                };
                Ok(api
                    .update_access_policy(
                        id,
                        &crate::v2_8_0::types::AccessPolicyEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the ProcessGroups API.
pub struct DynamicProcessGroupsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicProcessGroupsApi<'a> {
    /// Generates a copy response for the given copy request
    pub async fn copy(
        &self,
        id: &str,
        body: types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy(&crate::v2_6_0::types::CopyRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy(&crate::v2_7_2::types::CopyRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsCopyApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy(&crate::v2_8_0::types::CopyRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Copies a snippet and discards it.
    pub async fn copy_snippet(
        &self,
        id: &str,
        body: types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_6_0::types::CopySnippetRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_7_2::types::CopySnippetRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsSnippetInstanceApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .copy_snippet(&crate::v2_8_0::types::CopySnippetRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a connection
    pub async fn create_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_connection(&crate::v2_6_0::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_connection(&crate::v2_7_2::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_connection(&crate::v2_8_0::types::ConnectionEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new controller service
    pub async fn create_controller_service_1(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_6_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_7_2::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsControllerServicesApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_controller_service_1(
                        &crate::v2_8_0::types::ControllerServiceEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a request to drop all flowfiles of all connection queues in this process group.
    pub async fn create_empty_all_connections_request(
        &self,
        id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.create_empty_all_connections_request().await?.into())
            }
        }
    }
    /// Creates a funnel
    pub async fn create_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_funnel(&crate::v2_6_0::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_funnel(&crate::v2_7_2::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_funnel(&crate::v2_8_0::types::FunnelEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates an input port
    pub async fn create_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_input_port(&crate::v2_6_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_input_port(&crate::v2_7_2::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_input_port(&crate::v2_8_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a label
    pub async fn create_label(
        &self,
        id: &str,
        body: types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_label(&crate::v2_6_0::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_label(&crate::v2_7_2::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_label(&crate::v2_8_0::types::LabelEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates an output port
    pub async fn create_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_output_port(&crate::v2_6_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_output_port(&crate::v2_7_2::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_output_port(&crate::v2_8_0::types::PortEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a process group
    pub async fn create_process_group(
        &self,
        id: &str,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_6_0::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_6_0::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_7_2::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_7_2::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_process_group(
                        parameter_context_handling_strategy
                            .map(crate::v2_8_0::types::ParameterContextHandlingStrategy::try_from)
                            .transpose()?,
                        &crate::v2_8_0::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new processor
    pub async fn create_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_processor(&crate::v2_6_0::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_processor(&crate::v2_7_2::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_processor(&crate::v2_8_0::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a new process group
    pub async fn create_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_6_0::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_7_2::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .create_remote_process_group(
                        &crate::v2_8_0::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Replace Request with the given ID
    pub async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_replace_process_group_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Gets a process group for download
    pub async fn export_process_group(
        &self,
        id: &str,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_process_group(include_referenced_services).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_process_group(include_referenced_services).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_process_group(include_referenced_services).await
            }
        }
    }
    /// Gets all connections
    pub async fn get_connections(&self, id: &str) -> Result<types::ConnectionsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connections().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connections().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsConnectionsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_connections().await?.into())
            }
        }
    }
    /// Gets the current status of a drop all flowfiles request.
    pub async fn get_drop_all_flowfiles_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_drop_all_flowfiles_request(drop_request_id)
                    .await?
                    .into())
            }
        }
    }
    /// Gets all funnels
    pub async fn get_funnels(&self, id: &str) -> Result<types::FunnelsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_funnels().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_funnels().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFunnelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_funnels().await?.into())
            }
        }
    }
    /// Gets all input ports
    pub async fn get_input_ports(&self, id: &str) -> Result<types::InputPortsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_input_ports().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_input_ports().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsInputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_input_ports().await?.into())
            }
        }
    }
    /// Gets all labels
    pub async fn get_labels(&self, id: &str) -> Result<types::LabelsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_labels().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_labels().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLabelsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_labels().await?.into())
            }
        }
    }
    /// Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
    pub async fn get_local_modifications(
        &self,
        id: &str,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsLocalModificationsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_local_modifications().await?.into())
            }
        }
    }
    /// Gets all output ports
    pub async fn get_output_ports(&self, id: &str) -> Result<types::OutputPortsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_output_ports().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_output_ports().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsOutputPortsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_output_ports().await?.into())
            }
        }
    }
    /// Gets a process group
    pub async fn get_process_group(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_process_group(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_process_group(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_process_group(id).await?.into())
            }
        }
    }
    /// Gets all process groups
    pub async fn get_process_groups(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_groups().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_groups().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_process_groups().await?.into())
            }
        }
    }
    /// Gets all processors
    pub async fn get_processors(
        &self,
        id: &str,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessorsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processors(include_descendant_groups).await?.into())
            }
        }
    }
    /// Gets all remote process groups
    pub async fn get_remote_process_groups(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsRemoteProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_remote_process_groups().await?.into())
            }
        }
    }
    /// Returns the Replace Request with the given ID
    pub async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_replace_process_group_request(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_replace_process_group_request(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_replace_process_group_request(id).await?.into())
            }
        }
    }
    /// Imports a specified process group
    pub async fn import_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_6_0::types::ProcessGroupUploadEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_7_2::types::ProcessGroupUploadEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .import_process_group(
                        &crate::v2_8_0::types::ProcessGroupUploadEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Initiate the Replace Request of a Process Group with the given ID
    pub async fn initiate_replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_6_0::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_7_2::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsReplaceRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .initiate_replace_process_group(
                        &crate::v2_8_0::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Pastes into the specified process group
    pub async fn paste(
        &self,
        id: &str,
        body: types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .paste(&crate::v2_6_0::types::PasteRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .paste(&crate::v2_7_2::types::PasteRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsPasteApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .paste(&crate::v2_8_0::types::PasteRequestEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Cancels and/or removes a request to drop all flowfiles.
    pub async fn remove_drop_request_1(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsEmptyAllConnectionsRequestsApi {
                    client: self.client,
                    id,
                };
                Ok(api.remove_drop_request_1(drop_request_id).await?.into())
            }
        }
    }
    /// Deletes a process group
    pub async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_process_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Replace Process Group contents with the given ID with the specified Process Group contents
    pub async fn replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_6_0::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_7_2::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsFlowContentsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .replace_process_group(
                        &crate::v2_8_0::types::ProcessGroupImportEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a process group
    pub async fn update_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_process_group(
                        id,
                        &crate::v2_6_0::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_process_group(
                        id,
                        &crate::v2_7_2::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_process_group(
                        id,
                        &crate::v2_8_0::types::ProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Uploads a versioned flow definition and creates a process group
    pub async fn upload_process_group(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.upload_process_group().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.upload_process_group().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processgroups::ProcessGroupsProcessGroupsApi {
                    client: self.client,
                    id,
                };
                Ok(api.upload_process_group().await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Processors API.
pub struct DynamicProcessorsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicProcessorsApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    pub async fn analyze_configuration_2(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_2(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a processor
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_5(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_5".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_5(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_5(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears the state for a processor
    pub async fn clear_state_3(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_3(&crate::v2_8_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes a processor
    pub async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_processor(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    pub async fn delete_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_2(request_id).await?.into())
            }
        }
    }
    /// Gets a processor
    pub async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api.get_processor(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api.get_processor(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api.get_processor(id).await?.into())
            }
        }
    }
    /// Gets diagnostics information about a processor
    pub async fn get_processor_diagnostics(
        &self,
        id: &str,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsDiagnosticsApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_processor_diagnostics().await?.into())
            }
        }
    }
    /// Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs
    pub async fn get_processor_run_status_details(
        &self,
        body: types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_run_status_details(
                        &crate::v2_6_0::types::RunStatusDetailsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_run_status_details(
                        &crate::v2_7_2::types::RunStatusDetailsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .get_processor_run_status_details(
                        &crate::v2_8_0::types::RunStatusDetailsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets the descriptor for a processor property
    pub async fn get_property_descriptor_3(
        &self,
        id: &str,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_3(client_id, property_name, sensitive)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the state for a processor
    pub async fn get_state_2(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_2().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_2().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_2().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    pub async fn get_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_2(request_id).await?.into())
            }
        }
    }
    /// Performs verification of the Processor's configuration
    pub async fn submit_processor_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_processor_verification_request(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Terminates a processor, essentially "deleting" its threads and any active tasks
    pub async fn terminate_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id,
                };
                Ok(api.terminate_processor().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id,
                };
                Ok(api.terminate_processor().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsThreadsApi {
                    client: self.client,
                    id,
                };
                Ok(api.terminate_processor().await?.into())
            }
        }
    }
    /// Updates a processor
    pub async fn update_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .update_processor(id, &crate::v2_6_0::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .update_processor(id, &crate::v2_7_2::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsApi {
                    client: self.client,
                };
                Ok(api
                    .update_processor(id, &crate::v2_8_0::types::ProcessorEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a processor
    pub async fn update_run_status_4(
        &self,
        id: &str,
        body: types::ProcessorRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_6_0::types::ProcessorRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_7_2::types::ProcessorRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::processors::ProcessorsRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_4(&crate::v2_8_0::types::ProcessorRunStatusEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Provenance API.
pub struct DynamicProvenanceApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicProvenanceApi<'a> {
    /// Deletes a lineage query
    pub async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_lineage(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_lineage(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_lineage(id, cluster_node_id).await?.into())
            }
        }
    }
    /// Deletes a provenance query
    pub async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_provenance(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_provenance(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.delete_provenance(id, cluster_node_id).await?.into())
            }
        }
    }
    /// Gets a lineage query
    pub async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_lineage(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_lineage(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_lineage(id, cluster_node_id).await?.into())
            }
        }
    }
    /// Gets a provenance query
    pub async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the searchable attributes for provenance events
    pub async fn get_search_options(&self) -> Result<types::ProvenanceOptionsDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_search_options().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_search_options().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api.get_search_options().await?.into())
            }
        }
    }
    /// Submits a lineage query
    pub async fn submit_lineage_request(
        &self,
        body: types::LineageEntity,
    ) -> Result<types::LineageDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_lineage_request(&crate::v2_6_0::types::LineageEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_lineage_request(&crate::v2_7_2::types::LineageEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_lineage_request(&crate::v2_8_0::types::LineageEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Submits a provenance query
    pub async fn submit_provenance_request(
        &self,
        body: types::ProvenanceEntity,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_provenance_request(&crate::v2_6_0::types::ProvenanceEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_provenance_request(&crate::v2_7_2::types::ProvenanceEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenance::ProvenanceApi {
                    client: self.client,
                };
                Ok(api
                    .submit_provenance_request(&crate::v2_8_0::types::ProvenanceEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the ProvenanceEvents API.
pub struct DynamicProvenanceEventsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicProvenanceEventsApi<'a> {
    /// Gets the input content for a provenance event
    pub async fn get_input_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_input_content(cluster_node_id).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_input_content(cluster_node_id).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_input_content(cluster_node_id).await
            }
        }
    }
    /// Retrieves the latest cached Provenance Events for the specified component
    pub async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<types::LatestProvenanceEventsDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .get_latest_provenance_events(component_id, limit)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .get_latest_provenance_events(component_id, limit)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .get_latest_provenance_events(component_id, limit)
                    .await?
                    .into())
            }
        }
    }
    /// Gets the output content for a provenance event
    pub async fn get_output_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_output_content(cluster_node_id).await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_output_content(cluster_node_id).await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsContentApi {
                    client: self.client,
                    id,
                };
                api.get_output_content(cluster_node_id).await
            }
        }
    }
    /// Gets a provenance event
    pub async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api.get_provenance_event(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api.get_provenance_event(id, cluster_node_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api.get_provenance_event(id, cluster_node_id).await?.into())
            }
        }
    }
    /// Replays content from a provenance event
    pub async fn submit_replay(
        &self,
        body: types::SubmitReplayRequestEntity,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay(&crate::v2_6_0::types::SubmitReplayRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay(&crate::v2_7_2::types::SubmitReplayRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay(&crate::v2_8_0::types::SubmitReplayRequestEntity::try_from(
                        body,
                    )?)
                    .await?
                    .into())
            }
        }
    }
    /// Replays content from a provenance event
    pub async fn submit_replay_latest_event(
        &self,
        body: types::ReplayLastEventRequestEntity,
    ) -> Result<types::ReplayLastEventResponseEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay_latest_event(
                        &crate::v2_6_0::types::ReplayLastEventRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay_latest_event(
                        &crate::v2_7_2::types::ReplayLastEventRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
                    client: self.client,
                };
                Ok(api
                    .submit_replay_latest_event(
                        &crate::v2_8_0::types::ReplayLastEventRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the RemoteProcessGroups API.
pub struct DynamicRemoteProcessGroupsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicRemoteProcessGroupsApi<'a> {
    /// Clears bulletins for a remote process group
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_6(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_6".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsBulletinsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .clear_bulletins_6(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsBulletinsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .clear_bulletins_6(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Gets a remote process group
    pub async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_remote_process_group(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_remote_process_group(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api.get_remote_process_group(id).await?.into())
            }
        }
    }
    /// Gets the state for a RemoteProcessGroup
    pub async fn get_state_3(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_3().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_3().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_3().await?.into())
            }
        }
    }
    /// Deletes a remote process group
    pub async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_remote_process_group(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_remote_process_group(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_remote_process_group(
                        id,
                        version,
                        client_id,
                        disconnected_node_acknowledged,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a remote process group
    pub async fn update_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_remote_process_group(
                        id,
                        &crate::v2_6_0::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_remote_process_group(
                        id,
                        &crate::v2_7_2::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsApi {
                    client: self.client,
                };
                Ok(api
                    .update_remote_process_group(
                        id,
                        &crate::v2_8_0::types::RemoteProcessGroupEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a remote port
    pub async fn update_remote_process_group_input_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_7_2::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port(
                        port_id,
                        &crate::v2_8_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a remote port
    pub async fn update_remote_process_group_input_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsInputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_input_port_run_status(
                        port_id,
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a remote port
    pub async fn update_remote_process_group_output_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_6_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_7_2::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port(
                        port_id,
                        &crate::v2_8_0::types::RemoteProcessGroupPortEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a remote port
    pub async fn update_remote_process_group_output_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsOutputPortsApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_output_port_run_status(
                        port_id,
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a remote process group
    pub async fn update_remote_process_group_run_status(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_status(
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of all remote process groups in a process group (recursively)
    pub async fn update_remote_process_group_run_statuses(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api =
                    crate::v2_6_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_6_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api =
                    crate::v2_7_2::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_7_2::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api =
                    crate::v2_8_0::api::remoteprocessgroups::RemoteProcessGroupsRunStatusApi {
                        client: self.client,
                        id,
                    };
                Ok(api
                    .update_remote_process_group_run_statuses(
                        &crate::v2_8_0::types::RemotePortRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the ReportingTasks API.
pub struct DynamicReportingTasksApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicReportingTasksApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    pub async fn analyze_configuration_3(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_7_2::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .analyze_configuration_3(
                        &crate::v2_8_0::types::ConfigurationAnalysisEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears bulletins for a reporting task
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    pub async fn clear_bulletins_7(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => Err(NifiError::UnsupportedEndpoint {
                endpoint: "clear_bulletins_7".to_string(),
                version: "2.6.0".to_string(),
            }),
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_7(
                        &crate::v2_7_2::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksBulletinsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_bulletins_7(
                        &crate::v2_8_0::types::ClearBulletinsRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Clears the state for a reporting task
    pub async fn clear_state_4(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_7_2::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .clear_state_4(&crate::v2_8_0::types::ComponentStateEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Verification Request with the given ID
    pub async fn delete_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.delete_verification_request_3(request_id).await?.into())
            }
        }
    }
    /// Gets a reporting task property descriptor
    pub async fn get_property_descriptor_4(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksDescriptorsApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .get_property_descriptor_4(property_name, sensitive)
                    .await?
                    .into())
            }
        }
    }
    /// Gets a reporting task
    pub async fn get_reporting_task(
        &self,
        id: &str,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api.get_reporting_task(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api.get_reporting_task(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api.get_reporting_task(id).await?.into())
            }
        }
    }
    /// Gets the state for a reporting task
    pub async fn get_state_4(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_4().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_4().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksStateApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_state_4().await?.into())
            }
        }
    }
    /// Returns the Verification Request with the given ID
    pub async fn get_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api.get_verification_request_3(request_id).await?.into())
            }
        }
    }
    /// Deletes a reporting task
    pub async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Performs verification of the Reporting Task's configuration
    pub async fn submit_config_verification_request_2(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_7_2::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksConfigApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .submit_config_verification_request_2(
                        &crate::v2_8_0::types::VerifyConfigRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates a reporting task
    pub async fn update_reporting_task(
        &self,
        id: &str,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .update_reporting_task(
                        id,
                        &crate::v2_6_0::types::ReportingTaskEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .update_reporting_task(
                        id,
                        &crate::v2_7_2::types::ReportingTaskEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksApi {
                    client: self.client,
                };
                Ok(api
                    .update_reporting_task(
                        id,
                        &crate::v2_8_0::types::ReportingTaskEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates run status of a reporting task
    pub async fn update_run_status_5(
        &self,
        id: &str,
        body: types::ReportingTaskRunStatusEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_6_0::types::ReportingTaskRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_7_2::types::ReportingTaskRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::reportingtasks::ReportingTasksRunStatusApi {
                    client: self.client,
                    id,
                };
                Ok(api
                    .update_run_status_5(
                        &crate::v2_8_0::types::ReportingTaskRunStatusEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Resources API.
pub struct DynamicResourcesApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicResourcesApi<'a> {
    /// Gets the available resources that support access/authorization policies
    pub async fn get_resources(&self) -> Result<types::ResourcesEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::resources::ResourcesApi {
                    client: self.client,
                };
                Ok(api.get_resources().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::resources::ResourcesApi {
                    client: self.client,
                };
                Ok(api.get_resources().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::resources::ResourcesApi {
                    client: self.client,
                };
                Ok(api.get_resources().await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the SiteToSite API.
pub struct DynamicSiteToSiteApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicSiteToSiteApi<'a> {
    /// Returns the available Peers and its status of this NiFi
    pub async fn get_peers(&self) -> Result<types::PeersEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_peers().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_peers().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_peers().await?.into())
            }
        }
    }
    /// Returns the details about this NiFi necessary to communicate via site to site
    pub async fn get_site_to_site_details(&self) -> Result<types::ControllerDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_site_to_site_details().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_site_to_site_details().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::sitetosite::SiteToSiteApi {
                    client: self.client,
                };
                Ok(api.get_site_to_site_details().await?.into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Snippets API.
pub struct DynamicSnippetsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicSnippetsApi<'a> {
    /// Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.
    pub async fn create_snippet(
        &self,
        body: types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .create_snippet(&crate::v2_6_0::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .create_snippet(&crate::v2_7_2::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .create_snippet(&crate::v2_8_0::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the components in a snippet and discards the snippet
    pub async fn delete_snippet(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_snippet(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_snippet(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_snippet(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Move's the components in this Snippet into a new Process Group and discards the snippet
    pub async fn update_snippet(
        &self,
        id: &str,
        body: types::SnippetEntity,
    ) -> Result<types::SnippetEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .update_snippet(id, &crate::v2_6_0::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .update_snippet(id, &crate::v2_7_2::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::snippets::SnippetsApi {
                    client: self.client,
                };
                Ok(api
                    .update_snippet(id, &crate::v2_8_0::types::SnippetEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the SystemDiagnostics API.
pub struct DynamicSystemDiagnosticsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicSystemDiagnosticsApi<'a> {
    /// Retrieve available JMX metrics
    pub async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<types::JmxMetricsResultsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api.get_jmx_metrics(bean_name_filter).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api.get_jmx_metrics(bean_name_filter).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api.get_jmx_metrics(bean_name_filter).await?.into())
            }
        }
    }
    /// Gets the diagnostics for the system NiFi is running on
    pub async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::SystemDiagnosticsDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api
                    .get_system_diagnostics(
                        nodewise,
                        diagnostic_level
                            .map(crate::v2_6_0::types::DiagnosticLevel::try_from)
                            .transpose()?,
                        cluster_node_id,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api
                    .get_system_diagnostics(
                        nodewise,
                        diagnostic_level
                            .map(crate::v2_7_2::types::DiagnosticLevel::try_from)
                            .transpose()?,
                        cluster_node_id,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::systemdiagnostics::SystemDiagnosticsApi {
                    client: self.client,
                };
                Ok(api
                    .get_system_diagnostics(
                        nodewise,
                        diagnostic_level
                            .map(crate::v2_8_0::types::DiagnosticLevel::try_from)
                            .transpose()?,
                        cluster_node_id,
                    )
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Tenants API.
pub struct DynamicTenantsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicTenantsApi<'a> {
    /// Creates a user
    pub async fn create_user(
        &self,
        body: types::UserEntity,
    ) -> Result<types::UserEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user(&crate::v2_6_0::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user(&crate::v2_7_2::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user(&crate::v2_8_0::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Creates a user group
    pub async fn create_user_group(
        &self,
        body: types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user_group(&crate::v2_6_0::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user_group(&crate::v2_7_2::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .create_user_group(&crate::v2_8_0::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Gets a user
    pub async fn get_user(&self, id: &str) -> Result<types::UserEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user(id).await?.into())
            }
        }
    }
    /// Gets a user group
    pub async fn get_user_group(&self, id: &str) -> Result<types::UserGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_group(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_group(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_group(id).await?.into())
            }
        }
    }
    /// Gets all user groups
    pub async fn get_user_groups(&self) -> Result<types::UserGroupsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_groups().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_groups().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_user_groups().await?.into())
            }
        }
    }
    /// Gets all users
    pub async fn get_users(&self) -> Result<types::UsersEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_users().await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_users().await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.get_users().await?.into())
            }
        }
    }
    /// Deletes a user
    pub async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes a user group
    pub async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Searches for a tenant with the specified identity
    pub async fn search_tenants(&self, q: &str) -> Result<types::TenantsEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.search_tenants(q).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.search_tenants(q).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api.search_tenants(q).await?.into())
            }
        }
    }
    /// Updates a user
    pub async fn update_user(
        &self,
        id: &str,
        body: types::UserEntity,
    ) -> Result<types::UserEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user(id, &crate::v2_6_0::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user(id, &crate::v2_7_2::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user(id, &crate::v2_8_0::types::UserEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
    /// Updates a user group
    pub async fn update_user_group(
        &self,
        id: &str,
        body: types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user_group(id, &crate::v2_6_0::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user_group(id, &crate::v2_7_2::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::tenants::TenantsApi {
                    client: self.client,
                };
                Ok(api
                    .update_user_group(id, &crate::v2_8_0::types::UserGroupEntity::try_from(body)?)
                    .await?
                    .into())
            }
        }
    }
}
/// Dynamic dispatch wrapper for the Versions API.
pub struct DynamicVersionsApi<'a> {
    client: &'a NifiClient,
    version: DetectedVersion,
}
#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]
impl<'a> DynamicVersionsApi<'a> {
    /// Create a version control request
    pub async fn create_version_control_request(
        &self,
        body: types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                api.create_version_control_request(
                    &crate::v2_6_0::types::CreateActiveRequestEntity::try_from(body)?,
                )
                .await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                api.create_version_control_request(
                    &crate::v2_7_2::types::CreateActiveRequestEntity::try_from(body)?,
                )
                .await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                api.create_version_control_request(
                    &crate::v2_8_0::types::CreateActiveRequestEntity::try_from(body)?,
                )
                .await
            }
        }
    }
    /// Deletes the Revert Request with the given ID
    pub async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_revert_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_revert_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_revert_request(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the Update Request with the given ID
    pub async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_update_request_1(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_update_request_1(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .delete_update_request_1(id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Deletes the version control request with the given ID
    pub async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                api.delete_version_control_request(id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    /// Gets the latest version of a Process Group for download
    pub async fn export_flow_version(&self, id: &str) -> Result<(), NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_flow_version().await
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_flow_version().await
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsDownloadApi {
                    client: self.client,
                    id,
                };
                api.export_flow_version().await
            }
        }
    }
    /// Returns the Revert Request with the given ID
    pub async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_revert_request(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_revert_request(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_revert_request(id).await?.into())
            }
        }
    }
    /// Returns the Update Request with the given ID
    pub async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_update_request(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_update_request(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_update_request(id).await?.into())
            }
        }
    }
    /// Gets the Version Control information for a process group
    pub async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_version_information(id).await?.into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_version_information(id).await?.into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api.get_version_information(id).await?.into())
            }
        }
    }
    /// Initiate the Revert Request of a Process Group with the given ID
    pub async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_revert_flow_version(
                        id,
                        &crate::v2_6_0::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_revert_flow_version(
                        id,
                        &crate::v2_7_2::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_revert_flow_version(
                        id,
                        &crate::v2_8_0::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Initiate the Update Request of a Process Group with the given ID
    pub async fn initiate_version_control_update(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_version_control_update(
                        id,
                        &crate::v2_6_0::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_version_control_update(
                        id,
                        &crate::v2_7_2::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .initiate_version_control_update(
                        id,
                        &crate::v2_8_0::types::VersionControlInformationEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Save the Process Group with the given ID
    pub async fn save_to_flow_registry(
        &self,
        id: &str,
        body: types::StartVersionControlRequestEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .save_to_flow_registry(
                        id,
                        &crate::v2_6_0::types::StartVersionControlRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .save_to_flow_registry(
                        id,
                        &crate::v2_7_2::types::StartVersionControlRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .save_to_flow_registry(
                        id,
                        &crate::v2_8_0::types::StartVersionControlRequestEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Stops version controlling the Process Group with the given ID
    pub async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .stop_version_control(id, version, client_id, disconnected_node_acknowledged)
                    .await?
                    .into())
            }
        }
    }
    /// Update the version of a Process Group with the given ID
    pub async fn update_flow_version(
        &self,
        id: &str,
        body: types::VersionedFlowSnapshotEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_version(
                        id,
                        &crate::v2_6_0::types::VersionedFlowSnapshotEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_version(
                        id,
                        &crate::v2_7_2::types::VersionedFlowSnapshotEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_flow_version(
                        id,
                        &crate::v2_8_0::types::VersionedFlowSnapshotEntity::try_from(body)?,
                    )
                    .await?
                    .into())
            }
        }
    }
    /// Updates the request with the given ID
    pub async fn update_version_control_request(
        &self,
        id: &str,
        body: types::VersionControlComponentMappingEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_version_control_request(
                        id,
                        &crate::v2_6_0::types::VersionControlComponentMappingEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_version_control_request(
                        id,
                        &crate::v2_7_2::types::VersionControlComponentMappingEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
            DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::versions::VersionsApi {
                    client: self.client,
                };
                Ok(api
                    .update_version_control_request(
                        id,
                        &crate::v2_8_0::types::VersionControlComponentMappingEntity::try_from(
                            body,
                        )?,
                    )
                    .await?
                    .into())
            }
        }
    }
}

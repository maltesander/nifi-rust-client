// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

mod conversions;
pub mod dispatch;
mod impls;
pub mod traits;
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
    pub fn access_api(&self) -> dispatch::AccessApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::AccessApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0AccessApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::AccessApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2AccessApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::AccessApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0AccessApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Authentication API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn authentication_api(&self) -> dispatch::AuthenticationApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::AuthenticationApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0AuthenticationApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::AuthenticationApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2AuthenticationApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::AuthenticationApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0AuthenticationApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Connections API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn connections_api(&self) -> dispatch::ConnectionsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ConnectionsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ConnectionsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ConnectionsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ConnectionsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ConnectionsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ConnectionsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Controller API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn controller_api(&self) -> dispatch::ControllerApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ControllerApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ControllerApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ControllerApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ControllerApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ControllerApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ControllerApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Controller Services API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn controller_services_api(&self) -> dispatch::ControllerServicesApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::ControllerServicesApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0ControllerServicesApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::ControllerServicesApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2ControllerServicesApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::ControllerServicesApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0ControllerServicesApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Counters API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn counters_api(&self) -> dispatch::CountersApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::CountersApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0CountersApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::CountersApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2CountersApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::CountersApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0CountersApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [DataTransfer API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn datatransfer_api(&self) -> dispatch::DataTransferApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::DataTransferApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0DataTransferApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::DataTransferApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2DataTransferApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::DataTransferApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0DataTransferApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Flow API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn flow_api(&self) -> dispatch::FlowApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::FlowApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0FlowApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::FlowApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2FlowApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::FlowApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0FlowApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [FlowFileQueues API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn flowfilequeues_api(&self) -> dispatch::FlowFileQueuesApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::FlowFileQueuesApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0FlowFileQueuesApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::FlowFileQueuesApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2FlowFileQueuesApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::FlowFileQueuesApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0FlowFileQueuesApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Funnels API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn funnels_api(&self) -> dispatch::FunnelsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::FunnelsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0FunnelsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::FunnelsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2FunnelsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::FunnelsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0FunnelsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [InputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn inputports_api(&self) -> dispatch::InputPortsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::InputPortsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0InputPortsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::InputPortsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2InputPortsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::InputPortsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0InputPortsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Labels API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn labels_api(&self) -> dispatch::LabelsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::LabelsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0LabelsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::LabelsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2LabelsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::LabelsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0LabelsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [OutputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn outputports_api(&self) -> dispatch::OutputPortsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::OutputPortsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0OutputPortsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::OutputPortsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2OutputPortsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::OutputPortsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0OutputPortsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [ParameterContexts API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn parametercontexts_api(&self) -> dispatch::ParameterContextsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::ParameterContextsApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0ParameterContextsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::ParameterContextsApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2ParameterContextsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::ParameterContextsApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0ParameterContextsApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [ParameterProviders API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn parameterproviders_api(&self) -> dispatch::ParameterProvidersApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::ParameterProvidersApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0ParameterProvidersApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::ParameterProvidersApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2ParameterProvidersApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::ParameterProvidersApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0ParameterProvidersApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Policies API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn policies_api(&self) -> dispatch::PoliciesApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::PoliciesApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0PoliciesApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::PoliciesApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2PoliciesApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::PoliciesApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0PoliciesApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [ProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn processgroups_api(&self) -> dispatch::ProcessGroupsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ProcessGroupsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ProcessGroupsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ProcessGroupsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ProcessGroupsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ProcessGroupsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ProcessGroupsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Processors API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn processors_api(&self) -> dispatch::ProcessorsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ProcessorsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ProcessorsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ProcessorsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ProcessorsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ProcessorsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ProcessorsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Provenance API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn provenance_api(&self) -> dispatch::ProvenanceApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ProvenanceApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ProvenanceApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ProvenanceApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ProvenanceApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ProvenanceApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ProvenanceApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [ProvenanceEvents API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn provenanceevents_api(&self) -> dispatch::ProvenanceEventsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::ProvenanceEventsApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0ProvenanceEventsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::ProvenanceEventsApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2ProvenanceEventsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::ProvenanceEventsApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0ProvenanceEventsApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [RemoteProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn remoteprocessgroups_api(&self) -> dispatch::RemoteProcessGroupsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::RemoteProcessGroupsApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0RemoteProcessGroupsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::RemoteProcessGroupsApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2RemoteProcessGroupsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::RemoteProcessGroupsApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0RemoteProcessGroupsApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [ReportingTasks API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn reportingtasks_api(&self) -> dispatch::ReportingTasksApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::ReportingTasksApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0ReportingTasksApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::ReportingTasksApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2ReportingTasksApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::ReportingTasksApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0ReportingTasksApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Resources API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn resources_api(&self) -> dispatch::ResourcesApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::ResourcesApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0ResourcesApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::ResourcesApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2ResourcesApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::ResourcesApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0ResourcesApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [SiteToSite API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn sitetosite_api(&self) -> dispatch::SiteToSiteApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::SiteToSiteApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0SiteToSiteApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::SiteToSiteApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2SiteToSiteApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::SiteToSiteApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0SiteToSiteApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Snippets API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn snippets_api(&self) -> dispatch::SnippetsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::SnippetsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0SnippetsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::SnippetsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2SnippetsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::SnippetsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0SnippetsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [SystemDiagnostics API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn systemdiagnostics_api(&self) -> dispatch::SystemDiagnosticsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => dispatch::SystemDiagnosticsApiDispatch::V2_6_0(
                impls::v2_6_0::V2_6_0SystemDiagnosticsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_7_2 => dispatch::SystemDiagnosticsApiDispatch::V2_7_2(
                impls::v2_7_2::V2_7_2SystemDiagnosticsApi {
                    client: &self.client,
                },
            ),
            DetectedVersion::V2_8_0 => dispatch::SystemDiagnosticsApiDispatch::V2_8_0(
                impls::v2_8_0::V2_8_0SystemDiagnosticsApi {
                    client: &self.client,
                },
            ),
        }
    }
    /// Access the [Tenants API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn tenants_api(&self) -> dispatch::TenantsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::TenantsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0TenantsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::TenantsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2TenantsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::TenantsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0TenantsApi {
                    client: &self.client,
                })
            }
        }
    }
    /// Access the [Versions API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.
    pub fn versions_api(&self) -> dispatch::VersionsApiDispatch<'_> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                dispatch::VersionsApiDispatch::V2_6_0(impls::v2_6_0::V2_6_0VersionsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_7_2 => {
                dispatch::VersionsApiDispatch::V2_7_2(impls::v2_7_2::V2_7_2VersionsApi {
                    client: &self.client,
                })
            }
            DetectedVersion::V2_8_0 => {
                dispatch::VersionsApiDispatch::V2_8_0(impls::v2_8_0::V2_8_0VersionsApi {
                    client: &self.client,
                })
            }
        }
    }
}

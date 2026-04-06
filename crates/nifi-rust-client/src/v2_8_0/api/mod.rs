pub mod access;
pub mod authentication;
pub mod connections;
pub mod controller;
pub mod controller_services;
pub mod counters;
pub mod datatransfer;
pub mod flow;
pub mod flowfilequeues;
pub mod funnels;
pub mod inputports;
pub mod labels;
pub mod outputports;
pub mod parametercontexts;
pub mod parameterproviders;
pub mod policies;
pub mod processgroups;
pub mod processors;
pub mod provenance;
pub mod provenanceevents;
pub mod remoteprocessgroups;
pub mod reportingtasks;
pub mod resources;
pub mod sitetosite;
pub mod snippets;
pub mod systemdiagnostics;
pub mod tenants;
pub mod versions;
impl crate::NifiClient {
    /// Access the [Access API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn access_api(&self) -> access::AccessApi<'_> {
        access::AccessApi { client: self }
    }
    /// Access the [Authentication API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn authentication_api(&self) -> authentication::AuthenticationApi<'_> {
        authentication::AuthenticationApi { client: self }
    }
    /// Access the [Connections API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn connections_api(&self) -> connections::ConnectionsApi<'_> {
        connections::ConnectionsApi { client: self }
    }
    /// Access the [Controller API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn controller_api(&self) -> controller::ControllerApi<'_> {
        controller::ControllerApi { client: self }
    }
    /// Access the [Controller Services API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn controller_services_api(&self) -> controller_services::ControllerServicesApi<'_> {
        controller_services::ControllerServicesApi { client: self }
    }
    /// Access the [Counters API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn counters_api(&self) -> counters::CountersApi<'_> {
        counters::CountersApi { client: self }
    }
    /// Access the [DataTransfer API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn datatransfer_api(&self) -> datatransfer::DataTransferApi<'_> {
        datatransfer::DataTransferApi { client: self }
    }
    /// Access the [Flow API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn flow_api(&self) -> flow::FlowApi<'_> {
        flow::FlowApi { client: self }
    }
    /// Access the [FlowFileQueues API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn flowfilequeues_api(&self) -> flowfilequeues::FlowFileQueuesApi<'_> {
        flowfilequeues::FlowFileQueuesApi { client: self }
    }
    /// Access the [Funnels API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn funnels_api(&self) -> funnels::FunnelsApi<'_> {
        funnels::FunnelsApi { client: self }
    }
    /// Access the [InputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn inputports_api(&self) -> inputports::InputPortsApi<'_> {
        inputports::InputPortsApi { client: self }
    }
    /// Access the [Labels API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn labels_api(&self) -> labels::LabelsApi<'_> {
        labels::LabelsApi { client: self }
    }
    /// Access the [OutputPorts API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn outputports_api(&self) -> outputports::OutputPortsApi<'_> {
        outputports::OutputPortsApi { client: self }
    }
    /// Access the [ParameterContexts API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn parametercontexts_api(&self) -> parametercontexts::ParameterContextsApi<'_> {
        parametercontexts::ParameterContextsApi { client: self }
    }
    /// Access the [ParameterProviders API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn parameterproviders_api(&self) -> parameterproviders::ParameterProvidersApi<'_> {
        parameterproviders::ParameterProvidersApi { client: self }
    }
    /// Access the [Policies API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn policies_api(&self) -> policies::PoliciesApi<'_> {
        policies::PoliciesApi { client: self }
    }
    /// Access the [ProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn processgroups_api(&self) -> processgroups::ProcessGroupsApi<'_> {
        processgroups::ProcessGroupsApi { client: self }
    }
    /// Access the [Processors API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn processors_api(&self) -> processors::ProcessorsApi<'_> {
        processors::ProcessorsApi { client: self }
    }
    /// Access the [Provenance API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn provenance_api(&self) -> provenance::ProvenanceApi<'_> {
        provenance::ProvenanceApi { client: self }
    }
    /// Access the [ProvenanceEvents API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn provenanceevents_api(&self) -> provenanceevents::ProvenanceEventsApi<'_> {
        provenanceevents::ProvenanceEventsApi { client: self }
    }
    /// Access the [RemoteProcessGroups API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn remoteprocessgroups_api(&self) -> remoteprocessgroups::RemoteProcessGroupsApi<'_> {
        remoteprocessgroups::RemoteProcessGroupsApi { client: self }
    }
    /// Access the [ReportingTasks API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn reportingtasks_api(&self) -> reportingtasks::ReportingTasksApi<'_> {
        reportingtasks::ReportingTasksApi { client: self }
    }
    /// Access the [Resources API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn resources_api(&self) -> resources::ResourcesApi<'_> {
        resources::ResourcesApi { client: self }
    }
    /// Access the [SiteToSite API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn sitetosite_api(&self) -> sitetosite::SiteToSiteApi<'_> {
        sitetosite::SiteToSiteApi { client: self }
    }
    /// Access the [Snippets API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn snippets_api(&self) -> snippets::SnippetsApi<'_> {
        snippets::SnippetsApi { client: self }
    }
    /// Access the [SystemDiagnostics API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn systemdiagnostics_api(&self) -> systemdiagnostics::SystemDiagnosticsApi<'_> {
        systemdiagnostics::SystemDiagnosticsApi { client: self }
    }
    /// Access the [Tenants API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn tenants_api(&self) -> tenants::TenantsApi<'_> {
        tenants::TenantsApi { client: self }
    }
    /// Access the [Versions API](https://nifi.apache.org/nifi-docs/rest-api.html).
    pub fn versions_api(&self) -> versions::VersionsApi<'_> {
        versions::VersionsApi { client: self }
    }
}

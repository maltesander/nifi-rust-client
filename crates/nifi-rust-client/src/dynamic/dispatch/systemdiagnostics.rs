// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SystemDiagnosticsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the SystemDiagnostics API. Use via the [`SystemDiagnosticsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum SystemDiagnosticsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0SystemDiagnosticsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2SystemDiagnosticsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0SystemDiagnosticsApi<'a>),
}
impl<'a> SystemDiagnosticsApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl SystemDiagnosticsApi for SystemDiagnosticsApiDispatch<'_> {
    async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<types::JmxMetricsResultsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_jmx_metrics(bean_name_filter).await,
            Self::V2_7_2(api) => api.get_jmx_metrics(bean_name_filter).await,
            Self::V2_8_0(api) => api.get_jmx_metrics(bean_name_filter).await,
        }
    }
    async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::SystemDiagnosticsDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_system_diagnostics(nodewise, diagnostic_level, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_system_diagnostics(nodewise, diagnostic_level, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_system_diagnostics(nodewise, diagnostic_level, cluster_node_id)
                    .await
            }
        }
    }
}

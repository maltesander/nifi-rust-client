// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SystemDiagnosticsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0SystemDiagnosticsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl SystemDiagnosticsApi for V2_6_0SystemDiagnosticsApi<'_> {
    async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<types::JmxMetricsResultsEntity, NifiError> {
        let api = crate::v2_6_0::api::systemdiagnostics::SystemDiagnosticsApi {
            client: self.client,
        };
        Ok(api.get_jmx_metrics(bean_name_filter).await?.into())
    }
    async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::SystemDiagnosticsDto, NifiError> {
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
}

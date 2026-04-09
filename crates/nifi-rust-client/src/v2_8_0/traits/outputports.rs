// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `bulletins` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsBulletinsApi {
    /// Clears bulletins for an output port
    async fn clear_bulletins_3(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError>;
}
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsRunStatusApi {
    /// Updates run status of an output-port
    async fn update_run_status_3(
        &self,
        body: &crate::v2_8_0::types::PortRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::ProcessorEntity, NifiError>;
}
/// The OutputPorts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsApi {
    type OutputPortsBulletinsApi<'b>: OutputPortsBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::OutputPortsBulletinsApi<'b>;
    type OutputPortsRunStatusApi<'b>: OutputPortsRunStatusApi
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::OutputPortsRunStatusApi<'b>;
    /// Deletes an output port
    async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError>;
    /// Gets an output port
    async fn get_output_port(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError>;
    /// Updates an output port
    async fn update_output_port(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::PortEntity,
    ) -> Result<crate::v2_8_0::types::PortEntity, NifiError>;
}

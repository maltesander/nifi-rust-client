// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `run_status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait InputPortsRunStatusApi {
    /// Updates run status of an input-port
    async fn update_run_status_2(
        &self,
        body: &crate::v2_6_0::types::PortRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError>;
}
/// The InputPorts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait InputPortsApi {
    type InputPortsRunStatusApi<'b>: InputPortsRunStatusApi
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::InputPortsRunStatusApi<'b>;
    /// Deletes an input port
    async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::PortEntity, NifiError>;
    /// Gets an input port
    async fn get_input_port(&self, id: &str)
    -> Result<crate::v2_6_0::types::PortEntity, NifiError>;
    /// Updates an input port
    async fn update_input_port(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::PortEntity,
    ) -> Result<crate::v2_6_0::types::PortEntity, NifiError>;
}

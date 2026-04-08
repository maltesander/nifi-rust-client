// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::OutputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0OutputPortsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl OutputPortsApi for V2_6_0OutputPortsApi<'_> {
    async fn get_output_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::outputports::OutputPortsApi {
            client: self.client,
        };
        Ok(api.get_output_port(id).await?.into())
    }
    async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::outputports::OutputPortsApi {
            client: self.client,
        };
        Ok(api
            .remove_output_port(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::outputports::OutputPortsApi {
            client: self.client,
        };
        Ok(api
            .update_output_port(id, &crate::v2_6_0::types::PortEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn update_run_status_3(
        &self,
        id: &str,
        body: types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::outputports::OutputPortsRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status_3(&crate::v2_6_0::types::PortRunStatusEntity::try_from(body)?)
            .await?
            .into())
    }
}

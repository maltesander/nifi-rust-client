// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::InputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0InputPortsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl InputPortsApi for V2_6_0InputPortsApi<'_> {
    async fn get_input_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::inputports::InputPortsApi {
            client: self.client,
        };
        Ok(api.get_input_port(id).await?.into())
    }
    async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::inputports::InputPortsApi {
            client: self.client,
        };
        Ok(api
            .remove_input_port(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_6_0::api::inputports::InputPortsApi {
            client: self.client,
        };
        Ok(api
            .update_input_port(id, &crate::v2_6_0::types::PortEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn update_run_status_2(
        &self,
        id: &str,
        body: types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::inputports::InputPortsRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status_2(&crate::v2_6_0::types::PortRunStatusEntity::try_from(body)?)
            .await?
            .into())
    }
}

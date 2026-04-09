// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::InputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::InputPortsBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::InputPortsRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0InputPortsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl InputPortsApi for V2_8_0InputPortsApi<'_> {
    type InputPortsBulletinsApi<'b>
        = crate::dynamic::dispatch::InputPortsBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::InputPortsBulletinsApi<'b> {
        crate::dynamic::dispatch::InputPortsBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    type InputPortsRunStatusApi<'b>
        = crate::dynamic::dispatch::InputPortsRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::InputPortsRunStatusApi<'b> {
        crate::dynamic::dispatch::InputPortsRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn get_input_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_8_0::api::inputports::InputPortsApi {
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
        let api = crate::v2_8_0::api::inputports::InputPortsApi {
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
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_8_0::api::inputports::InputPortsApi {
            client: self.client,
        };
        Ok(api
            .update_input_port(
                id,
                &crate::v2_8_0::types::PortEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

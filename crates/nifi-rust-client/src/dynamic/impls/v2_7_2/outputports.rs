// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::OutputPortsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::OutputPortsBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::OutputPortsRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2OutputPortsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl OutputPortsApi for V2_7_2OutputPortsApi<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl OutputPortsBulletinsApi + 'b {
        crate::dynamic::dispatch::OutputPortsBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl OutputPortsRunStatusApi + 'b {
        crate::dynamic::dispatch::OutputPortsRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    async fn get_output_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_7_2::api::outputports::OutputPortsApi {
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
        let api = crate::v2_7_2::api::outputports::OutputPortsApi {
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
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        let api = crate::v2_7_2::api::outputports::OutputPortsApi {
            client: self.client,
        };
        Ok(api
            .update_output_port(
                id,
                &crate::v2_7_2::types::PortEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

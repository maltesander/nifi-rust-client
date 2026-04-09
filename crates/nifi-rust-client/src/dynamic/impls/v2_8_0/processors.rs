// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsConfigApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsDescriptorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsDiagnosticsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsStateApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ProcessorsThreadsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ProcessorsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProcessorsApi for V2_8_0ProcessorsApi<'_> {
    fn bulletins<'b>(&'b self, id: &'b str) -> impl ProcessorsBulletinsApi + 'b {
        crate::dynamic::dispatch::ProcessorsBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn config<'b>(&'b self, id: &'b str) -> impl ProcessorsConfigApi + 'b {
        crate::dynamic::dispatch::ProcessorsConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn descriptors<'b>(&'b self, id: &'b str) -> impl ProcessorsDescriptorsApi + 'b {
        crate::dynamic::dispatch::ProcessorsDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn diagnostics<'b>(&'b self, id: &'b str) -> impl ProcessorsDiagnosticsApi + 'b {
        crate::dynamic::dispatch::ProcessorsDiagnosticsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn run_status<'b>(&'b self, id: &'b str) -> impl ProcessorsRunStatusApi + 'b {
        crate::dynamic::dispatch::ProcessorsRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn state<'b>(&'b self, id: &'b str) -> impl ProcessorsStateApi + 'b {
        crate::dynamic::dispatch::ProcessorsStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn threads<'b>(&'b self, id: &'b str) -> impl ProcessorsThreadsApi + 'b {
        crate::dynamic::dispatch::ProcessorsThreadsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_8_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .delete_processor(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_8_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api.get_processor(id).await?.into())
    }
    async fn get_processor_run_status_details(
        &self,
        body: &types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        let api = crate::v2_8_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .get_processor_run_status_details(
                &crate::v2_8_0::types::RunStatusDetailsRequestEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
    async fn update_processor(
        &self,
        id: &str,
        body: &types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_8_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .update_processor(
                id,
                &crate::v2_8_0::types::ProcessorEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

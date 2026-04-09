// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ReportingTasksApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ReportingTasksBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ReportingTasksConfigApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ReportingTasksDescriptorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ReportingTasksRunStatusApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ReportingTasksStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ReportingTasksApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ReportingTasksApi for V2_7_2ReportingTasksApi<'_> {
    type ReportingTasksBulletinsApi<'b>
        = crate::dynamic::dispatch::ReportingTasksBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ReportingTasksBulletinsApi<'b> {
        crate::dynamic::dispatch::ReportingTasksBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ReportingTasksConfigApi<'b>
        = crate::dynamic::dispatch::ReportingTasksConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ReportingTasksConfigApi<'b> {
        crate::dynamic::dispatch::ReportingTasksConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ReportingTasksDescriptorsApi<'b>
        = crate::dynamic::dispatch::ReportingTasksDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ReportingTasksDescriptorsApi<'b> {
        crate::dynamic::dispatch::ReportingTasksDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ReportingTasksRunStatusApi<'b>
        = crate::dynamic::dispatch::ReportingTasksRunStatusApiDispatch<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ReportingTasksRunStatusApi<'b> {
        crate::dynamic::dispatch::ReportingTasksRunStatusApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ReportingTasksStateApi<'b>
        = crate::dynamic::dispatch::ReportingTasksStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ReportingTasksStateApi<'b> {
        crate::dynamic::dispatch::ReportingTasksStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    async fn get_reporting_task(&self, id: &str) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api.get_reporting_task(id).await?.into())
    }
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api
            .remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_reporting_task(
        &self,
        id: &str,
        body: &types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        let api = crate::v2_7_2::api::reportingtasks::ReportingTasksApi {
            client: self.client,
        };
        Ok(api
            .update_reporting_task(
                id,
                &crate::v2_7_2::types::ReportingTaskEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

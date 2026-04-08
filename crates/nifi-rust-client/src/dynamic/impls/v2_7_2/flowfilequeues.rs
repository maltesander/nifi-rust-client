// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowFileQueuesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2FlowFileQueuesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl FlowFileQueuesApi for V2_7_2FlowFileQueuesApi<'_> {
    async fn create_drop_request(&self, id: &str) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.create_drop_request().await?.into())
    }
    async fn create_flow_file_listing(
        &self,
        id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.create_flow_file_listing().await?.into())
    }
    async fn delete_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.delete_listing_request(listing_request_id).await?.into())
    }
    async fn download_flow_file_content(
        &self,
        id: &str,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
            client: self.client,
            id,
        };
        api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
            .await
    }
    async fn get_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.get_drop_request(drop_request_id).await?.into())
    }
    async fn get_flow_file(
        &self,
        id: &str,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::FlowFileDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_flow_file(flowfile_uuid, cluster_node_id)
            .await?
            .into())
    }
    async fn get_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.get_listing_request(listing_request_id).await?.into())
    }
    async fn remove_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
            client: self.client,
            id,
        };
        Ok(api.remove_drop_request(drop_request_id).await?.into())
    }
}

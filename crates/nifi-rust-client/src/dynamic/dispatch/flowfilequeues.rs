// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowFileQueuesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the FlowFileQueues API. Use via the [`FlowFileQueuesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum FlowFileQueuesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0FlowFileQueuesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2FlowFileQueuesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0FlowFileQueuesApi<'a>),
}
impl FlowFileQueuesApi for FlowFileQueuesApiDispatch<'_> {
    async fn create_drop_request(&self, id: &str) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_drop_request(id).await,
            Self::V2_7_2(api) => api.create_drop_request(id).await,
            Self::V2_8_0(api) => api.create_drop_request(id).await,
        }
    }
    async fn create_flow_file_listing(
        &self,
        id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_flow_file_listing(id).await,
            Self::V2_7_2(api) => api.create_flow_file_listing(id).await,
            Self::V2_8_0(api) => api.create_flow_file_listing(id).await,
        }
    }
    async fn delete_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_listing_request(id, listing_request_id).await,
            Self::V2_7_2(api) => api.delete_listing_request(id, listing_request_id).await,
            Self::V2_8_0(api) => api.delete_listing_request(id, listing_request_id).await,
        }
    }
    async fn download_flow_file_content(
        &self,
        id: &str,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.download_flow_file_content(id, flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.download_flow_file_content(id, flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.download_flow_file_content(id, flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
        }
    }
    async fn get_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_drop_request(id, drop_request_id).await,
            Self::V2_7_2(api) => api.get_drop_request(id, drop_request_id).await,
            Self::V2_8_0(api) => api.get_drop_request(id, drop_request_id).await,
        }
    }
    async fn get_flow_file(
        &self,
        id: &str,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::FlowFileDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_flow_file(id, flowfile_uuid, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_flow_file(id, flowfile_uuid, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_flow_file(id, flowfile_uuid, cluster_node_id).await,
        }
    }
    async fn get_listing_request(
        &self,
        id: &str,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_listing_request(id, listing_request_id).await,
            Self::V2_7_2(api) => api.get_listing_request(id, listing_request_id).await,
            Self::V2_8_0(api) => api.get_listing_request(id, listing_request_id).await,
        }
    }
    async fn remove_drop_request(
        &self,
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.remove_drop_request(id, drop_request_id).await,
            Self::V2_7_2(api) => api.remove_drop_request(id, drop_request_id).await,
            Self::V2_8_0(api) => api.remove_drop_request(id, drop_request_id).await,
        }
    }
}

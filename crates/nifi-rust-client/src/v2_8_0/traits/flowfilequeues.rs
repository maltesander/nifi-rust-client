// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `drop_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesDropRequestsApi {
    /// Creates a request to drop the contents of the queue in this connection.
    async fn create_drop_request(&self) -> Result<crate::v2_8_0::types::DropRequestDto, NifiError>;
    /// Cancels and/or removes a request to drop the contents of this connection.
    async fn remove_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_8_0::types::DropRequestDto, NifiError>;
    /// Gets the current status of a drop request for the specified connection.
    async fn get_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_8_0::types::DropRequestDto, NifiError>;
}
/// Sub-resource trait for the `flowfiles` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesFlowfilesApi {
    /// Gets a FlowFile from a Connection.
    async fn get_flow_file(
        &self,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::FlowFileDto, NifiError>;
    /// Gets the content for a FlowFile in a Connection.
    async fn download_flow_file_content(
        &self,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError>;
}
/// Sub-resource trait for the `listing_requests` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesListingRequestsApi {
    /// Lists the contents of the queue in this connection.
    async fn create_flow_file_listing(
        &self,
    ) -> Result<crate::v2_8_0::types::ListingRequestDto, NifiError>;
    /// Cancels and/or removes a request to list the contents of this connection.
    async fn delete_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<crate::v2_8_0::types::ListingRequestDto, NifiError>;
    /// Gets the current status of a listing request for the specified connection.
    async fn get_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<crate::v2_8_0::types::ListingRequestDto, NifiError>;
}
/// The FlowFileQueues API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesApi {
    type FlowFileQueuesDropRequestsApi<'b>: FlowFileQueuesDropRequestsApi
    where
        Self: 'b;
    fn drop_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesDropRequestsApi<'b>;
    type FlowFileQueuesFlowfilesApi<'b>: FlowFileQueuesFlowfilesApi
    where
        Self: 'b;
    fn flowfiles<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesFlowfilesApi<'b>;
    type FlowFileQueuesListingRequestsApi<'b>: FlowFileQueuesListingRequestsApi
    where
        Self: 'b;
    fn listing_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesListingRequestsApi<'b>;
}

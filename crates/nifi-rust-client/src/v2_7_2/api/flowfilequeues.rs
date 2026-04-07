// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct FlowFileQueuesApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowFileQueuesApi<'a> {
    /// Scope operations to the `drop_requests` sub-resource of a specific process group.
    ///
    /// - `id`: The connection id.
    pub fn drop_requests<'b>(&'b self, id: &'b str) -> FlowFileQueuesDropRequestsApi<'b> {
        FlowFileQueuesDropRequestsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `flowfiles` sub-resource of a specific process group.
    ///
    /// - `id`: The connection id.
    pub fn flowfiles<'b>(&'b self, id: &'b str) -> FlowFileQueuesFlowfilesApi<'b> {
        FlowFileQueuesFlowfilesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `listing_requests` sub-resource of a specific process group.
    ///
    /// - `id`: The connection id.
    pub fn listing_requests<'b>(&'b self, id: &'b str) -> FlowFileQueuesListingRequestsApi<'b> {
        FlowFileQueuesListingRequestsApi {
            client: self.client,
            id,
        }
    }
}
pub struct FlowFileQueuesDropRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowFileQueuesDropRequestsApi<'a> {
    /// Creates a request to drop the contents of the queue in this connection.
    ///
    /// Calls `POST /nifi-api/flowfile-queues/{id}/drop-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write Source Data - /data/{component-type}/{uuid}`.
    pub async fn create_drop_request(
        &self,
    ) -> Result<crate::v2_7_2::types::DropRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::DropRequestEntity = self
            .client
            .post_no_body(&format!("/flowfile-queues/{id}/drop-requests"))
            .await?;
        Ok(e.drop_request.unwrap_or_default())
    }
    /// Cancels and/or removes a request to drop the contents of this connection.
    ///
    /// Calls `DELETE /nifi-api/flowfile-queues/{id}/drop-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write Source Data - /data/{component-type}/{uuid}`.
    pub async fn remove_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_7_2::types::DropRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::DropRequestEntity = self
            .client
            .delete_returning(&format!(
                "/flowfile-queues/{id}/drop-requests/{drop_request_id}"
            ))
            .await?;
        Ok(e.drop_request.unwrap_or_default())
    }
    /// Gets the current status of a drop request for the specified connection.
    ///
    /// Calls `GET /nifi-api/flowfile-queues/{id}/drop-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write Source Data - /data/{component-type}/{uuid}`.
    pub async fn get_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::v2_7_2::types::DropRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::DropRequestEntity = self
            .client
            .get(&format!(
                "/flowfile-queues/{id}/drop-requests/{drop_request_id}"
            ))
            .await?;
        Ok(e.drop_request.unwrap_or_default())
    }
}
pub struct FlowFileQueuesFlowfilesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowFileQueuesFlowfilesApi<'a> {
    /// Gets a FlowFile from a Connection.
    ///
    /// Calls `GET /nifi-api/flowfile-queues/{id}/flowfiles/{flowfile-uuid}`.
    ///
    /// # Parameters
    /// - `flowfile_uuid`: The flowfile uuid.
    /// - `cluster_node_id`: The id of the node where the content exists if clustered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    pub async fn get_flow_file(
        &self,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_7_2::types::FlowFileDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_7_2::types::FlowFileEntity = self
            .client
            .get_with_query(
                &format!("/flowfile-queues/{id}/flowfiles/{flowfile_uuid}"),
                &query,
            )
            .await?;
        Ok(e.flow_file.unwrap_or_default())
    }
    /// Gets the content for a FlowFile in a Connection.
    ///
    /// Calls `GET /nifi-api/flowfile-queues/{id}/flowfiles/{flowfile-uuid}/content`.
    ///
    /// # Parameters
    /// - `flowfile_uuid`: The flowfile uuid.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `cluster_node_id`: The id of the node where the content exists if clustered.
    ///
    /// # Errors
    /// - `206`: Partial Content with range of bytes requested
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `416`: Requested Range Not Satisfiable based on bytes requested
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    pub async fn download_flow_file_content(
        &self,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_void_with_query(
                &format!("/flowfile-queues/{id}/flowfiles/{flowfile_uuid}/content"),
                &query,
            )
            .await
    }
}
pub struct FlowFileQueuesListingRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FlowFileQueuesListingRequestsApi<'a> {
    /// Lists the contents of the queue in this connection.
    ///
    /// Calls `POST /nifi-api/flowfile-queues/{id}/listing-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    pub async fn create_flow_file_listing(
        &self,
    ) -> Result<crate::v2_7_2::types::ListingRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ListingRequestEntity = self
            .client
            .post_no_body(&format!("/flowfile-queues/{id}/listing-requests"))
            .await?;
        Ok(e.listing_request.unwrap_or_default())
    }
    /// Cancels and/or removes a request to list the contents of this connection.
    ///
    /// Calls `DELETE /nifi-api/flowfile-queues/{id}/listing-requests/{listing-request-id}`.
    ///
    /// # Parameters
    /// - `listing_request_id`: The listing request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    pub async fn delete_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<crate::v2_7_2::types::ListingRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ListingRequestEntity = self
            .client
            .delete_returning(&format!(
                "/flowfile-queues/{id}/listing-requests/{listing_request_id}"
            ))
            .await?;
        Ok(e.listing_request.unwrap_or_default())
    }
    /// Gets the current status of a listing request for the specified connection.
    ///
    /// Calls `GET /nifi-api/flowfile-queues/{id}/listing-requests/{listing-request-id}`.
    ///
    /// # Parameters
    /// - `listing_request_id`: The listing request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    pub async fn get_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<crate::v2_7_2::types::ListingRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ListingRequestEntity = self
            .client
            .get(&format!(
                "/flowfile-queues/{id}/listing-requests/{listing_request_id}"
            ))
            .await?;
        Ok(e.listing_request.unwrap_or_default())
    }
}

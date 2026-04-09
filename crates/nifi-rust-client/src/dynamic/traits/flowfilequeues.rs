// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for FlowFileQueuesDropRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesDropRequestsApi {
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
    async fn create_drop_request(&self) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_drop_request".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn get_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_drop_request".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn remove_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_drop_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowFileQueuesFlowfilesApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesFlowfilesApi {
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
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `416`: Requested Range Not Satisfiable based on bytes requested
    ///
    /// # Permissions
    /// Requires `Read Source Data - /data/{component-type}/{uuid}`.
    async fn download_flow_file_content(
        &self,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "download_flow_file_content".to_string(),
            version: "unknown".to_string(),
        })
    }
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
    async fn get_flow_file(
        &self,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::FlowFileDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_file".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowFileQueuesListingRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesListingRequestsApi {
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
    async fn create_flow_file_listing(&self) -> Result<types::ListingRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_flow_file_listing".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn delete_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_listing_request".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn get_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_listing_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The FlowFileQueues API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowFileQueuesApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    type FlowFileQueuesDropRequestsApi<'b>: FlowFileQueuesDropRequestsApi
    where
        Self: 'b;
    fn drop_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesDropRequestsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    type FlowFileQueuesFlowfilesApi<'b>: FlowFileQueuesFlowfilesApi
    where
        Self: 'b;
    fn flowfiles<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesFlowfilesApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    type FlowFileQueuesListingRequestsApi<'b>: FlowFileQueuesListingRequestsApi
    where
        Self: 'b;
    fn listing_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesListingRequestsApi<'b>;
}

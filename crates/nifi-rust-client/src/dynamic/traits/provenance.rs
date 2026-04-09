// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Provenance API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceApi {
    /// Deletes a lineage query
    ///
    /// Calls `DELETE /nifi-api/provenance/lineage/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the lineage query.
    /// - `cluster_node_id`: The id of the node where this query exists if clustered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_lineage".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a provenance query
    ///
    /// Calls `DELETE /nifi-api/provenance/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the provenance query.
    /// - `cluster_node_id`: The id of the node where this query exists if clustered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_provenance".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a lineage query
    ///
    /// Calls `GET /nifi-api/provenance/lineage/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the lineage query.
    /// - `cluster_node_id`: The id of the node where this query exists if clustered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    /// Requires `Read - /data/{component-type}/{uuid}`.
    async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_lineage".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a provenance query
    ///
    /// Calls `GET /nifi-api/provenance/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the provenance query.
    /// - `cluster_node_id`: The id of the node where this query exists if clustered.
    /// - `summarize`: Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default.
    /// - `incremental_results`: Whether or not to summarize provenance events returned. This property is false by default.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    /// Requires `Read - /data/{component-type}/{uuid}`.
    async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_provenance".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the searchable attributes for provenance events
    ///
    /// Calls `GET /nifi-api/provenance/search-options`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    async fn get_search_options(&self) -> Result<types::ProvenanceOptionsDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_search_options".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Submits a lineage query
    ///
    /// Lineage queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the lineage request should be deleted by the client who originally submitted it.
    ///
    /// Calls `POST /nifi-api/provenance/lineage`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    /// Requires `Read - /data/{component-type}/{uuid}`.
    async fn submit_lineage_request(
        &self,
        body: &types::LineageEntity,
    ) -> Result<types::LineageDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_lineage_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Submits a provenance query
    ///
    /// Provenance queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the provenance request should be deleted by the client who originally submitted it.
    ///
    /// Calls `POST /nifi-api/provenance`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /provenance`.
    /// Requires `Read - /data/{component-type}/{uuid}`.
    async fn submit_provenance_request(
        &self,
        body: &types::ProvenanceEntity,
    ) -> Result<types::ProvenanceDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_provenance_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}

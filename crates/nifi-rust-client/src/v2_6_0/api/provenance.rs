// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ProvenanceApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProvenanceApi<'a> {
    /// Submits a provenance query
    ///
    /// Provenance queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the provenance request should be deleted by the client who originally submitted it.
    ///
    /// Calls `POST /nifi-api/provenance`.
    ///
    /// # Parameters
    /// - `body`: The provenance query details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /provenance`
    /// - `Read - /data/{component-type}/{uuid}`
    pub async fn submit_provenance_request(
        &self,
        body: &crate::v2_6_0::types::ProvenanceEntity,
    ) -> Result<crate::v2_6_0::types::ProvenanceDto, NifiError> {
        let e: crate::v2_6_0::types::ProvenanceEntity =
            self.client.post("/provenance", body).await?;
        Ok(e.provenance)
    }
    /// Submits a lineage query
    ///
    /// Lineage queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the lineage request should be deleted by the client who originally submitted it.
    ///
    /// Calls `POST /nifi-api/provenance/lineage`.
    ///
    /// # Parameters
    /// - `body`: The lineage query details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /provenance`
    /// - `Read - /data/{component-type}/{uuid}`
    pub async fn submit_lineage_request(
        &self,
        body: &crate::v2_6_0::types::LineageEntity,
    ) -> Result<crate::v2_6_0::types::LineageDto, NifiError> {
        let e: crate::v2_6_0::types::LineageEntity =
            self.client.post("/provenance/lineage", body).await?;
        Ok(e.lineage)
    }
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
    pub async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::LineageDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_6_0::types::LineageEntity = self
            .client
            .delete_returning_with_query(&format!("/provenance/lineage/{id}"), &query)
            .await?;
        Ok(e.lineage)
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
    /// - `Read - /provenance`
    /// - `Read - /data/{component-type}/{uuid}`
    pub async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::LineageDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_6_0::types::LineageEntity = self
            .client
            .get_with_query(&format!("/provenance/lineage/{id}"), &query)
            .await?;
        Ok(e.lineage)
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
    pub async fn get_search_options(
        &self,
    ) -> Result<crate::v2_6_0::types::ProvenanceOptionsDto, NifiError> {
        let e: crate::v2_6_0::types::ProvenanceOptionsEntity =
            self.client.get("/provenance/search-options").await?;
        Ok(e.provenance_options)
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
    pub async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProvenanceDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_6_0::types::ProvenanceEntity = self
            .client
            .delete_returning_with_query(&format!("/provenance/{id}"), &query)
            .await?;
        Ok(e.provenance)
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
    /// - `Read - /provenance`
    /// - `Read - /data/{component-type}/{uuid}`
    pub async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProvenanceDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        if let Some(v) = summarize {
            query.push(("summarize", v.to_string()));
        }
        if let Some(v) = incremental_results {
            query.push(("incrementalResults", v.to_string()));
        }
        let e: crate::v2_6_0::types::ProvenanceEntity = self
            .client
            .get_with_query(&format!("/provenance/{id}"), &query)
            .await?;
        Ok(e.provenance)
    }
}

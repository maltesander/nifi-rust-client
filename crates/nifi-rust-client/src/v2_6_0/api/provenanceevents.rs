// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ProvenanceEventsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProvenanceEventsApi<'a> {
    /// Replays content from a provenance event
    ///
    /// Calls `POST /nifi-api/provenance-events/latest/replays`.
    ///
    /// # Parameters
    /// - `body`: The replay request.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`
    /// - `Read Component Data - /data/{component-type}/{uuid}`
    /// - `Write Component Data - /data/{component-type}/{uuid}`
    pub async fn submit_replay_latest_event(
        &self,
        body: &crate::v2_6_0::types::ReplayLastEventRequestEntity,
    ) -> Result<crate::v2_6_0::types::ReplayLastEventResponseEntity, NifiError> {
        self.client
            .post("/provenance-events/latest/replays", body)
            .await
    }
    /// Retrieves the latest cached Provenance Events for the specified component
    ///
    /// Calls `GET /nifi-api/provenance-events/latest/{componentId}`.
    ///
    /// # Parameters
    /// - `component_id`: The ID of the component to retrieve the latest Provenance Events for.
    /// - `limit`: The number of events to limit the response to. Defaults to 10.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`
    /// - `Read Component Data - /data/{component-type}/{uuid}`
    pub async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<crate::v2_6_0::types::LatestProvenanceEventsDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = limit {
            query.push(("limit", v.to_string()));
        }
        let e: crate::v2_6_0::types::LatestProvenanceEventsEntity = self
            .client
            .get_with_query(&format!("/provenance-events/latest/{component_id}"), &query)
            .await?;
        Ok(e.latest_provenance_events.unwrap_or_default())
    }
    /// Replays content from a provenance event
    ///
    /// Calls `POST /nifi-api/provenance-events/replays`.
    ///
    /// # Parameters
    /// - `body`: The replay request.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`
    /// - `Read Component Data - /data/{component-type}/{uuid}`
    /// - `Write Component Data - /data/{component-type}/{uuid}`
    pub async fn submit_replay(
        &self,
        body: &crate::v2_6_0::types::SubmitReplayRequestEntity,
    ) -> Result<crate::v2_6_0::types::ProvenanceEventDto, NifiError> {
        let e: crate::v2_6_0::types::ProvenanceEventEntity =
            self.client.post("/provenance-events/replays", body).await?;
        Ok(e.provenance_event.unwrap_or_default())
    }
    /// Gets a provenance event
    ///
    /// Calls `GET /nifi-api/provenance-events/{id}`.
    ///
    /// # Parameters
    /// - `id`: The provenance event id.
    /// - `cluster_node_id`: The id of the node where this event exists if clustered.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`.
    pub async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProvenanceEventDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_6_0::types::ProvenanceEventEntity = self
            .client
            .get_with_query(&format!("/provenance-events/{id}"), &query)
            .await?;
        Ok(e.provenance_event.unwrap_or_default())
    }
    /// Scope operations to the `content` sub-resource of a specific process group.
    ///
    /// - `id`: The provenance event id.
    pub fn content<'b>(&'b self, id: &'b str) -> ProvenanceEventsContentApi<'b> {
        ProvenanceEventsContentApi {
            client: self.client,
            id,
        }
    }
}
pub struct ProvenanceEventsContentApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProvenanceEventsContentApi<'a> {
    /// Gets the input content for a provenance event
    ///
    /// Calls `GET /nifi-api/provenance-events/{id}/content/input`.
    ///
    /// # Parameters
    /// - `cluster_node_id`: The id of the node where the content exists if clustered.
    ///
    /// # Returns
    /// - `206`: Partial Content with range of bytes requested
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
    /// - `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`
    /// - `Read Component Data - /data/{component-type}/{uuid}`
    pub async fn get_input_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_void_with_query(&format!("/provenance-events/{id}/content/input"), &query)
            .await
    }
    /// Gets the output content for a provenance event
    ///
    /// Calls `GET /nifi-api/provenance-events/{id}/content/output`.
    ///
    /// # Parameters
    /// - `cluster_node_id`: The id of the node where the content exists if clustered.
    ///
    /// # Returns
    /// - `206`: Partial Content with range of bytes requested
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
    /// - `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`
    /// - `Read Component Data - /data/{component-type}/{uuid}`
    pub async fn get_output_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        self.client
            .get_void_with_query(&format!("/provenance-events/{id}/content/output"), &query)
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProvenanceEventsApi for ProvenanceEventsApi<'_> {
    fn content<'b>(
        &'b self,
        id: &'b str,
    ) -> impl crate::v2_6_0::traits::ProvenanceEventsContentApi + 'b {
        ProvenanceEventsContentApi {
            client: self.client,
            id,
        }
    }
    async fn submit_replay_latest_event(
        &self,
        body: &crate::v2_6_0::types::ReplayLastEventRequestEntity,
    ) -> Result<crate::v2_6_0::types::ReplayLastEventResponseEntity, NifiError> {
        self.submit_replay_latest_event(body).await
    }
    async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<crate::v2_6_0::types::LatestProvenanceEventsDto, NifiError> {
        self.get_latest_provenance_events(component_id, limit).await
    }
    async fn submit_replay(
        &self,
        body: &crate::v2_6_0::types::SubmitReplayRequestEntity,
    ) -> Result<crate::v2_6_0::types::ProvenanceEventDto, NifiError> {
        self.submit_replay(body).await
    }
    async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProvenanceEventDto, NifiError> {
        self.get_provenance_event(id, cluster_node_id).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProvenanceEventsContentApi for ProvenanceEventsContentApi<'_> {
    async fn get_input_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        self.get_input_content(cluster_node_id).await
    }
    async fn get_output_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        self.get_output_content(cluster_node_id).await
    }
}

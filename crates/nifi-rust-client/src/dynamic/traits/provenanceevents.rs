// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for ProvenanceEventsContentApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceEventsContentApi {
    /// Gets the input content for a provenance event
    ///
    /// Calls `GET /nifi-api/provenance-events/{id}/content/input`.
    ///
    /// # Parameters
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
    /// Requires `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`.
    /// Requires `Read Component Data - /data/{component-type}/{uuid}`.
    async fn get_input_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_input_content".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the output content for a provenance event
    ///
    /// Calls `GET /nifi-api/provenance-events/{id}/content/output`.
    ///
    /// # Parameters
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
    /// Requires `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`.
    /// Requires `Read Component Data - /data/{component-type}/{uuid}`.
    async fn get_output_content(&self, cluster_node_id: Option<&str>) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_output_content".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The ProvenanceEvents API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceEventsApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The provenance event id.
    fn content<'b>(&'b self, id: &'b str) -> impl ProvenanceEventsContentApi + 'b;
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
    /// Requires `Read Component Provenance Data - /provenance-data/{component-type}/{uuid}`.
    /// Requires `Read Component Data - /data/{component-type}/{uuid}`.
    async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<types::LatestProvenanceEventsDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_latest_provenance_events".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_provenance_event".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Replays content from a provenance event
    ///
    /// Calls `POST /nifi-api/provenance-events/replays`.
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
    /// Requires `Read Component Data - /data/{component-type}/{uuid}`.
    /// Requires `Write Component Data - /data/{component-type}/{uuid}`.
    async fn submit_replay(
        &self,
        body: &types::SubmitReplayRequestEntity,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_replay".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Replays content from a provenance event
    ///
    /// Calls `POST /nifi-api/provenance-events/latest/replays`.
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
    /// Requires `Read Component Data - /data/{component-type}/{uuid}`.
    /// Requires `Write Component Data - /data/{component-type}/{uuid}`.
    async fn submit_replay_latest_event(
        &self,
        body: &types::ReplayLastEventRequestEntity,
    ) -> Result<types::ReplayLastEventResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_replay_latest_event".to_string(),
            version: "unknown".to_string(),
        })
    }
}

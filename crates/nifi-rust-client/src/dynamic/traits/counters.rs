// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Counters API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait CountersApi {
    /// Gets the current counters for this NiFi
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/counters`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /counters`.
    async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::CountersDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_counters".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates all counters. This will reset all counter values to 0
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/counters`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /counters`.
    async fn update_all_counters(&self) -> Result<types::CountersDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_all_counters".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates the specified counter. This will reset the counter value to 0
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/counters/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the counter.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /counters`.
    async fn update_counter(&self, id: &str) -> Result<types::CounterDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_counter".to_string(),
            version: "unknown".to_string(),
        })
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Access API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait AccessApi {
    /// Performs a logout for other providers that have been issued a JWT.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/access/logout`.
    ///
    /// # Errors
    /// - `401`: Authentication token provided was empty or not in the correct JWT format.
    /// - `500`: Client failed to log out.
    async fn log_out(&self) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "log_out".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/access/logout/complete`.
    ///
    /// # Errors
    /// - `401`: Authentication token provided was empty or not in the correct JWT format.
    /// - `500`: Client failed to log out.
    async fn log_out_complete(&self) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "log_out_complete".to_string(),
            version: "unknown".to_string(),
        })
    }
}

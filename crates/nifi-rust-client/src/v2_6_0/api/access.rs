// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct AccessApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> AccessApi<'a> {
    /// Performs a logout for other providers that have been issued a JWT.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/access/logout`.
    ///
    /// # Returns
    /// - `200`: User was logged out successfully.
    ///
    /// # Errors
    /// - `401`: Authentication token provided was empty or not in the correct JWT format.
    /// - `500`: Client failed to log out.
    pub async fn log_out(&self) -> Result<(), NifiError> {
        self.client.delete("/access/logout").await
    }
    /// Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/access/logout/complete`.
    ///
    /// # Errors
    /// - `302`: User was logged out successfully.
    /// - `401`: Authentication token provided was empty or not in the correct JWT format.
    /// - `500`: Client failed to log out.
    pub async fn log_out_complete(&self) -> Result<(), NifiError> {
        self.client.get_void("/access/logout/complete").await
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Access API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait AccessApi {
    /// Performs a logout for other providers that have been issued a JWT.
    async fn log_out(&self) -> Result<(), NifiError>;
    /// Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.
    async fn log_out_complete(&self) -> Result<(), NifiError>;
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The SiteToSite API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SiteToSiteApi {
    /// Returns the available Peers and its status of this NiFi
    ///
    /// Calls `GET /nifi-api/site-to-site/peers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /site-to-site`.
    async fn get_peers(&self) -> Result<types::PeersEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_peers".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the details about this NiFi necessary to communicate via site to site
    ///
    /// Calls `GET /nifi-api/site-to-site`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /site-to-site`.
    async fn get_site_to_site_details(&self) -> Result<types::ControllerDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_site_to_site_details".to_string(),
            version: "unknown".to_string(),
        })
    }
}

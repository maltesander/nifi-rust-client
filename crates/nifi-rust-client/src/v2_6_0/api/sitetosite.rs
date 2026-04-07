// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct SiteToSiteApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> SiteToSiteApi<'a> {
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
    pub async fn get_site_to_site_details(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerDto, NifiError> {
        let e: crate::v2_6_0::types::ControllerEntity = self.client.get("/site-to-site").await?;
        Ok(e.controller.unwrap_or_default())
    }
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
    pub async fn get_peers(&self) -> Result<crate::v2_6_0::types::PeersEntity, NifiError> {
        self.client.get("/site-to-site/peers").await
    }
}

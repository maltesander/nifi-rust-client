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
    pub async fn get_site_to_site_details(&self) -> Result<crate::types::ControllerDto, NifiError> {
        let e: crate::types::ControllerEntity = self.client.get("/site-to-site").await?;
        Ok(e.controller)
    }
    /// Returns the available Peers and its status of this NiFi
    ///
    /// Calls `GET /nifi-api/site-to-site/peers`.
    pub async fn get_peers(&self) -> Result<crate::types::PeersEntity, NifiError> {
        self.client.get("/site-to-site/peers").await
    }
}

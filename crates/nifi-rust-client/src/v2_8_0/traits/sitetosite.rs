// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The SiteToSite API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SiteToSiteApi {
    /// Returns the details about this NiFi necessary to communicate via site to site
    async fn get_site_to_site_details(
        &self,
    ) -> Result<crate::v2_8_0::types::ControllerDto, NifiError>;
    /// Returns the available Peers and its status of this NiFi
    async fn get_peers(&self) -> Result<crate::v2_8_0::types::PeersEntity, NifiError>;
}

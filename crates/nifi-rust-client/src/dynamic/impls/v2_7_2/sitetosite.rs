// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SiteToSiteApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2SiteToSiteApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl SiteToSiteApi for V2_7_2SiteToSiteApi<'_> {
    async fn get_peers(&self) -> Result<types::PeersEntity, NifiError> {
        let api = crate::v2_7_2::api::sitetosite::SiteToSiteApi {
            client: self.client,
        };
        Ok(api.get_peers().await?.into())
    }
    async fn get_site_to_site_details(&self) -> Result<types::ControllerDto, NifiError> {
        let api = crate::v2_7_2::api::sitetosite::SiteToSiteApi {
            client: self.client,
        };
        Ok(api.get_site_to_site_details().await?.into())
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::SiteToSiteApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the SiteToSite API. Use via the [`SiteToSiteApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum SiteToSiteApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0SiteToSiteApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2SiteToSiteApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0SiteToSiteApi<'a>),
}
impl<'a> SiteToSiteApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl SiteToSiteApi for SiteToSiteApiDispatch<'_> {
    async fn get_peers(&self) -> Result<types::PeersEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_peers().await,
            Self::V2_7_2(api) => api.get_peers().await,
            Self::V2_8_0(api) => api.get_peers().await,
        }
    }
    async fn get_site_to_site_details(&self) -> Result<types::ControllerDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_site_to_site_details().await,
            Self::V2_7_2(api) => api.get_site_to_site_details().await,
            Self::V2_8_0(api) => api.get_site_to_site_details().await,
        }
    }
}

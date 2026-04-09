// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ResourcesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Resources API. Use via the [`ResourcesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ResourcesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ResourcesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ResourcesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ResourcesApi<'a>),
}
impl<'a> ResourcesApiDispatch<'a> {
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
impl ResourcesApi for ResourcesApiDispatch<'_> {
    async fn get_resources(&self) -> Result<types::ResourcesEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_resources().await,
            Self::V2_7_2(api) => api.get_resources().await,
            Self::V2_8_0(api) => api.get_resources().await,
        }
    }
}

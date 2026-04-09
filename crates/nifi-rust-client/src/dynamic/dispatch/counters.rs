// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::CountersApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Counters API. Use via the [`CountersApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum CountersApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0CountersApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2CountersApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0CountersApi<'a>),
}
impl<'a> CountersApiDispatch<'a> {
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
impl CountersApi for CountersApiDispatch<'_> {
    async fn get_counters(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::CountersDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_counters(nodewise, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_counters(nodewise, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_counters(nodewise, cluster_node_id).await,
        }
    }
    async fn update_all_counters(&self) -> Result<types::CountersDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_all_counters().await,
            Self::V2_7_2(api) => api.update_all_counters().await,
            Self::V2_8_0(api) => api.update_all_counters().await,
        }
    }
    async fn update_counter(&self, id: &str) -> Result<types::CounterDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_counter(id).await,
            Self::V2_7_2(api) => api.update_counter(id).await,
            Self::V2_8_0(api) => api.update_counter(id).await,
        }
    }
}

// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::AccessApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Access API. Use via the [`AccessApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum AccessApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0AccessApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2AccessApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0AccessApi<'a>),
}
impl AccessApi for AccessApiDispatch<'_> {
    async fn log_out(&self) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.log_out().await,
            Self::V2_7_2(api) => api.log_out().await,
            Self::V2_8_0(api) => api.log_out().await,
        }
    }
    async fn log_out_complete(&self) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.log_out_complete().await,
            Self::V2_7_2(api) => api.log_out_complete().await,
            Self::V2_8_0(api) => api.log_out_complete().await,
        }
    }
}

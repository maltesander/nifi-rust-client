// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::AuthenticationApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Authentication API. Use via the [`AuthenticationApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum AuthenticationApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0AuthenticationApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2AuthenticationApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0AuthenticationApi<'a>),
}
impl AuthenticationApi for AuthenticationApiDispatch<'_> {
    async fn get_authentication_configuration(
        &self,
    ) -> Result<types::AuthenticationConfigurationDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_authentication_configuration().await,
            Self::V2_7_2(api) => api.get_authentication_configuration().await,
            Self::V2_8_0(api) => api.get_authentication_configuration().await,
        }
    }
}

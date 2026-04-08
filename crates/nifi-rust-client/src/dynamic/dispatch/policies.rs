// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::PoliciesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Policies API. Use via the [`PoliciesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum PoliciesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0PoliciesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2PoliciesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0PoliciesApi<'a>),
}
impl PoliciesApi for PoliciesApiDispatch<'_> {
    async fn create_access_policy(
        &self,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_access_policy(body).await,
            Self::V2_7_2(api) => api.create_access_policy(body).await,
            Self::V2_8_0(api) => api.create_access_policy(body).await,
        }
    }
    async fn get_access_policy(&self, id: &str) -> Result<types::AccessPolicyEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_access_policy(id).await,
            Self::V2_7_2(api) => api.get_access_policy(id).await,
            Self::V2_8_0(api) => api.get_access_policy(id).await,
        }
    }
    async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_access_policy_for_resource(action, resource).await,
            Self::V2_7_2(api) => api.get_access_policy_for_resource(action, resource).await,
            Self::V2_8_0(api) => api.get_access_policy_for_resource(action, resource).await,
        }
    }
    async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn update_access_policy(
        &self,
        id: &str,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_access_policy(id, body).await,
            Self::V2_7_2(api) => api.update_access_policy(id, body).await,
            Self::V2_8_0(api) => api.update_access_policy(id, body).await,
        }
    }
}

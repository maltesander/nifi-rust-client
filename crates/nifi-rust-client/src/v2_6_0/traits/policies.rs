// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Policies API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait PoliciesApi {
    /// Creates an access policy
    async fn create_access_policy(
        &self,
        body: &crate::v2_6_0::types::AccessPolicyEntity,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError>;
    /// Gets an access policy for the specified action and resource
    async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError>;
    /// Deletes an access policy
    async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError>;
    /// Gets an access policy
    async fn get_access_policy(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError>;
    /// Updates a access policy
    async fn update_access_policy(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::AccessPolicyEntity,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError>;
}

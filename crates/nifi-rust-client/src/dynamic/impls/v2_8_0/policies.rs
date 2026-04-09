// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::PoliciesApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0PoliciesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl PoliciesApi for V2_8_0PoliciesApi<'_> {
    async fn create_access_policy(
        &self,
        body: &types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        let api = crate::v2_8_0::api::policies::PoliciesApi {
            client: self.client,
        };
        Ok(api
            .create_access_policy(&crate::v2_8_0::types::AccessPolicyEntity::try_from(
                body.clone(),
            )?)
            .await?
            .into())
    }
    async fn get_access_policy(&self, id: &str) -> Result<types::AccessPolicyEntity, NifiError> {
        let api = crate::v2_8_0::api::policies::PoliciesApi {
            client: self.client,
        };
        Ok(api.get_access_policy(id).await?.into())
    }
    async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        let api = crate::v2_8_0::api::policies::PoliciesApi {
            client: self.client,
        };
        Ok(api
            .get_access_policy_for_resource(action, resource)
            .await?
            .into())
    }
    async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        let api = crate::v2_8_0::api::policies::PoliciesApi {
            client: self.client,
        };
        Ok(api
            .remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_access_policy(
        &self,
        id: &str,
        body: &types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        let api = crate::v2_8_0::api::policies::PoliciesApi {
            client: self.client,
        };
        Ok(api
            .update_access_policy(
                id,
                &crate::v2_8_0::types::AccessPolicyEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

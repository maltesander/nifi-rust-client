// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ParameterProvidersApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersApplyParametersRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersBulletinsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersConfigApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersDescriptorsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersParametersApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersReferencesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::ParameterProvidersStateApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2ParameterProvidersApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ParameterProvidersApi for V2_7_2ParameterProvidersApi<'_> {
    type ParameterProvidersApplyParametersRequestsApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersApplyParametersRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn apply_parameters_requests<'b>(
        &'b self,
        provider_id: &'b str,
    ) -> Self::ParameterProvidersApplyParametersRequestsApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersApplyParametersRequestsApiDispatch {
            client: self.client,
            provider_id: provider_id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersBulletinsApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersBulletinsApiDispatch<'b>
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersBulletinsApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersBulletinsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersConfigApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersConfigApiDispatch<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersConfigApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersConfigApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersDescriptorsApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersDescriptorsApiDispatch<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersDescriptorsApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersDescriptorsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersParametersApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersParametersApiDispatch<'b>
    where
        Self: 'b;
    fn parameters<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersParametersApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersParametersApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersReferencesApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersReferencesApiDispatch<'b>
    where
        Self: 'b;
    fn references<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersReferencesApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersReferencesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type ParameterProvidersStateApi<'b>
        = crate::dynamic::dispatch::ParameterProvidersStateApiDispatch<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersStateApi<'b> {
        crate::dynamic::dispatch::ParameterProvidersStateApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api.get_parameter_provider(id).await?.into())
    }
    async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api
            .remove_parameter_provider(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        let api = crate::v2_7_2::api::parameterproviders::ParameterProvidersApi {
            client: self.client,
        };
        Ok(api
            .update_parameter_provider(
                id,
                &crate::v2_7_2::types::ParameterProviderEntity::try_from(body.clone())?,
            )
            .await?
            .into())
    }
}

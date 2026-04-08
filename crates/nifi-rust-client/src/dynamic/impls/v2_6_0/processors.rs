// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProcessorsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0ProcessorsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProcessorsApi for V2_6_0ProcessorsApi<'_> {
    async fn analyze_configuration_2(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .analyze_configuration_2(
                &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn clear_state_3(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
            client: self.client,
            id,
        };
        Ok(api
            .clear_state_3(&crate::v2_6_0::types::ComponentStateEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .delete_processor(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn delete_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
            client: self.client,
            id,
        };
        Ok(api.delete_verification_request_2(request_id).await?.into())
    }
    async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api.get_processor(id).await?.into())
    }
    async fn get_processor_diagnostics(
        &self,
        id: &str,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsDiagnosticsApi {
            client: self.client,
            id,
        };
        Ok(api.get_processor_diagnostics().await?.into())
    }
    async fn get_processor_run_status_details(
        &self,
        body: types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .get_processor_run_status_details(
                &crate::v2_6_0::types::RunStatusDetailsRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn get_property_descriptor_3(
        &self,
        id: &str,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsDescriptorsApi {
            client: self.client,
            id,
        };
        Ok(api
            .get_property_descriptor_3(client_id, property_name, sensitive)
            .await?
            .into())
    }
    async fn get_state_2(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsStateApi {
            client: self.client,
            id,
        };
        Ok(api.get_state_2().await?.into())
    }
    async fn get_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
            client: self.client,
            id,
        };
        Ok(api.get_verification_request_2(request_id).await?.into())
    }
    async fn submit_processor_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsConfigApi {
            client: self.client,
            id,
        };
        Ok(api
            .submit_processor_verification_request(
                &crate::v2_6_0::types::VerifyConfigRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
    async fn terminate_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsThreadsApi {
            client: self.client,
            id,
        };
        Ok(api.terminate_processor().await?.into())
    }
    async fn update_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsApi {
            client: self.client,
        };
        Ok(api
            .update_processor(id, &crate::v2_6_0::types::ProcessorEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn update_run_status_4(
        &self,
        id: &str,
        body: types::ProcessorRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        let api = crate::v2_6_0::api::processors::ProcessorsRunStatusApi {
            client: self.client,
            id,
        };
        Ok(api
            .update_run_status_4(&crate::v2_6_0::types::ProcessorRunStatusEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
}

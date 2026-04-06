// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ControllerServicesApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesApi<'a> {
    /// Deletes a controller service
    ///
    /// Calls `DELETE /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ControllerServiceEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = version {
            query.push(("version", v.to_string()));
        }
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/controller-services/{id}"), &query)
            .await
    }
    /// Gets a controller service
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    pub async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ControllerServiceEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = ui_only {
            query.push(("uiOnly", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/controller-services/{id}"), &query)
            .await
    }
    /// Updates a controller service
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    /// - `body`: The controller service configuration details.
    pub async fn update_controller_service(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ControllerServiceEntity,
    ) -> Result<crate::v2_8_0::types::ControllerServiceEntity, NifiError> {
        self.client
            .put(&format!("/controller-services/{id}"), body)
            .await
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> ControllerServicesBulletinsApi<'b> {
        ControllerServicesBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `config` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn config<'b>(&'b self, id: &'b str) -> ControllerServicesConfigApi<'b> {
        ControllerServicesConfigApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `descriptors` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn descriptors<'b>(&'b self, id: &'b str) -> ControllerServicesDescriptorsApi<'b> {
        ControllerServicesDescriptorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `references` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn references<'b>(&'b self, id: &'b str) -> ControllerServicesReferencesApi<'b> {
        ControllerServicesReferencesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> ControllerServicesRunStatusApi<'b> {
        ControllerServicesRunStatusApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The controller service id.
    pub fn state<'b>(&'b self, id: &'b str) -> ControllerServicesStateApi<'b> {
        ControllerServicesStateApi {
            client: self.client,
            id,
        }
    }
}
pub struct ControllerServicesBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesBulletinsApi<'a> {
    /// Clears bulletins for a controller service
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The clear bulletin request specifying the timestamp from which to clear bulletins.
    pub async fn clear_bulletins(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/controller-services/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct ControllerServicesConfigApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesConfigApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The configuration analysis request.
    pub async fn analyze_configuration(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ConfigurationAnalysisEntity = self
            .client
            .post(&format!("/controller-services/{id}/config/analysis"), body)
            .await?;
        Ok(e.configuration_analysis)
    }
    /// Performs verification of the Controller Service's configuration
    ///
    /// This will initiate the process of verifying a given Controller Service configuration. This may be a long-running task. As a result, this endpoint will immediately return a ControllerServiceConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /controller-services/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /controller-services/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The controller service configuration verification request.
    pub async fn submit_config_verification_request(
        &self,
        body: &crate::v2_8_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/controller-services/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request)
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller-services/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn delete_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/controller-services/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn get_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/controller-services/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
}
pub struct ControllerServicesDescriptorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesDescriptorsApi<'a> {
    /// Gets a controller service property descriptor
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `property_name`: The property name to return the descriptor for.
    /// - `sensitive`: Property Descriptor requested sensitive status
    pub async fn get_property_descriptor_1(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("propertyName", property_name.to_string()));
        if let Some(v) = sensitive {
            query.push(("sensitive", v.to_string()));
        }
        let e: crate::v2_8_0::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(&format!("/controller-services/{id}/descriptors"), &query)
            .await?;
        Ok(e.property_descriptor)
    }
}
pub struct ControllerServicesReferencesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesReferencesApi<'a> {
    /// Gets a controller service
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/references`.
    pub async fn get_controller_service_references(
        &self,
    ) -> Result<crate::v2_8_0::types::ControllerServiceReferencingComponentsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/controller-services/{id}/references"))
            .await
    }
    /// Updates a controller services references
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}/references`.
    ///
    /// # Parameters
    /// - `body`: The controller service request update request.
    pub async fn update_controller_service_references(
        &self,
        body: &crate::v2_8_0::types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<crate::v2_8_0::types::ControllerServiceReferencingComponentsEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/controller-services/{id}/references"), body)
            .await
    }
}
pub struct ControllerServicesRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesRunStatusApi<'a> {
    /// Updates run status of a controller service
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The controller service run status.
    pub async fn update_run_status_1(
        &self,
        body: &crate::v2_8_0::types::ControllerServiceRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::ControllerServiceEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/controller-services/{id}/run-status"), body)
            .await
    }
}
pub struct ControllerServicesStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerServicesStateApi<'a> {
    /// Gets the state for a controller service
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/state`.
    pub async fn get_state(&self) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .get(&format!("/controller-services/{id}/state"))
            .await?;
        Ok(e.component_state)
    }
    /// Clears the state for a controller service
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: Optional component state to perform a selective key removal. If omitted, clears all state.
    pub async fn clear_state_1(
        &self,
        body: &crate::v2_8_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .post(
                &format!("/controller-services/{id}/state/clear-requests"),
                body,
            )
            .await?;
        Ok(e.component_state)
    }
}

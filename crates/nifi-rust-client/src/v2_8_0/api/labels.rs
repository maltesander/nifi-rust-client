use crate::NifiClient;
use crate::NifiError;
pub struct LabelsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> LabelsApi<'a> {
    /// Deletes a label
    ///
    /// Calls `DELETE /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_label(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::LabelEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/labels/{id}"), &query)
            .await
    }
    /// Gets a label
    ///
    /// Calls `GET /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
    pub async fn get_label(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::LabelEntity, NifiError> {
        self.client.get(&format!("/labels/{id}")).await
    }
    /// Updates a label
    ///
    /// Calls `PUT /nifi-api/labels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The label id.
    /// - `body`: The label configuration details.
    pub async fn update_label(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::LabelEntity,
    ) -> Result<crate::v2_8_0::types::LabelEntity, NifiError> {
        self.client.put(&format!("/labels/{id}"), body).await
    }
}

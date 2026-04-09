// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct DataTransferApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> DataTransferApi<'a> {
    /// Scope operations to the `transactions` sub-resource of a specific process group.
    ///
    /// - `port_id`: The input port id.
    pub fn transactions<'b>(&'b self, port_id: &'b str) -> DataTransferTransactionsApi<'b> {
        DataTransferTransactionsApi {
            client: self.client,
            port_id,
        }
    }
}
pub struct DataTransferTransactionsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) port_id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> DataTransferTransactionsApi<'a> {
    /// Commit or cancel the specified transaction
    ///
    /// Calls `DELETE /nifi-api/data-transfer/input-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Parameters
    /// - `transaction_id`: The transaction id.
    /// - `response_code`: The response code. Available values are BAD_CHECKSUM(19), CONFIRM_TRANSACTION(12) or CANCEL_TRANSACTION(15).
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/input-ports/{uuid}`.
    pub async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("responseCode", response_code.to_string()));
        self.client
            .delete_returning_with_query(
                &format!("/data-transfer/input-ports/{port_id}/transactions/{transaction_id}"),
                &query,
            )
            .await
    }
    /// Extend transaction TTL
    ///
    /// Calls `PUT /nifi-api/data-transfer/input-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/input-ports/{uuid}`.
    pub async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        self.client
            .put_no_body(&format!(
                "/data-transfer/input-ports/{port_id}/transactions/{transaction_id}"
            ))
            .await
    }
    /// Transfer flow files to the input port
    ///
    /// Calls `POST /nifi-api/data-transfer/input-ports/{portId}/transactions/{transactionId}/flow-files`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/input-ports/{uuid}`.
    pub async fn receive_flow_files(
        &self,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        let port_id = self.port_id;
        self.client
            .post_void_octet_stream(
                &format!(
                    "/data-transfer/input-ports/{port_id}/transactions/{transaction_id}/flow-files"
                ),
                filename,
                data,
            )
            .await
    }
    /// Commit or cancel the specified transaction
    ///
    /// Calls `DELETE /nifi-api/data-transfer/output-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Parameters
    /// - `transaction_id`: The transaction id.
    /// - `response_code`: The response code. Available values are CONFIRM_TRANSACTION(12) or CANCEL_TRANSACTION(15).
    /// - `checksum`: A checksum calculated at client side using CRC32 to check flow file content integrity. It must match with the value calculated at server side.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/output-ports/{uuid}`.
    pub async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("responseCode", response_code.to_string()));
        query.push(("checksum", checksum.to_string()));
        self.client
            .delete_returning_with_query(
                &format!("/data-transfer/output-ports/{port_id}/transactions/{transaction_id}"),
                &query,
            )
            .await
    }
    /// Extend transaction TTL
    ///
    /// Calls `PUT /nifi-api/data-transfer/output-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/output-ports/{uuid}`.
    pub async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        self.client
            .put_no_body(&format!(
                "/data-transfer/output-ports/{port_id}/transactions/{transaction_id}"
            ))
            .await
    }
    /// Transfer flow files from the output port
    ///
    /// Calls `GET /nifi-api/data-transfer/output-ports/{portId}/transactions/{transactionId}/flow-files`.
    ///
    /// # Returns
    /// - `200`: There is no flow file to return.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/output-ports/{uuid}`.
    pub async fn transfer_flow_files(&self, transaction_id: &str) -> Result<(), NifiError> {
        let port_id = self.port_id;
        self.client
            .get_void(&format!(
                "/data-transfer/output-ports/{port_id}/transactions/{transaction_id}/flow-files"
            ))
            .await
    }
    /// Create a transaction to the specified output port or input port
    ///
    /// Calls `POST /nifi-api/data-transfer/{portType}/{portId}/transactions`.
    ///
    /// # Parameters
    /// - `port_type`: The port type.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    /// - `503`: NiFi instance is not ready for serving request, or temporarily overloaded. Retrying the same request later may be successful
    ///
    /// # Permissions
    /// Requires `Write - /data-transfer/{component-type}/{uuid}`.
    pub async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        self.client
            .post_no_body(&format!(
                "/data-transfer/{port_type}/{port_id}/transactions"
            ))
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::DataTransferApi for DataTransferApi<'_> {
    fn transactions<'b>(
        &'b self,
        port_id: &'b str,
    ) -> impl crate::v2_8_0::traits::DataTransferTransactionsApi + 'b {
        DataTransferTransactionsApi {
            client: self.client,
            port_id,
        }
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::DataTransferTransactionsApi for DataTransferTransactionsApi<'_> {
    async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        self.commit_input_port_transaction(transaction_id, response_code)
            .await
    }
    async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        self.extend_input_port_transaction_t_t_l(transaction_id)
            .await
    }
    async fn receive_flow_files(
        &self,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        self.receive_flow_files(transaction_id, filename, data)
            .await
    }
    async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        self.commit_output_port_transaction(transaction_id, response_code, checksum)
            .await
    }
    async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        self.extend_output_port_transaction_t_t_l(transaction_id)
            .await
    }
    async fn transfer_flow_files(&self, transaction_id: &str) -> Result<(), NifiError> {
        self.transfer_flow_files(transaction_id).await
    }
    async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError> {
        self.create_port_transaction(port_type).await
    }
}

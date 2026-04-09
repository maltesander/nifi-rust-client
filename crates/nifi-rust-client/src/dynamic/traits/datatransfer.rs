// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for DataTransferTransactionsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait DataTransferTransactionsApi {
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
    async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "commit_input_port_transaction".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "commit_output_port_transaction".to_string(),
            version: "unknown".to_string(),
        })
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
    async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_port_transaction".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Extend transaction TTL
    ///
    /// Calls `PUT /nifi-api/data-transfer/input-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Parameters
    /// - `transaction_id`
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
    async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "extend_input_port_transaction_t_t_l".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Extend transaction TTL
    ///
    /// Calls `PUT /nifi-api/data-transfer/output-ports/{portId}/transactions/{transactionId}`.
    ///
    /// # Parameters
    /// - `transaction_id`
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
    async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "extend_output_port_transaction_t_t_l".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Transfer flow files to the input port
    ///
    /// Calls `POST /nifi-api/data-transfer/input-ports/{portId}/transactions/{transactionId}/flow-files`.
    ///
    /// # Parameters
    /// - `transaction_id`
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
    async fn receive_flow_files(
        &self,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "receive_flow_files".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Transfer flow files from the output port
    ///
    /// Calls `GET /nifi-api/data-transfer/output-ports/{portId}/transactions/{transactionId}/flow-files`.
    ///
    /// # Parameters
    /// - `transaction_id`
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
    async fn transfer_flow_files(&self, transaction_id: &str) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "transfer_flow_files".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The DataTransfer API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait DataTransferApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `port_id`: The input port id.
    fn transactions<'b>(&'b self, port_id: &'b str) -> impl DataTransferTransactionsApi + 'b;
}

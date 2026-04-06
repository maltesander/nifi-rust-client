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
    pub async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<crate::v2_7_2::types::TransactionResultEntity, NifiError> {
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
    pub async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_7_2::types::TransactionResultEntity, NifiError> {
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
    pub async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<crate::v2_7_2::types::TransactionResultEntity, NifiError> {
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
    pub async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_7_2::types::TransactionResultEntity, NifiError> {
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
    pub async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<crate::v2_7_2::types::TransactionResultEntity, NifiError> {
        let port_id = self.port_id;
        self.client
            .post_no_body(&format!(
                "/data-transfer/{port_type}/{port_id}/transactions"
            ))
            .await
    }
}

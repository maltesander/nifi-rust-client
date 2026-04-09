// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `transactions` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait DataTransferTransactionsApi {
    /// Commit or cancel the specified transaction
    async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError>;
    /// Extend transaction TTL
    async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError>;
    /// Transfer flow files to the input port
    async fn receive_flow_files(
        &self,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError>;
    /// Commit or cancel the specified transaction
    async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError>;
    /// Extend transaction TTL
    async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError>;
    /// Transfer flow files from the output port
    async fn transfer_flow_files(&self, transaction_id: &str) -> Result<(), NifiError>;
    /// Create a transaction to the specified output port or input port
    async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<crate::v2_8_0::types::TransactionResultEntity, NifiError>;
}
/// The DataTransfer API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait DataTransferApi {
    fn transactions<'b>(&'b self, port_id: &'b str) -> impl DataTransferTransactionsApi + 'b;
}

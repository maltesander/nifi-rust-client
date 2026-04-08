// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::DataTransferApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0DataTransferApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl DataTransferApi for V2_6_0DataTransferApi<'_> {
    async fn commit_input_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        Ok(api
            .commit_input_port_transaction(transaction_id, response_code)
            .await?
            .into())
    }
    async fn commit_output_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        Ok(api
            .commit_output_port_transaction(transaction_id, response_code, checksum)
            .await?
            .into())
    }
    async fn create_port_transaction(
        &self,
        port_id: &str,
        port_type: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        Ok(api.create_port_transaction(port_type).await?.into())
    }
    async fn extend_input_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        Ok(api
            .extend_input_port_transaction_t_t_l(transaction_id)
            .await?
            .into())
    }
    async fn extend_output_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        Ok(api
            .extend_output_port_transaction_t_t_l(transaction_id)
            .await?
            .into())
    }
    async fn receive_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        api.receive_flow_files(transaction_id, filename, data).await
    }
    async fn transfer_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<(), NifiError> {
        let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
            client: self.client,
            port_id,
        };
        api.transfer_flow_files(transaction_id).await
    }
}

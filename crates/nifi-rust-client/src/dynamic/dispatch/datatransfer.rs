// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::DataTransferApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the DataTransfer API. Use via the [`DataTransferApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum DataTransferApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0DataTransferApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2DataTransferApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0DataTransferApi<'a>),
}
impl DataTransferApi for DataTransferApiDispatch<'_> {
    async fn commit_input_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.commit_input_port_transaction(port_id, transaction_id, response_code)
                    .await
            }
            Self::V2_7_2(api) => {
                api.commit_input_port_transaction(port_id, transaction_id, response_code)
                    .await
            }
            Self::V2_8_0(api) => {
                api.commit_input_port_transaction(port_id, transaction_id, response_code)
                    .await
            }
        }
    }
    async fn commit_output_port_transaction(
        &self,
        port_id: &str,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.commit_output_port_transaction(port_id, transaction_id, response_code, checksum)
                    .await
            }
            Self::V2_7_2(api) => {
                api.commit_output_port_transaction(port_id, transaction_id, response_code, checksum)
                    .await
            }
            Self::V2_8_0(api) => {
                api.commit_output_port_transaction(port_id, transaction_id, response_code, checksum)
                    .await
            }
        }
    }
    async fn create_port_transaction(
        &self,
        port_id: &str,
        port_type: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_port_transaction(port_id, port_type).await,
            Self::V2_7_2(api) => api.create_port_transaction(port_id, port_type).await,
            Self::V2_8_0(api) => api.create_port_transaction(port_id, port_type).await,
        }
    }
    async fn extend_input_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.extend_input_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.extend_input_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.extend_input_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
        }
    }
    async fn extend_output_port_transaction_t_t_l(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.extend_output_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
            Self::V2_7_2(api) => {
                api.extend_output_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
            Self::V2_8_0(api) => {
                api.extend_output_port_transaction_t_t_l(port_id, transaction_id)
                    .await
            }
        }
    }
    async fn receive_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.receive_flow_files(port_id, transaction_id, filename, data)
                    .await
            }
            Self::V2_7_2(api) => {
                api.receive_flow_files(port_id, transaction_id, filename, data)
                    .await
            }
            Self::V2_8_0(api) => {
                api.receive_flow_files(port_id, transaction_id, filename, data)
                    .await
            }
        }
    }
    async fn transfer_flow_files(
        &self,
        port_id: &str,
        transaction_id: &str,
    ) -> Result<(), NifiError> {
        match self {
            Self::V2_6_0(api) => api.transfer_flow_files(port_id, transaction_id).await,
            Self::V2_7_2(api) => api.transfer_flow_files(port_id, transaction_id).await,
            Self::V2_8_0(api) => api.transfer_flow_files(port_id, transaction_id).await,
        }
    }
}

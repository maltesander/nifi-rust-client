// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::DataTransferApi;
use crate::dynamic::traits::DataTransferTransactionsApi;
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
impl<'a> DataTransferApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl DataTransferApi for DataTransferApiDispatch<'_> {
    type DataTransferTransactionsApi<'b>
        = DataTransferTransactionsApiDispatch<'b>
    where
        Self: 'b;
    fn transactions<'b>(&'b self, port_id: &'b str) -> Self::DataTransferTransactionsApi<'b> {
        DataTransferTransactionsApiDispatch {
            client: self.client(),
            port_id: port_id.to_string(),
            version: self.version(),
        }
    }
}
/// Sub-resource dispatch struct for [DataTransferTransactionsApi].
pub struct DataTransferTransactionsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) port_id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl DataTransferTransactionsApi for DataTransferTransactionsApiDispatch<'_> {
    async fn commit_input_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_input_port_transaction(transaction_id, response_code)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "commit_input_port_transaction".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn commit_output_port_transaction(
        &self,
        transaction_id: &str,
        response_code: i32,
        checksum: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .commit_output_port_transaction(transaction_id, response_code, checksum)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "commit_output_port_transaction".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn create_port_transaction(
        &self,
        port_type: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api.create_port_transaction(port_type).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_port_transaction".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn extend_input_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_input_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "extend_input_port_transaction_t_t_l".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn extend_output_port_transaction_t_t_l(
        &self,
        transaction_id: &str,
    ) -> Result<types::TransactionResultEntity, NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                Ok(api
                    .extend_output_port_transaction_t_t_l(transaction_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "extend_output_port_transaction_t_t_l".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn receive_flow_files(
        &self,
        transaction_id: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.receive_flow_files(transaction_id, filename, data).await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "receive_flow_files".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn transfer_flow_files(&self, transaction_id: &str) -> Result<(), NifiError> {
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::datatransfer::DataTransferTransactionsApi {
                    client: self.client,
                    port_id: &self.port_id,
                };
                api.transfer_flow_files(transaction_id).await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "transfer_flow_files".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

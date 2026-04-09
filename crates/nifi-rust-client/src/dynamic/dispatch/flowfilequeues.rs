// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowFileQueuesApi;
use crate::dynamic::traits::FlowFileQueuesDropRequestsApi;
use crate::dynamic::traits::FlowFileQueuesFlowfilesApi;
use crate::dynamic::traits::FlowFileQueuesListingRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the FlowFileQueues API. Use via the [`FlowFileQueuesApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum FlowFileQueuesApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0FlowFileQueuesApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2FlowFileQueuesApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0FlowFileQueuesApi<'a>),
}
impl<'a> FlowFileQueuesApiDispatch<'a> {
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
impl FlowFileQueuesApi for FlowFileQueuesApiDispatch<'_> {
    fn drop_requests<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesDropRequestsApi + 'b {
        FlowFileQueuesDropRequestsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn flowfiles<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesFlowfilesApi + 'b {
        FlowFileQueuesFlowfilesApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
    fn listing_requests<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesListingRequestsApi + 'b {
        FlowFileQueuesListingRequestsApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
}
/// Sub-resource dispatch struct for [FlowFileQueuesDropRequestsApi].
pub struct FlowFileQueuesDropRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowFileQueuesDropRequestsApi for FlowFileQueuesDropRequestsApiDispatch<'_> {
    async fn create_drop_request(&self) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_drop_request().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_drop_request().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_drop_request().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_drop_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_drop_request(drop_request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_drop_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn remove_drop_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesDropRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.remove_drop_request(drop_request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "remove_drop_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowFileQueuesFlowfilesApi].
pub struct FlowFileQueuesFlowfilesApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowFileQueuesFlowfilesApi for FlowFileQueuesFlowfilesApiDispatch<'_> {
    async fn download_flow_file_content(
        &self,
        flowfile_uuid: &str,
        client_id: Option<&str>,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                api.download_flow_file_content(flowfile_uuid, client_id, cluster_node_id)
                    .await
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "download_flow_file_content".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_flow_file(
        &self,
        flowfile_uuid: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::FlowFileDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesFlowfilesApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .get_flow_file(flowfile_uuid, cluster_node_id)
                    .await?
                    .into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_flow_file".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}
/// Sub-resource dispatch struct for [FlowFileQueuesListingRequestsApi].
pub struct FlowFileQueuesListingRequestsApiDispatch<'a> {
    pub(crate) client: &'a crate::NifiClient,
    pub(crate) id: String,
    pub(crate) version: crate::dynamic::DetectedVersion,
}
impl FlowFileQueuesListingRequestsApi for FlowFileQueuesListingRequestsApiDispatch<'_> {
    async fn create_flow_file_listing(&self) -> Result<types::ListingRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.create_flow_file_listing().await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "create_flow_file_listing".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn delete_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.delete_listing_request(listing_request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "delete_listing_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
    async fn get_listing_request(
        &self,
        listing_request_id: &str,
    ) -> Result<types::ListingRequestDto, NifiError> {
        #[allow(unreachable_patterns)]
        match self.version {
            crate::dynamic::DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_7_2 => {
                let api = crate::v2_7_2::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
            crate::dynamic::DetectedVersion::V2_8_0 => {
                let api = crate::v2_8_0::api::flowfilequeues::FlowFileQueuesListingRequestsApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api.get_listing_request(listing_request_id).await?.into())
            }
            _ => Err(NifiError::UnsupportedEndpoint {
                endpoint: "get_listing_request".to_string(),
                version: "unknown".to_string(),
            }),
        }
    }
}

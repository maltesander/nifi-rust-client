// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProvenanceApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Provenance API. Use via the [`ProvenanceApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum ProvenanceApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0ProvenanceApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2ProvenanceApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0ProvenanceApi<'a>),
}
impl<'a> ProvenanceApiDispatch<'a> {
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
impl ProvenanceApi for ProvenanceApiDispatch<'_> {
    async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_lineage(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.delete_lineage(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.delete_lineage(id, cluster_node_id).await,
        }
    }
    async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.delete_provenance(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.delete_provenance(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.delete_provenance(id, cluster_node_id).await,
        }
    }
    async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_lineage(id, cluster_node_id).await,
            Self::V2_7_2(api) => api.get_lineage(id, cluster_node_id).await,
            Self::V2_8_0(api) => api.get_lineage(id, cluster_node_id).await,
        }
    }
    async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await
            }
            Self::V2_7_2(api) => {
                api.get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await
            }
            Self::V2_8_0(api) => {
                api.get_provenance(id, cluster_node_id, summarize, incremental_results)
                    .await
            }
        }
    }
    async fn get_search_options(&self) -> Result<types::ProvenanceOptionsDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_search_options().await,
            Self::V2_7_2(api) => api.get_search_options().await,
            Self::V2_8_0(api) => api.get_search_options().await,
        }
    }
    async fn submit_lineage_request(
        &self,
        body: &types::LineageEntity,
    ) -> Result<types::LineageDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_lineage_request(body).await,
            Self::V2_7_2(api) => api.submit_lineage_request(body).await,
            Self::V2_8_0(api) => api.submit_lineage_request(body).await,
        }
    }
    async fn submit_provenance_request(
        &self,
        body: &types::ProvenanceEntity,
    ) -> Result<types::ProvenanceDto, NifiError> {
        match self {
            Self::V2_6_0(api) => api.submit_provenance_request(body).await,
            Self::V2_7_2(api) => api.submit_provenance_request(body).await,
            Self::V2_8_0(api) => api.submit_provenance_request(body).await,
        }
    }
}

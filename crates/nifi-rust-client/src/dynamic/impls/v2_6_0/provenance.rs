// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProvenanceApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_6_0ProvenanceApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProvenanceApi for V2_6_0ProvenanceApi<'_> {
    async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api.delete_lineage(id, cluster_node_id).await?.into())
    }
    async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api.delete_provenance(id, cluster_node_id).await?.into())
    }
    async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::LineageDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api.get_lineage(id, cluster_node_id).await?.into())
    }
    async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<types::ProvenanceDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api
            .get_provenance(id, cluster_node_id, summarize, incremental_results)
            .await?
            .into())
    }
    async fn get_search_options(&self) -> Result<types::ProvenanceOptionsDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api.get_search_options().await?.into())
    }
    async fn submit_lineage_request(
        &self,
        body: types::LineageEntity,
    ) -> Result<types::LineageDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api
            .submit_lineage_request(&crate::v2_6_0::types::LineageEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn submit_provenance_request(
        &self,
        body: types::ProvenanceEntity,
    ) -> Result<types::ProvenanceDto, NifiError> {
        let api = crate::v2_6_0::api::provenance::ProvenanceApi {
            client: self.client,
        };
        Ok(api
            .submit_provenance_request(&crate::v2_6_0::types::ProvenanceEntity::try_from(body)?)
            .await?
            .into())
    }
}

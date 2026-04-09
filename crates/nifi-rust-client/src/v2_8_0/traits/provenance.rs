// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Provenance API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProvenanceApi {
    /// Submits a provenance query
    async fn submit_provenance_request(
        &self,
        body: &crate::v2_8_0::types::ProvenanceEntity,
    ) -> Result<crate::v2_8_0::types::ProvenanceDto, NifiError>;
    /// Submits a lineage query
    async fn submit_lineage_request(
        &self,
        body: &crate::v2_8_0::types::LineageEntity,
    ) -> Result<crate::v2_8_0::types::LineageDto, NifiError>;
    /// Deletes a lineage query
    async fn delete_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::LineageDto, NifiError>;
    /// Gets a lineage query
    async fn get_lineage(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::LineageDto, NifiError>;
    /// Gets the searchable attributes for provenance events
    async fn get_search_options(
        &self,
    ) -> Result<crate::v2_8_0::types::ProvenanceOptionsDto, NifiError>;
    /// Deletes a provenance query
    async fn delete_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::ProvenanceDto, NifiError>;
    /// Gets a provenance query
    async fn get_provenance(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
        summarize: Option<bool>,
        incremental_results: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ProvenanceDto, NifiError>;
}

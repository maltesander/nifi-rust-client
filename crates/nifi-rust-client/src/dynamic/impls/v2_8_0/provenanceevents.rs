// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::ProvenanceEventsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0ProvenanceEventsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl ProvenanceEventsApi for V2_8_0ProvenanceEventsApi<'_> {
    async fn get_input_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsContentApi {
            client: self.client,
            id,
        };
        api.get_input_content(cluster_node_id).await
    }
    async fn get_latest_provenance_events(
        &self,
        component_id: &str,
        limit: Option<i32>,
    ) -> Result<types::LatestProvenanceEventsDto, NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
            client: self.client,
        };
        Ok(api
            .get_latest_provenance_events(component_id, limit)
            .await?
            .into())
    }
    async fn get_output_content(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<(), NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsContentApi {
            client: self.client,
            id,
        };
        api.get_output_content(cluster_node_id).await
    }
    async fn get_provenance_event(
        &self,
        id: &str,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
            client: self.client,
        };
        Ok(api.get_provenance_event(id, cluster_node_id).await?.into())
    }
    async fn submit_replay(
        &self,
        body: types::SubmitReplayRequestEntity,
    ) -> Result<types::ProvenanceEventDto, NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
            client: self.client,
        };
        Ok(api
            .submit_replay(&crate::v2_8_0::types::SubmitReplayRequestEntity::try_from(
                body,
            )?)
            .await?
            .into())
    }
    async fn submit_replay_latest_event(
        &self,
        body: types::ReplayLastEventRequestEntity,
    ) -> Result<types::ReplayLastEventResponseEntity, NifiError> {
        let api = crate::v2_8_0::api::provenanceevents::ProvenanceEventsApi {
            client: self.client,
        };
        Ok(api
            .submit_replay_latest_event(
                &crate::v2_8_0::types::ReplayLastEventRequestEntity::try_from(body)?,
            )
            .await?
            .into())
    }
}

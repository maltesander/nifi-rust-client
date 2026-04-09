// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::FlowFileQueuesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::FlowFileQueuesDropRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::traits::FlowFileQueuesFlowfilesApi;
#[allow(unused_imports)]
use crate::dynamic::traits::FlowFileQueuesListingRequestsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_7_2FlowFileQueuesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl FlowFileQueuesApi for V2_7_2FlowFileQueuesApi<'_> {
    type FlowFileQueuesDropRequestsApi<'b>
        = crate::dynamic::dispatch::FlowFileQueuesDropRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn drop_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesDropRequestsApi<'b> {
        crate::dynamic::dispatch::FlowFileQueuesDropRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type FlowFileQueuesFlowfilesApi<'b>
        = crate::dynamic::dispatch::FlowFileQueuesFlowfilesApiDispatch<'b>
    where
        Self: 'b;
    fn flowfiles<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesFlowfilesApi<'b> {
        crate::dynamic::dispatch::FlowFileQueuesFlowfilesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
    type FlowFileQueuesListingRequestsApi<'b>
        = crate::dynamic::dispatch::FlowFileQueuesListingRequestsApiDispatch<'b>
    where
        Self: 'b;
    fn listing_requests<'b>(&'b self, id: &'b str) -> Self::FlowFileQueuesListingRequestsApi<'b> {
        crate::dynamic::dispatch::FlowFileQueuesListingRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_7_2,
        }
    }
}

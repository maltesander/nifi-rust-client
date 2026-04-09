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
pub(crate) struct V2_8_0FlowFileQueuesApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl FlowFileQueuesApi for V2_8_0FlowFileQueuesApi<'_> {
    fn drop_requests<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesDropRequestsApi + 'b {
        crate::dynamic::dispatch::FlowFileQueuesDropRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn flowfiles<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesFlowfilesApi + 'b {
        crate::dynamic::dispatch::FlowFileQueuesFlowfilesApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
    fn listing_requests<'b>(&'b self, id: &'b str) -> impl FlowFileQueuesListingRequestsApi + 'b {
        crate::dynamic::dispatch::FlowFileQueuesListingRequestsApiDispatch {
            client: self.client,
            id: id.to_string(),
            version: crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}

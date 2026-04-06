// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResultEntity {
    pub flow_file_sent: Option<i32>,
    pub message: Option<String>,
    pub response_code: Option<i32>,
}

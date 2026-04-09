// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Resources API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ResourcesApi {
    /// Gets the available resources that support access/authorization policies
    async fn get_resources(&self) -> Result<crate::v2_8_0::types::ResourcesEntity, NifiError>;
}

use crate::error::CliError;
use crate::output::CliOutput;
use nifi_rust_client::dynamic::DynamicClient;

pub async fn status(client: &DynamicClient) -> Result<CliOutput, CliError> {
    let about = client.flow().get_about_info().await?;
    let value = serde_json::to_value(&about)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

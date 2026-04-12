use crate::error::CliError;
use serde_json::Value;
use std::io::{self, IsTerminal, Read};
use std::path::Path;

/// Resolve a JSON body from --body, --body-file, or stdin.
pub fn resolve_body(inline: Option<&str>, file_path: Option<&Path>) -> Result<Value, CliError> {
    if let Some(json_str) = inline {
        serde_json::from_str(json_str)
            .map_err(|e| CliError::User(format!("invalid --body JSON: {e}")))
    } else if let Some(path) = file_path {
        let content = std::fs::read_to_string(path).map_err(|e| {
            CliError::User(format!("failed to read body file {}: {e}", path.display()))
        })?;
        serde_json::from_str(&content)
            .map_err(|e| CliError::User(format!("invalid JSON in {}: {e}", path.display())))
    } else if !io::stdin().is_terminal() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        serde_json::from_str(&buf)
            .map_err(|e| CliError::User(format!("invalid JSON from stdin: {e}")))
    } else {
        Err(CliError::User(
            "request body required — use --body, --body-file, or pipe JSON to stdin".to_string(),
        ))
    }
}

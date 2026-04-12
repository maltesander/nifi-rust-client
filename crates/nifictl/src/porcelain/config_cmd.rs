use crate::error::CliError;
use std::path::Path;

pub fn use_context(config_path: &Path, name: &str) -> Result<(), CliError> {
    let content = std::fs::read_to_string(config_path)?;
    let mut doc: toml::Table =
        toml::from_str(&content).map_err(|e| CliError::User(format!("invalid config: {e}")))?;

    // Verify context exists
    let contexts = doc
        .get("contexts")
        .and_then(|v| v.as_array())
        .ok_or_else(|| CliError::User("no contexts defined".to_string()))?;
    let exists = contexts
        .iter()
        .any(|c| c.get("name").and_then(|n| n.as_str()) == Some(name));
    if !exists {
        return Err(CliError::User(format!("unknown context: '{name}'")));
    }

    doc.insert(
        "current_context".to_string(),
        toml::Value::String(name.to_string()),
    );
    let output = toml::to_string_pretty(&doc)
        .map_err(|e| CliError::User(format!("failed to serialize config: {e}")))?;
    std::fs::write(config_path, output)?;
    eprintln!("Switched to context '{name}'");
    Ok(())
}

pub fn delete_context(config_path: &Path, name: &str) -> Result<(), CliError> {
    let content = std::fs::read_to_string(config_path)?;
    let mut doc: toml::Table =
        toml::from_str(&content).map_err(|e| CliError::User(format!("invalid config: {e}")))?;

    let contexts = doc
        .get_mut("contexts")
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| CliError::User("no contexts defined".to_string()))?;
    let before = contexts.len();
    contexts.retain(|c| c.get("name").and_then(|n| n.as_str()) != Some(name));
    if contexts.len() == before {
        return Err(CliError::User(format!("unknown context: '{name}'")));
    }

    if doc.get("current_context").and_then(|v| v.as_str()) == Some(name) {
        doc.remove("current_context");
    }

    let output = toml::to_string_pretty(&doc)
        .map_err(|e| CliError::User(format!("failed to serialize config: {e}")))?;
    std::fs::write(config_path, output)?;
    eprintln!("Deleted context '{name}'");
    Ok(())
}

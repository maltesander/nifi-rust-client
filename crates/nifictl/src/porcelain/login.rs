use crate::client_factory::ResolvedParams;
use crate::error::CliError;
use std::path::PathBuf;

fn token_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".nifictl").join("tokens")
}

fn token_path(context_name: &str) -> PathBuf {
    token_dir().join(context_name)
}

pub async fn login(params: &ResolvedParams, context_name: &str) -> Result<(), CliError> {
    let client = params.build_client().await?;
    if let Some(token) = client.token().await {
        std::fs::create_dir_all(token_dir())?;
        std::fs::write(token_path(context_name), &token)?;
        eprintln!("Logged in to {} (token cached)", params.url);
        if let Some(version) = client.detected_version() {
            eprintln!("NiFi version: {version}");
        }
    }
    Ok(())
}

pub fn logout(context_name: &str) -> Result<(), CliError> {
    let path = token_path(context_name);
    if path.exists() {
        std::fs::remove_file(&path)?;
        eprintln!("Logged out (token cleared for context '{context_name}')");
    } else {
        eprintln!("No cached token for context '{context_name}'");
    }
    Ok(())
}

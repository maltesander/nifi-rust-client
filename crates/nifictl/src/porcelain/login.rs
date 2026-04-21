use crate::client_factory::ResolvedParams;
use crate::error::CliError;
use crate::jwt;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const EXPIRY_WARNING_THRESHOLD: Duration = Duration::from_secs(24 * 3600);

/// Format `"warning: token expires in Xh Ym"` for a remaining duration.
fn format_expiry_warning(remaining: Duration) -> String {
    let total_minutes = remaining.as_secs() / 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("warning: token expires in {hours}h {minutes}m")
}

/// Return the warning line for a token if expiry is below the threshold,
/// or `None` if it is not. Silent on malformed tokens.
fn expiry_warning_for(token: &str, now: SystemTime) -> Option<String> {
    let remaining = jwt::expiry_remaining(token, now)?;
    if remaining < EXPIRY_WARNING_THRESHOLD {
        Some(format_expiry_warning(remaining))
    } else {
        None
    }
}

fn token_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".nifictl").join("tokens")
}

fn token_path(context_name: &str) -> PathBuf {
    token_dir().join(context_name)
}

/// Write the cached token to disk at `path`. On Unix, sets the file's
/// permissions to `0o600` after write so a multi-user system does not
/// expose the JWT. On non-Unix platforms the umask applies.
pub(crate) fn write_token_file(path: &std::path::Path, token: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // Best-effort on parent dir — don't fail login if the dir already
            // exists with different perms.
            let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));
        }
    }
    std::fs::write(path, token)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

pub async fn login(params: &ResolvedParams, context_name: &str) -> Result<(), CliError> {
    let client = params.build_client().await?;
    if let Some(token) = client.token().await {
        write_token_file(&token_path(context_name), &token)?;
        eprintln!("Logged in to {} (token cached)", params.url);
        if let Some(version) = client.detected_version() {
            eprintln!("NiFi version: {version}");
        }
        if let Some(msg) = expiry_warning_for(&token, SystemTime::now()) {
            eprintln!("{msg}");
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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use std::time::{Duration, UNIX_EPOCH};

    fn jwt_with_exp(exp_secs: i64) -> String {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256"}"#);
        let payload = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{exp_secs}}}"#));
        let sig = URL_SAFE_NO_PAD.encode(b"sig");
        format!("{header}.{payload}.{sig}")
    }

    #[test]
    fn format_expiry_warning_rounds_to_hours_and_minutes() {
        let msg = format_expiry_warning(Duration::from_secs(3 * 3600 + 42 * 60));
        assert_eq!(msg, "warning: token expires in 3h 42m");
    }

    #[test]
    fn format_expiry_warning_handles_under_an_hour() {
        let msg = format_expiry_warning(Duration::from_secs(17 * 60));
        assert_eq!(msg, "warning: token expires in 0h 17m");
    }

    #[test]
    fn should_warn_when_under_threshold() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let in_five_hours = 1_700_000_000 + 5 * 3600;
        let token = jwt_with_exp(in_five_hours);
        let msg = expiry_warning_for(&token, now);
        assert_eq!(msg.as_deref(), Some("warning: token expires in 5h 0m"));
    }

    #[test]
    fn should_not_warn_beyond_threshold() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let in_48h = 1_700_000_000 + 48 * 3600;
        let token = jwt_with_exp(in_48h);
        assert!(expiry_warning_for(&token, now).is_none());
    }

    #[test]
    fn should_not_warn_at_exact_threshold() {
        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let exactly_24h = 1_700_000_000 + 24 * 3600;
        let token = jwt_with_exp(exactly_24h);
        assert!(expiry_warning_for(&token, now).is_none());
    }

    #[test]
    fn should_not_warn_for_malformed_token() {
        let now = UNIX_EPOCH;
        assert!(expiry_warning_for("garbage", now).is_none());
    }

    #[cfg(unix)]
    #[test]
    fn token_file_is_written_with_mode_0o600() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("token");
        super::write_token_file(&path, "jwt-payload").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(
            mode, 0o600,
            "token file must be owner-read/write only, got {:o}",
            mode
        );
    }

    #[cfg(unix)]
    #[test]
    fn token_file_overwrite_resets_mode_to_0o600() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("token");
        std::fs::write(&path, "old").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        super::write_token_file(&path, "new-jwt").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new-jwt");
    }
}

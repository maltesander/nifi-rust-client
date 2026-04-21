//! Interactive password prompt via `rpassword`.
//!
//! On a TTY, echo-free prompt for a password. Off-TTY, refuse with a
//! clear `CliError::User` pointing at the env var + flag fallback.

use std::io::{self, IsTerminal};

use crate::error::CliError;

pub(crate) const PROMPT_NON_TTY_ERROR: &str = "no password available and stdin is not a TTY";

pub(crate) const PROMPT_NON_TTY_HINT: &str = "set NIFI_PASSWORD or pass --password";

pub(crate) fn format_prompt(username: &str, base_url: &str) -> String {
    format!("Password for {username}@{base_url}: ")
}

/// Prompt the user for a password (no echo). Errors off-TTY.
pub fn prompt_password(username: &str, base_url: &str) -> Result<String, CliError> {
    if !io::stdin().is_terminal() {
        return Err(CliError::User(format!(
            "{PROMPT_NON_TTY_ERROR}\nhint: {PROMPT_NON_TTY_HINT}"
        )));
    }
    let prompt = format_prompt(username, base_url);
    rpassword::prompt_password(&prompt).map_err(CliError::Io)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_format_matches_spec() {
        assert_eq!(
            format_prompt("admin", "https://nifi:8443"),
            "Password for admin@https://nifi:8443: "
        );
    }

    // TTY behavior is exercised end-to-end in `tests/cli_integration.rs`
    // because `io::stdin().is_terminal()` can't be faked in a unit test.
}

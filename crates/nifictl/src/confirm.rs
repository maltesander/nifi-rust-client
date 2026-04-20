//! Confirmation prompts for destructive commands.
//!
//! Contract:
//! * `--yes` / `-y` bypasses.
//! * Non-TTY without `--yes` → error, no silent skip.
//! * TTY → read one line from stdin, require exact `y`/`yes` (case-insensitive).

use std::io::{self, BufRead, IsTerminal, Write};

use crate::dry_run::CliCtx;
use crate::error::CliError;

/// Error message returned when a confirmable command is run in a
/// non-interactive environment without `--yes`. Exposed as a const so
/// integration tests can assert the exact text.
pub const NON_INTERACTIVE_ERROR: &str =
    "refusing to run destructive command without --yes in non-interactive mode";

/// Prompt wording helper. `what` is the short description of what
/// would happen, e.g. `"delete processor 'proc-1'"`.
pub(crate) fn prompt_message(what: &str) -> String {
    format!("About to {what}. Continue? [y/N]: ")
}

/// Gate on confirmation. Returns `Ok(())` when the user (or `--yes`) agrees.
pub fn confirm_destructive(what: &str, ctx: &CliCtx) -> Result<(), CliError> {
    if ctx.yes {
        return Ok(());
    }
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        return Err(CliError::User(NON_INTERACTIVE_ERROR.to_string()));
    }
    let mut stderr = io::stderr();
    write!(stderr, "{}", prompt_message(what)).map_err(CliError::Io)?;
    stderr.flush().map_err(CliError::Io)?;
    let mut line = String::new();
    stdin.lock().read_line(&mut line).map_err(CliError::Io)?;
    let trimmed = line.trim().to_ascii_lowercase();
    if trimmed == "y" || trimmed == "yes" {
        Ok(())
    } else {
        Err(CliError::User("cancelled".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_bypasses_prompt() {
        let ctx = CliCtx {
            dry_run: false,
            yes: true,
            base_url: "https://x",
        };
        confirm_destructive("delete the world", &ctx).unwrap();
    }

    #[test]
    fn prompt_message_formats_as_expected() {
        assert_eq!(
            prompt_message("delete processor 'p1'"),
            "About to delete processor 'p1'. Continue? [y/N]: "
        );
    }

    // Non-TTY behavior is exercised in `tests/cli_integration.rs` via a
    // spawned process — here we only check the `--yes` and formatter paths
    // because io::stdin().is_terminal() can't be faked in a unit test.
}

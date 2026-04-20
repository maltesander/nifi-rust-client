use std::process::Command;

fn nifictl() -> Command {
    Command::new(env!("CARGO_BIN_EXE_nifictl"))
}

#[test]
fn help_shows_resources() {
    let output = nifictl()
        .arg("--help")
        .output()
        .expect("failed to run nifictl");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "nifictl --help failed: {stdout}");
    assert!(stdout.contains("processors"), "help should list processors");
    assert!(stdout.contains("flow"), "help should list flow");
    assert!(stdout.contains("config"), "help should list config");
    assert!(
        stdout.contains("completions"),
        "help should list completions"
    );
    assert!(stdout.contains("login"), "help should list login");
    assert!(stdout.contains("status"), "help should list status");
}

#[test]
fn completions_bash() {
    let output = nifictl()
        .args(["completions", "bash"])
        .output()
        .expect("failed");
    assert!(output.status.success(), "completions failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("nifictl"),
        "completions should reference nifictl"
    );
}

#[test]
fn completions_fish() {
    let output = nifictl()
        .args(["completions", "fish"])
        .output()
        .expect("failed");
    assert!(output.status.success());
}

#[test]
fn completions_zsh() {
    let output = nifictl()
        .args(["completions", "zsh"])
        .output()
        .expect("failed");
    assert!(output.status.success());
}

#[test]
fn config_view_no_file() {
    let output = nifictl()
        .args(["--config", "/nonexistent/path.toml", "config", "view"])
        .output()
        .expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No config file found"));
}

#[test]
fn config_view_with_file() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    std::fs::write(
        &config_path,
        r#"
current_context = "test"

[[contexts]]
name = "test"
url = "https://localhost:8443"

[contexts.auth]
type = "password"
username = "admin"
password_env = "TEST_PW"
"#,
    )
    .unwrap();

    let output = nifictl()
        .args(["--config", &config_path.to_string_lossy(), "config", "view"])
        .output()
        .expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test"));
}

#[test]
fn config_contexts_list() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    std::fs::write(
        &config_path,
        r#"
current_context = "dev"

[[contexts]]
name = "dev"
url = "https://localhost:8443"

[contexts.auth]
type = "password"
username = "admin"
password_env = "PW"

[[contexts]]
name = "prod"
url = "https://nifi-prod:8443"

[contexts.auth]
type = "token"
token_env = "JWT"
"#,
    )
    .unwrap();

    let output = nifictl()
        .args([
            "--config",
            &config_path.to_string_lossy(),
            "config",
            "contexts",
        ])
        .output()
        .expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dev"));
    assert!(stdout.contains("prod"));
}

#[test]
fn processors_help_shows_subcommands() {
    let output = nifictl()
        .args(["processors", "--help"])
        .output()
        .expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("get"), "should have get command");
    assert!(stdout.contains("delete"), "should have delete command");
}

/// `--dry-run` with `--url` pointed at localhost:1 (closed port) must still
/// exit 0 and NOT attempt any HTTP call. If the command tried to connect,
/// it would fail with a connection error.
#[test]
fn dry_run_on_ops_stop_pg_does_not_connect() {
    let output = nifictl()
        .args([
            "--url",
            "http://localhost:1",
            "--username",
            "admin",
            "--password",
            "x",
            "--dry-run",
            "ops",
            "stop-pg",
            "pg-1",
        ])
        .output()
        .expect("failed to run nifictl");
    assert!(
        output.status.success(),
        "dry-run should exit 0; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("DRY RUN \u{2014} would send:"),
        "stdout should contain dry-run preamble: {stdout}"
    );
    assert!(
        stdout.contains("PUT http://localhost:1/nifi-api/flow/process-groups/pg-1"),
        "stdout should contain the target URL: {stdout}"
    );
    assert!(
        stdout.contains("\"state\": \"STOPPED\""),
        "stdout should show STOPPED body: {stdout}"
    );
}

/// Regression pin for Task 4 — `--help` must list every generated
/// resource at the top level, regardless of flatten vs explicit
/// enumeration.
#[test]
fn help_lists_all_generated_resources() {
    let output = nifictl()
        .arg("--help")
        .output()
        .expect("failed to run nifictl");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for resource in [
        "access",
        "authentication",
        "connections",
        "connectors",
        "controller",
        "controller_services",
        "counters",
        "datatransfer",
        "flow",
        "flowfilequeues",
        "funnels",
        "inputports",
        "labels",
        "outputports",
        "parametercontexts",
        "parameterproviders",
        "policies",
        "processgroups",
        "processors",
        "provenance",
        "provenanceevents",
        "remoteprocessgroups",
        "reportingtasks",
        "resources",
        "sitetosite",
        "snippets",
        "systemdiagnostics",
        "tenants",
        "versions",
    ] {
        // Whole-word match anchored on clap's subcommand-listing format
        // ("  <name>  <description>"). Substring matching would falsely
        // pass on e.g. "system" if `systemdiagnostics` were the only
        // listed variant.
        let found = stdout
            .lines()
            .any(|l| l.trim_start().starts_with(&format!("{resource} ")));
        assert!(
            found,
            "top-level help missing whole-word '{resource}': {stdout}"
        );
    }
}

/// `ops stop-pg` without `--yes` in non-TTY must refuse with a clear error.
#[test]
fn ops_stop_pg_without_yes_in_non_tty_refuses() {
    let output = nifictl()
        .args([
            "--url",
            "http://localhost:1",
            "--username",
            "admin",
            "--password",
            "x",
            "ops",
            "stop-pg",
            "pg-1",
        ])
        .output()
        .expect("failed to run nifictl");
    assert!(
        !output.status.success(),
        "expected non-zero exit; stdout={}",
        String::from_utf8_lossy(&output.stdout),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("refusing to run destructive command without --yes"),
        "stderr should contain refusal message: {stderr}"
    );
}

/// `flow export --dry-run` must print the GET URL and exit 0 without
/// hitting the server.
#[test]
fn dry_run_on_flow_export_does_not_connect() {
    let output = nifictl()
        .args([
            "--url",
            "http://localhost:1",
            "--username",
            "admin",
            "--password",
            "x",
            "--dry-run",
            "flow",
            "export",
            "pg-1",
        ])
        .output()
        .expect("failed to run nifictl");
    assert!(
        output.status.success(),
        "dry-run should exit 0; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("DRY RUN"),
        "stdout should contain dry-run preamble: {stdout}"
    );
    assert!(
        stdout.contains("GET http://localhost:1/nifi-api/process-groups/pg-1/download"),
        "stdout should contain the target URL: {stdout}"
    );
}

/// `flow replace` without `--yes` in non-TTY must refuse with a clear
/// error, without creating the snapshot file or touching the server.
#[test]
fn flow_replace_without_yes_in_non_tty_refuses() {
    let output = nifictl()
        .args([
            "--url",
            "http://localhost:1",
            "--username",
            "admin",
            "--password",
            "x",
            "flow",
            "replace",
            "pg-1",
            "/tmp/file-does-not-matter-confirm-fires-first.json",
        ])
        .output()
        .expect("failed to run nifictl");
    assert!(!output.status.success(), "expected non-zero exit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("refusing to run destructive command without --yes"),
        "stderr should contain refusal message: {stderr}"
    );
}

/// `flow replace --dry-run --stop-first` must print stop + GET + PUT + start
/// in order and exit 0 without hitting the server.
#[test]
fn dry_run_on_flow_replace_stop_first_prints_all_four_requests() {
    let output = nifictl()
        .args([
            "--url",
            "http://localhost:1",
            "--username",
            "admin",
            "--password",
            "x",
            "--dry-run",
            "--yes",
            "flow",
            "replace",
            "pg-1",
            "/tmp/no-read-in-dry-run.json",
            "--stop-first",
        ])
        .output()
        .expect("failed to run nifictl");
    assert!(
        output.status.success(),
        "dry-run should exit 0; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stop_idx = stdout
        .find("\"state\": \"STOPPED\"")
        .expect("should contain STOPPED body");
    let get_idx = stdout
        .find("GET http://localhost:1/nifi-api/process-groups/pg-1")
        .expect("should contain GET line");
    let put_idx = stdout
        .find("PUT http://localhost:1/nifi-api/process-groups/pg-1/flow-contents")
        .expect("should contain PUT replace");
    let start_idx = stdout
        .find("\"state\": \"RUNNING\"")
        .expect("should contain RUNNING body");
    assert!(
        stop_idx < get_idx && get_idx < put_idx && put_idx < start_idx,
        "expected order stop \u{2192} get \u{2192} put \u{2192} start; stdout={stdout}"
    );
}

/// `login` with a password-auth context and no password source, in a
/// non-TTY process, must refuse with a clear error — not hang on a
/// hidden prompt.
#[test]
fn login_password_auth_non_tty_no_password_refuses() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    std::fs::write(
        &config_path,
        r#"
current_context = "dev"

[[contexts]]
name = "dev"
url = "http://localhost:1"

[contexts.auth]
type = "password"
username = "admin"
"#,
    )
    .unwrap();

    let output = nifictl()
        .args(["--config", &config_path.to_string_lossy(), "login"])
        // Unset NIFI_PASSWORD so the clap env fallback doesn't populate --password.
        .env_remove("NIFI_PASSWORD")
        .output()
        .expect("failed to run nifictl");

    assert!(!output.status.success(), "expected non-zero exit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no password available and stdin is not a TTY"),
        "stderr should mention non-TTY: {stderr}"
    );
    assert!(
        stderr.contains("set NIFI_PASSWORD or pass --password"),
        "stderr should include hint: {stderr}"
    );
}

/// The existing context-with-password-env error message still fires
/// when the env var is explicitly referenced but missing — the
/// interactive-prompt code path only engages when NO password source
/// is configured at all.
#[test]
fn login_with_missing_password_env_still_errors_cleanly() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    std::fs::write(
        &config_path,
        r#"
current_context = "dev"

[[contexts]]
name = "dev"
url = "http://localhost:1"

[contexts.auth]
type = "password"
username = "admin"
password_env = "DEFINITELY_NOT_SET_PW"
"#,
    )
    .unwrap();

    let output = nifictl()
        .args(["--config", &config_path.to_string_lossy(), "login"])
        .env_remove("DEFINITELY_NOT_SET_PW")
        .output()
        .expect("failed to run nifictl");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("DEFINITELY_NOT_SET_PW"),
        "stderr should name the missing env var: {stderr}"
    );
}

/// `flow --help` should list the three porcelain verbs alongside the
/// generated flow subcommands.
#[test]
fn flow_help_lists_porcelain_and_generated() {
    let output = nifictl()
        .args(["flow", "--help"])
        .output()
        .expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in ["export", "import", "replace", "get-about-info"] {
        assert!(
            stdout.contains(expected),
            "flow --help missing '{expected}': {stdout}"
        );
    }
}

/// 401 from the server must render the two-line error+hint rendering:
/// `error: ...\nhint: run 'nifictl login'`.
#[tokio::test]
async fn hint_fires_on_401() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock = MockServer::start().await;
    // /flow/about is the version-detect probe — answer with 401 so the
    // token-auth status flow surfaces NifiError::Unauthorized, which
    // maps to the login-hint.
    Mock::given(method("GET"))
        .and(path("/nifi-api/flow/about"))
        .respond_with(ResponseTemplate::new(401).set_body_string("unauthenticated"))
        .mount(&mock)
        .await;

    // status uses an existing token (we provide --token). build_client's
    // token path calls detect_version() which hits GET /flow/about.
    let output = nifictl()
        .args([
            "--url",
            &mock.uri(),
            "--token",
            "dummy",
            "status",
        ])
        .output()
        .expect("failed to run nifictl");

    assert!(!output.status.success(), "expected non-zero exit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error: "),
        "stderr should start with 'error: ': {stderr}"
    );
    assert!(
        stderr.contains("hint: run 'nifictl login'"),
        "stderr should include the login hint: {stderr}"
    );
}

/// A TLS handshake failure (https URL to a plain-HTTP wiremock) must
/// render the `--insecure` hint.
#[tokio::test]
async fn tls_error_includes_insecure_hint() {
    use wiremock::MockServer;

    let mock = MockServer::start().await;
    // wiremock listens on plain HTTP. Connecting with https:// makes
    // reqwest/rustls fail the handshake.
    let plain_uri = mock.uri();
    let https_uri = plain_uri.replacen("http://", "https://", 1);

    let output = nifictl()
        .args([
            "--url",
            &https_uri,
            "--token",
            "dummy",
            "status",
        ])
        .output()
        .expect("failed to run nifictl");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("hint: pass --insecure for dev environments only"),
        "stderr should include the --insecure hint: {stderr}"
    );
}

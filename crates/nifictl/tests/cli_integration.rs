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

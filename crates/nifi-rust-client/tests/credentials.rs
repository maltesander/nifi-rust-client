#![cfg(not(feature = "dynamic"))]
#![allow(clippy::panic)]
use nifi_rust_client::CredentialProvider;
use nifi_rust_client::credentials::{EnvCredentials, StaticCredentials};

#[tokio::test]
async fn static_credentials_returns_fixed_values() {
    let creds = StaticCredentials::new("admin", "secret");
    let (user, pass) = creds.credentials().await.unwrap();
    assert_eq!(user, "admin");
    assert_eq!(pass, "secret");
}

#[tokio::test]
async fn env_credentials_reads_from_env() {
    let u_var = "TEST_CRED_USER_1";
    let p_var = "TEST_CRED_PASS_1";
    // SAFETY: test runs in isolation; no other thread reads these variables.
    unsafe {
        std::env::set_var(u_var, "env_user");
        std::env::set_var(p_var, "env_pass");
    }
    let creds = EnvCredentials::with_vars(u_var, p_var);
    let (user, pass) = creds.credentials().await.unwrap();
    assert_eq!(user, "env_user");
    assert_eq!(pass, "env_pass");
    // SAFETY: test cleanup; no other thread reads these variables.
    unsafe {
        std::env::remove_var(u_var);
        std::env::remove_var(p_var);
    }
}

#[tokio::test]
async fn env_credentials_returns_error_when_var_missing() {
    let creds =
        EnvCredentials::with_vars("NONEXISTENT_CRED_USER_XYZZY", "NONEXISTENT_CRED_PASS_XYZZY");
    let result = creds.credentials().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn env_credentials_default_uses_nifi_vars() {
    let creds = EnvCredentials::new();
    let creds_missing = EnvCredentials::with_vars(
        "DEFINITELY_NOT_SET_CRED_USER",
        "DEFINITELY_NOT_SET_CRED_PASS",
    );
    assert!(creds_missing.credentials().await.is_err());
    assert_eq!(creds.username_var(), "NIFI_USERNAME");
    assert_eq!(creds.password_var(), "NIFI_PASSWORD");
}

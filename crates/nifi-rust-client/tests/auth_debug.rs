#![cfg(not(feature = "dynamic"))]

use nifi_rust_client::config::auth::{PasswordAuth, StaticTokenAuth};

#[test]
fn password_auth_debug_masks_password() {
    let auth = PasswordAuth::new("alice", "hunter2");
    let rendered = format!("{auth:?}");
    assert!(
        rendered.contains("alice"),
        "username should appear in debug output: {rendered}"
    );
    assert!(
        !rendered.contains("hunter2"),
        "raw password must not appear in debug output: {rendered}"
    );
    assert!(
        rendered.contains("REDACTED"),
        "debug output should mark the password REDACTED: {rendered}"
    );
}

#[test]
fn static_token_auth_debug_masks_token() {
    let auth = StaticTokenAuth::new("super-secret-jwt");
    let rendered = format!("{auth:?}");
    assert!(
        !rendered.contains("super-secret-jwt"),
        "raw token must not appear in debug output: {rendered}"
    );
    assert!(
        rendered.contains("REDACTED"),
        "debug output should mark the token REDACTED: {rendered}"
    );
}

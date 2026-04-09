#![cfg(not(feature = "dynamic"))]
/// Integration tests require a running NiFi instance.
/// Use test/run.sh to start one, or set env vars manually:
///   NIFI_URL          (default: https://localhost:8443)
///   NIFI_USERNAME     (default: admin)
///   NIFI_PASSWORD     (default: adminpassword123)
///   NIFI_CA_CERT_PATH (optional: path to PEM CA cert for TLS verification)
fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use test/run.sh)"]
async fn login_and_connect_insecure() {
    let client = nifi_rust_client::NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in to NiFi");

    client
        .flow_api()
        .get_about_info()
        .await
        .expect("failed to reach NiFi");
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use test/run.sh)"]
async fn login_and_connect_with_ca_cert() {
    let cert_path = match std::env::var("NIFI_CA_CERT_PATH") {
        Ok(p) => p,
        Err(_) => {
            println!("NIFI_CA_CERT_PATH not set, skipping CA cert test");
            return;
        }
    };

    let ca_cert = std::fs::read(&cert_path)
        .unwrap_or_else(|e| panic!("failed to read CA cert from {cert_path}: {e}"));

    let client = nifi_rust_client::NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .add_root_certificate(&ca_cert)
        .build()
        .expect("failed to build NiFi client with CA cert");

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in to NiFi");

    client
        .flow_api()
        .get_about_info()
        .await
        .expect("failed to reach NiFi");
}

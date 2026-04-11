use crate::parser::ApiSpec;

/// Generates wiremock tests for the dynamic client.
/// Returns the content for `tests/dynamic_tests.rs`.
///
/// `specs` is `[(version_str, mod_name, spec)]`.
pub fn emit_dynamic_tests(specs: &[(&str, &str, &ApiSpec)]) -> String {
    let mut out = String::new();
    out.push_str("// @generated — do not edit; run `cargo run -p nifi-openapi-gen`\n\n");
    out.push_str("#![cfg(feature = \"dynamic\")]\n\n");
    out.push_str("use wiremock::{MockServer, Mock, ResponseTemplate};\n");
    out.push_str("use wiremock::matchers::{method, path};\n");
    out.push_str("use nifi_rust_client::NifiClientBuilder;\n\n");

    // One test per version: mock about endpoint, build_dynamic, verify detection
    for (version, _mod_name, _spec) in specs {
        let test_name = format!("auto_detects_v{}", version.replace('.', "_"));
        out.push_str("#[tokio::test]\n");
        out.push_str(&format!("async fn {test_name}() {{\n"));
        out.push_str("    let mock = MockServer::start().await;\n");
        out.push_str("    Mock::given(method(\"GET\"))\n");
        out.push_str("        .and(path(\"/nifi-api/flow/about\"))\n");
        out.push_str(
            "        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({\n",
        );
        out.push_str(&format!(
            "            \"about\": {{ \"title\": \"NiFi\", \"version\": \"{version}\" }}\n"
        ));
        out.push_str("        })))\n");
        out.push_str("        .mount(&mock).await;\n\n");
        out.push_str("    let client = NifiClientBuilder::new(&mock.uri()).unwrap()\n");
        out.push_str("        .build().unwrap();\n");
        out.push_str("    let dynamic = nifi_rust_client::dynamic::DynamicClient::from_client(client).await.unwrap();\n");
        out.push_str(&format!(
            "    assert_eq!(dynamic.detected_version().unwrap().to_string(), \"{version}\");\n"
        ));
        out.push_str("}\n\n");
    }

    // Unknown version test
    out.push_str("#[tokio::test]\n");
    out.push_str("async fn unknown_version_returns_error() {\n");
    out.push_str("    let mock = MockServer::start().await;\n");
    out.push_str("    Mock::given(method(\"GET\"))\n");
    out.push_str("        .and(path(\"/nifi-api/flow/about\"))\n");
    out.push_str(
        "        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({\n",
    );
    out.push_str("            \"about\": { \"title\": \"NiFi\", \"version\": \"1.15.0\" }\n");
    out.push_str("        })))\n");
    out.push_str("        .mount(&mock).await;\n\n");
    out.push_str("    let client = NifiClientBuilder::new(&mock.uri()).unwrap()\n");
    out.push_str("        .build().unwrap();\n");
    out.push_str(
        "    let result = nifi_rust_client::dynamic::DynamicClient::from_client(client).await;\n",
    );
    out.push_str("    assert!(result.is_err());\n");
    out.push_str("    let err = result.unwrap_err();\n");
    out.push_str("    assert!(err.to_string().contains(\"1.15.0\"));\n");
    out.push_str("}\n");

    out
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::*;
    use crate::parser::{ApiSpec, TagGroup};

    #[test]
    fn test_generates_version_detection_test_per_version() {
        let spec = ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "FlowApi".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow_api".to_string(),
                types: vec![],
                root_endpoints: vec![],
                sub_groups: vec![],
            }],
            all_types: vec![],
        };
        let output = emit_dynamic_tests(&[("2.7.2", "v2_7_2", &spec), ("2.8.0", "v2_8_0", &spec)]);
        assert!(output.contains("#![cfg(feature = \"dynamic\")]"));
        assert!(output.contains("auto_detects_v2_7_2"));
        assert!(output.contains("auto_detects_v2_8_0"));
        assert!(output.contains("unknown_version_returns_error"));
    }
}

use std::collections::BTreeMap;

use crate::parser::{ApiSpec, Endpoint};

pub fn format_source(src: &str) -> String {
    match syn::parse_file(src) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => src.to_string(),
    }
}

pub(crate) fn version_to_variant(version: &str) -> String {
    format!("V{}", version.replace('.', "_"))
}

/// Info about an endpoint in the context of a specific version.
pub(crate) struct EndpointInfo<'a> {
    pub endpoint: &'a Endpoint,
    /// None for root endpoints, Some(struct_name) for sub-group endpoints.
    pub sub_struct_name: Option<&'a str>,
    /// The field name for the primary param in the sub-struct (e.g. "id", "port_id").
    pub primary_param: Option<&'a str>,
}

/// Collect all unique (tag, struct_name, module_name, accessor_fn) across versions.
pub(crate) fn collect_all_tags(
    versions: &[(&str, &str, &str, &ApiSpec)],
) -> Vec<(String, String, String, String)> {
    let mut seen: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    for (_, _, _, spec) in versions {
        for tag in &spec.tags {
            seen.entry(tag.tag.clone()).or_insert_with(|| {
                (
                    tag.struct_name.clone(),
                    tag.module_name.clone(),
                    tag.accessor_fn.clone(),
                )
            });
        }
    }
    seen.into_iter()
        .map(|(tag, (sn, mn, af))| (tag, sn, mn, af))
        .collect()
}

/// For a given tag, collect all endpoints (including sub-group endpoints flattened) across versions.
/// Returns BTreeMap<fn_name, BTreeMap<version_str, EndpointInfo>>
pub(crate) fn collect_tag_endpoints<'a>(
    versions: &[(&'a str, &'a str, &'a str, &'a ApiSpec)],
    tag: &str,
) -> BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> {
    let mut result: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> = BTreeMap::new();

    for (ver, _mod_name, _feature, spec) in versions {
        for tg in &spec.tags {
            if tg.tag != tag {
                continue;
            }
            for ep in &tg.root_endpoints {
                result.entry(ep.fn_name.clone()).or_default().insert(
                    ver,
                    EndpointInfo {
                        endpoint: ep,
                        sub_struct_name: None,
                        primary_param: None,
                    },
                );
            }
            // Flatten sub-group endpoints
            for sg in &tg.sub_groups {
                for ep in &sg.endpoints {
                    result.entry(ep.fn_name.clone()).or_default().insert(
                        ver,
                        EndpointInfo {
                            endpoint: ep,
                            sub_struct_name: Some(&sg.struct_name),
                            primary_param: Some(&sg.primary_param),
                        },
                    );
                }
            }
        }
    }

    result
}

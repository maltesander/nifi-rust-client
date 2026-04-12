//! Method name stability for generated resource structs.
//!
//! This module runs as a post-pass after the parser has built per-version
//! `ApiSpec`s. It:
//!
//! 1. Applies `(version, operationId)` overrides from
//!    [`crate::naming_overrides`] to rewrite specific method names.
//! 2. Panics at build time if any two operations in the same tag would
//!    generate the same method name (same-tag collision — implemented in
//!    Task 5).
//! 3. Panics at build time if the same `(tag, method, path)` triple
//!    resolves to different method names in different versions
//!    (cross-version drift — implemented in Task 6).
//!
//! All three checks are strict — there are no ignore flags. The panics are
//! actionable and point at the exact override-table entry that would fix
//! each case.

use std::collections::HashMap;

use crate::naming_overrides::NAMING_OVERRIDES;
use crate::parser::{ApiSpec, Endpoint};

/// Apply override entries from the production `NAMING_OVERRIDES` table to a
/// single parsed spec, then run the per-version same-tag collision check.
///
/// This is the entry point used by the real generator. Tests that need to
/// inject a synthetic override map should call
/// [`apply_overrides_with_table`] directly.
pub fn apply_overrides_and_check_single(spec: &mut ApiSpec, version: &str) {
    let table: HashMap<(String, String), String> = NAMING_OVERRIDES
        .iter()
        .map(|((v, op), name)| ((v.to_string(), op.to_string()), name.to_string()))
        .collect();
    apply_overrides_with_table(spec, version, &table);
    check_collisions(spec, version);
}

/// Public test hook: apply overrides from a caller-supplied table.
pub fn apply_overrides_with_table(
    spec: &mut ApiSpec,
    version: &str,
    overrides: &HashMap<(String, String), String>,
) {
    for tag in &mut spec.tags {
        for ep in &mut tag.root_endpoints {
            maybe_override(ep, version, overrides);
        }
        for sg in &mut tag.sub_groups {
            for ep in &mut sg.endpoints {
                maybe_override(ep, version, overrides);
            }
        }
    }
}

fn maybe_override(
    ep: &mut Endpoint,
    version: &str,
    overrides: &HashMap<(String, String), String>,
) {
    let key = (version.to_string(), ep.raw_operation_id.clone());
    if let Some(name) = overrides.get(&key) {
        ep.fn_name = name.clone();
    }
}

/// One endpoint observation for the collision check:
/// `(tag, fn_name, method, path, raw_operation_id)`.
type EndpointEntry<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str);

/// One collision-group member: `(method, path, raw_operation_id)`.
type CollisionMember<'a> = (&'a str, &'a str, &'a str);

/// Panic if any two operations in the same tag would generate the same
/// Rust method name. Each resource struct can only expose one method per
/// name, so a collision here would be a compile error further downstream
/// — this check surfaces it with a much better error message.
pub fn check_collisions(spec: &ApiSpec, version: &str) {
    // Flat list of (tag, fn_name, method, path, raw_op_id) for every
    // endpoint (root + subgroups).
    let mut entries: Vec<EndpointEntry<'_>> = Vec::new();
    for tag in &spec.tags {
        for ep in &tag.root_endpoints {
            entries.push((
                tag.tag.as_str(),
                ep.fn_name.as_str(),
                ep.method.as_str(),
                ep.path.as_str(),
                ep.raw_operation_id.as_str(),
            ));
        }
        for sg in &tag.sub_groups {
            for ep in &sg.endpoints {
                entries.push((
                    tag.tag.as_str(),
                    ep.fn_name.as_str(),
                    ep.method.as_str(),
                    ep.path.as_str(),
                    ep.raw_operation_id.as_str(),
                ));
            }
        }
    }

    // Group by (tag, fn_name). Any group with len > 1 is a collision.
    let mut groups: HashMap<(&str, &str), Vec<CollisionMember<'_>>> = HashMap::new();
    for (tag, fn_name, method, path, op_id) in entries {
        groups
            .entry((tag, fn_name))
            .or_default()
            .push((method, path, op_id));
    }

    for ((tag, fn_name), members) in groups.iter() {
        if members.len() > 1 {
            panic!("{}", collision_message(version, tag, fn_name, members));
        }
    }
}

fn collision_message(
    version: &str,
    tag: &str,
    fn_name: &str,
    members: &[CollisionMember<'_>],
) -> String {
    let mut out = format!(
        "nifi-openapi-gen: fn_name collision in NiFi {version}\n\n\
         Two operations in tag \"{tag}\" would generate the same Rust method \
         name `{fn_name}`:\n\n"
    );
    for (method, path, op_id) in members {
        out.push_str(&format!(
            "  {method} {path}           (operationId: {op_id})\n"
        ));
    }
    out.push_str(&format!(
        "\nEach resource struct can only expose one `{fn_name}` method. Resolve \
         by adding an entry to crates/nifi-openapi-gen/src/naming_overrides.rs:\n\n\
         \x20   m.insert(\n\
         \x20       (\"{version}\", \"<raw_operationId>\"),\n\
         \x20       \"<explicit_fn_name>\",\n\
         \x20   );\n\n\
         Pick a name that describes what the endpoint actually does. The \
         override is keyed by (version, operationId) so older versions keep \
         their existing names.\n"
    ));
    out
}

/// Drift-check map key: `(tag, method, path)`.
type DriftKey = (String, String, String);

/// Drift-check map value entry: `(version, fn_name, raw_operation_id)`.
type DriftEntry = (String, String, String);

/// Panic if the same `(tag, method, path)` triple resolves to different
/// `fn_name`s in different versions. Strict across the entire supported
/// version set — dynamic mode requires one canonical name per endpoint
/// and does not allow intra-major drift.
pub fn check_drift(all_parsed: &[(String, ApiSpec)]) {
    // Map (tag, method, path) -> Vec<(version, fn_name, raw_op_id)>
    let mut seen: HashMap<DriftKey, Vec<DriftEntry>> = HashMap::new();

    for (version, spec) in all_parsed {
        for tag in &spec.tags {
            for ep in &tag.root_endpoints {
                insert_drift_entry(&mut seen, version, &tag.tag, ep);
            }
            for sg in &tag.sub_groups {
                for ep in &sg.endpoints {
                    insert_drift_entry(&mut seen, version, &tag.tag, ep);
                }
            }
        }
    }

    for ((tag, method, path), versions) in seen.iter() {
        let first_name = &versions[0].1;
        if versions.iter().any(|(_, name, _)| name != first_name) {
            panic!("{}", drift_message(tag, method, path, versions));
        }
    }
}

fn insert_drift_entry(
    map: &mut HashMap<DriftKey, Vec<DriftEntry>>,
    version: &str,
    tag: &str,
    ep: &Endpoint,
) {
    map.entry((tag.to_string(), ep.method.as_str().to_string(), ep.path.clone()))
        .or_default()
        .push((
            version.to_string(),
            ep.fn_name.clone(),
            ep.raw_operation_id.clone(),
        ));
}

fn drift_message(tag: &str, method: &str, path: &str, versions: &[DriftEntry]) -> String {
    let mut out = format!(
        "nifi-openapi-gen: fn_name drift across versions\n\n\
         The same endpoint (tag \"{tag}\") resolves to different Rust method \
         names in different versions:\n\n\
         \x20 {method} {path}\n\n"
    );
    for (version, fn_name, op_id) in versions {
        out.push_str(&format!(
            "    {version}: {fn_name}   (operationId: {op_id})\n"
        ));
    }
    let (latest_version, latest_name, latest_op_id) =
        versions.last().expect("at least one version");
    out.push_str(&format!(
        "\nDynamic mode requires one canonical name per endpoint. Either:\n\
         \x20 1. Pin the older name by overriding {latest_version}'s operationId:\n\
         \x20      m.insert((\"{latest_version}\", \"{latest_op_id}\"), \"<older_fn_name>\");\n\
         \x20 2. Accept the rename: update the golden at \
         specs/{latest_version}/fn_names.txt and note the breaking change \
         in NIFI_API_CHANGES.md. Current new name: {latest_name}.\n"
    ));
    out
}

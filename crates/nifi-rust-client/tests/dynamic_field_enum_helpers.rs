//! Smoke tests for generated dynamic-mode field-level enum helpers.
//!
//! Validates that:
//! - `as_str` produces the expected wire value
//! - `from_wire` round-trips the wire value back into the typed variant
//! - `Display` and `From<T> for String` agree with `as_str`
//! - the parent DTO field is still `Option<String>` (wire-tolerant)

#![cfg(feature = "dynamic")]

use nifi_rust_client::dynamic::types::{
    ActivateControllerServicesEntityState, ScheduleComponentsEntity,
    ScheduleComponentsEntityState,
};

#[test]
fn schedule_components_entity_state_round_trips() {
    let v = ScheduleComponentsEntityState::Running;
    assert_eq!(v.as_str(), "RUNNING");
    assert_eq!(format!("{v}"), "RUNNING");
    let s: String = v.into();
    assert_eq!(s, "RUNNING");
    assert_eq!(
        ScheduleComponentsEntityState::from_wire("RUNNING"),
        Some(ScheduleComponentsEntityState::Running)
    );
    assert_eq!(ScheduleComponentsEntityState::from_wire("UNKNOWN_FUTURE"), None);
}

#[test]
fn activate_controller_services_state_round_trips() {
    let v = ActivateControllerServicesEntityState::Enabled;
    assert_eq!(v.as_str(), "ENABLED");
    assert_eq!(
        ActivateControllerServicesEntityState::from_wire("DISABLED"),
        Some(ActivateControllerServicesEntityState::Disabled)
    );
}

#[test]
fn dto_field_remains_option_string_so_unknown_values_deserialize() {
    // The DTO field stays Option<String> — proven by deserializing
    // with a future server-only string the helper enum doesn't know about.
    let json = serde_json::json!({
        "id": "pg-1",
        "state": "QUARANTINED"  // not in any spec version
    });
    let entity: ScheduleComponentsEntity = serde_json::from_value(json)
        .expect("deserialization failed");
    assert_eq!(entity.state.as_deref(), Some("QUARANTINED"));
    // The helper returns None on unknown — caller decides whether to surface
    // the raw string or treat as an error.
    assert_eq!(
        entity
            .state
            .as_deref()
            .and_then(ScheduleComponentsEntityState::from_wire),
        None
    );
}

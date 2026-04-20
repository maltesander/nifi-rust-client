#![deny(missing_docs)]
//! One-shot bulk-control helpers for process groups.
//!
//! These functions wrap NiFi's native bulk-control endpoints and return
//! once the HTTP call resolves. They do NOT poll for the resulting state
//! transition — pair them with the [`crate::wait`] helpers when you need
//! that.

use crate::NifiError;

// ── bulk::{start,stop}_process_group ───────────────────────────────────────

#[cfg(not(feature = "dynamic"))]
use crate::types::{
    ActivateControllerServicesEntity, ActivateControllerServicesEntityState,
    ScheduleComponentsEntity, ScheduleComponentsEntityState,
};

/// Start (schedule) all authorized processors in the given process group.
///
/// Wraps `PUT /flow/process-groups/{id}` with `{ id, state: "RUNNING" }`.
#[cfg(not(feature = "dynamic"))]
pub async fn start_process_group(
    client: &crate::NifiClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some(ScheduleComponentsEntityState::Running),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

/// Stop (unschedule) all processors in the given process group.
#[cfg(not(feature = "dynamic"))]
pub async fn stop_process_group(
    client: &crate::NifiClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some(ScheduleComponentsEntityState::Stopped),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

// ── bulk::{enable,disable}_all_controller_services ─────────────────────────

/// Enable all authorized controller services in the given process group.
///
/// Wraps `PUT /flow/process-groups/{id}/controller-services` with
/// `{ id, state: "ENABLED" }`.
#[cfg(not(feature = "dynamic"))]
pub async fn enable_all_controller_services(
    client: &crate::NifiClient,
    group_id: &str,
) -> Result<ActivateControllerServicesEntity, NifiError> {
    let body = ActivateControllerServicesEntity {
        id: Some(group_id.to_string()),
        state: Some(ActivateControllerServicesEntityState::Enabled),
        ..Default::default()
    };
    client
        .flow()
        .activate_controller_services(group_id, &body)
        .await
}

/// Disable all controller services in the given process group.
#[cfg(not(feature = "dynamic"))]
pub async fn disable_all_controller_services(
    client: &crate::NifiClient,
    group_id: &str,
) -> Result<ActivateControllerServicesEntity, NifiError> {
    let body = ActivateControllerServicesEntity {
        id: Some(group_id.to_string()),
        state: Some(ActivateControllerServicesEntityState::Disabled),
        ..Default::default()
    };
    client
        .flow()
        .activate_controller_services(group_id, &body)
        .await
}

#[cfg(feature = "dynamic")]
use crate::dynamic::types::{
    ActivateControllerServicesEntity, ActivateControllerServicesEntityState,
    ScheduleComponentsEntity, ScheduleComponentsEntityState,
};

/// Dynamic-mode counterpart of `start_process_group`.
#[cfg(feature = "dynamic")]
pub async fn start_process_group_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some(ScheduleComponentsEntityState::Running.into()),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

/// Dynamic-mode counterpart of `stop_process_group`.
#[cfg(feature = "dynamic")]
pub async fn stop_process_group_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some(ScheduleComponentsEntityState::Stopped.into()),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

/// Dynamic-mode counterpart of `enable_all_controller_services`.
#[cfg(feature = "dynamic")]
pub async fn enable_all_controller_services_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ActivateControllerServicesEntity, NifiError> {
    let body = ActivateControllerServicesEntity {
        id: Some(group_id.to_string()),
        state: Some(ActivateControllerServicesEntityState::Enabled.into()),
        ..Default::default()
    };
    client
        .flow()
        .activate_controller_services(group_id, &body)
        .await
}

/// Dynamic-mode counterpart of `disable_all_controller_services`.
#[cfg(feature = "dynamic")]
pub async fn disable_all_controller_services_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ActivateControllerServicesEntity, NifiError> {
    let body = ActivateControllerServicesEntity {
        id: Some(group_id.to_string()),
        state: Some(ActivateControllerServicesEntityState::Disabled.into()),
        ..Default::default()
    };
    client
        .flow()
        .activate_controller_services(group_id, &body)
        .await
}

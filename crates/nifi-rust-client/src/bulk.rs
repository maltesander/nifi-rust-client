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
#[allow(unused_imports)]
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

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use crate::dynamic::types::{ActivateControllerServicesEntity, ScheduleComponentsEntity};

/// Dynamic-mode counterpart of [`start_process_group`].
#[cfg(feature = "dynamic")]
pub async fn start_process_group_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some("RUNNING".to_string()),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

/// Dynamic-mode counterpart of [`stop_process_group`].
#[cfg(feature = "dynamic")]
pub async fn stop_process_group_dynamic(
    client: &crate::dynamic::DynamicClient,
    group_id: &str,
) -> Result<ScheduleComponentsEntity, NifiError> {
    let body = ScheduleComponentsEntity {
        id: Some(group_id.to_string()),
        state: Some("STOPPED".to_string()),
        ..Default::default()
    };
    client.flow().schedule_components(group_id, &body).await
}

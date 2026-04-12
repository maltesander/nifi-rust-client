//! Pagination helpers for NiFi REST endpoints.
//!
//! Currently provides [`HistoryPaginator`] for walking
//! `GET /flow/history` one page (or one action) at a time. See
//! [`flow_history`] for the static-mode constructor and
//! [`flow_history_dynamic`] for the dynamic-mode constructor.
//!
//! The paginator is a thin state machine: one HTTP call per page,
//! offset advanced by `page_size`, termination by any of
//! (a) empty page, (b) short page, (c) offset reached `total`.
//! Retries are handled per-request by the underlying [`crate::NifiClient`];
//! the paginator adds no retry logic of its own.

use std::collections::VecDeque;

use crate::error::NifiError;
use crate::types::ActionEntity;

/// Filter criteria for `GET /flow/history`. All fields are optional;
/// `HistoryFilter::default()` yields an unfiltered history query.
///
/// Field names mirror NiFi's query parameters. Date fields are passed
/// through as strings in NiFi's expected format
/// (`MM/dd/yyyy HH:mm:ss`); the crate does not parse or validate them.
#[derive(Default, Debug, Clone)]
pub struct HistoryFilter {
    /// Column to sort by (e.g. `"timestamp"`).
    pub sort_column: Option<String>,
    /// Sort order (`"asc"` or `"desc"`).
    pub sort_order: Option<String>,
    /// Inclusive lower bound on action timestamp.
    pub start_date: Option<String>,
    /// Inclusive upper bound on action timestamp.
    pub end_date: Option<String>,
    /// Filter by user identity.
    pub user_identity: Option<String>,
    /// Filter by source component id.
    pub source_id: Option<String>,
}

/// Shape returned by a page-fetch closure given to [`HistoryPaginator`].
///
/// Exposed so advanced users can drive the paginator with their own
/// closure for endpoints not covered by [`flow_history`] /
/// [`flow_history_dynamic`].
#[derive(Debug, Clone)]
pub struct HistoryPage {
    /// Actions returned by the server for this page.
    pub actions: Vec<ActionEntity>,
    /// Server-reported total number of actions matching the filter.
    pub total: i32,
}

/// Async iterator over pages of NiFi flow history actions.
///
/// Created via [`flow_history`] or [`flow_history_dynamic`]. Each call
/// to [`next_page`](Self::next_page) issues one `GET /flow/history`
/// request advancing `offset` by `page_size`. [`next`](Self::next)
/// yields one item at a time, buffering the current page internally.
pub struct HistoryPaginator<F> {
    fetch: F,
    page_size: u32,
    offset: u32,
    buffer: VecDeque<ActionEntity>,
    exhausted: bool,
}

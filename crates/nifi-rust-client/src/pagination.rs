//! Pagination helpers for NiFi REST endpoints.
//!
//! Currently provides `HistoryPaginator` for walking
//! `GET /flow/history` one page (or one action) at a time. See
//! `flow_history` for the static-mode constructor and
//! `flow_history_dynamic` (requires `dynamic` feature) for the
//! dynamic-mode constructor.
//!
//! The paginator is a thin state machine: one HTTP call per page,
//! offset advanced by `page_size`, termination by any of
//! (a) empty page, (b) short page, (c) offset reached `total`.
//! Retries are handled per-request by the underlying [`crate::NifiClient`];
//! the paginator adds no retry logic of its own.

use std::collections::VecDeque;

#[cfg(feature = "dynamic")]
use crate::dynamic::types::ActionEntity;
use crate::error::NifiError;
#[cfg(not(feature = "dynamic"))]
use crate::types::ActionEntity;

/// Boxed future returned by the fetch closures in this module.
///
/// Using `Pin<Box<dyn Future>>` sidesteps chained-`impl Trait` lifetime
/// issues in the constructor return types. One allocation per page —
/// negligible against the HTTP round trip.
type BoxedFetchFuture<'a> = std::pin::Pin<
    Box<dyn core::future::Future<Output = Result<HistoryPage, NifiError>> + Send + 'a>,
>;

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
/// closure for endpoints not covered by `flow_history` /
/// `flow_history_dynamic`.
#[derive(Debug, Clone)]
pub struct HistoryPage {
    /// Actions returned by the server for this page.
    pub actions: Vec<ActionEntity>,
    /// Server-reported total number of actions matching the filter.
    pub total: i32,
}

/// Async iterator over pages of NiFi flow history actions.
///
/// Created via `flow_history` or `flow_history_dynamic`. Each call
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

impl<F, Fut> HistoryPaginator<F>
where
    F: FnMut(u32, u32) -> Fut,
    Fut: core::future::Future<Output = Result<HistoryPage, NifiError>>,
{
    /// Construct a paginator directly from a fetch closure.
    ///
    /// Most users call `flow_history` or `flow_history_dynamic`
    /// instead, which build the closure for the NiFi history endpoint.
    /// Advanced callers can use this constructor to paginate their
    /// own endpoints that follow the same offset/count + total shape.
    pub fn from_fetcher(fetch: F, page_size: u32) -> Self {
        Self {
            fetch,
            page_size,
            offset: 0,
            buffer: VecDeque::new(),
            exhausted: false,
        }
    }

    /// Fetch the next page of actions.
    ///
    /// Returns `Ok(None)` once the history is exhausted. Idempotent
    /// after exhaustion — further calls return `Ok(None)` without
    /// issuing a request.
    pub async fn next_page(&mut self) -> Result<Option<Vec<ActionEntity>>, NifiError> {
        if self.exhausted {
            return Ok(None);
        }
        let page = (self.fetch)(self.offset, self.page_size).await?;

        let returned = page.actions.len() as u32;
        self.offset = self.offset.saturating_add(returned);

        if returned == 0
            || returned < self.page_size
            || i64::from(self.offset) >= i64::from(page.total)
        {
            self.exhausted = true;
        }

        if page.actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(page.actions))
        }
    }

    /// Yield the next action, buffering pages internally.
    ///
    /// Returns `Ok(None)` once the history is exhausted. Each
    /// underlying page is fetched lazily on demand via
    /// [`next_page`](Self::next_page).
    pub async fn next(&mut self) -> Result<Option<ActionEntity>, NifiError> {
        loop {
            if let Some(item) = self.buffer.pop_front() {
                return Ok(Some(item));
            }
            match self.next_page().await? {
                Some(page) => self.buffer.extend(page),
                None => return Ok(None),
            }
        }
    }
}

/// Build a [`HistoryPaginator`] backed by a static-mode [`crate::NifiClient`].
///
/// Each page is fetched by calling `client.flow_api().query_history(...)`
/// with the provided `filter` and the current offset/page_size. Missing
/// `actions` or `total` fields in the response surface as
/// [`NifiError::MissingField`] via the [`crate::require!`] macro.
///
/// # Example
///
/// ```no_run
/// use nifi_rust_client::{NifiClientBuilder, NifiError};
/// use nifi_rust_client::pagination::{flow_history, HistoryFilter};
///
/// # async fn example() -> Result<(), NifiError> {
/// let client = NifiClientBuilder::new("https://nifi.example.com:8443")?.build()?;
/// client.login("admin", "adminpassword123").await?;
///
/// let mut pag = flow_history(&client, HistoryFilter::default(), 100);
/// while let Some(page) = pag.next_page().await? {
///     for action in page {
///         println!("{action:?}");
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[cfg(not(feature = "dynamic"))]
pub fn flow_history<'a>(
    client: &'a crate::NifiClient,
    filter: HistoryFilter,
    page_size: u32,
) -> HistoryPaginator<impl FnMut(u32, u32) -> BoxedFetchFuture<'a> + 'a> {
    use crate::require;
    let fetch = move |offset: u32, count: u32| -> BoxedFetchFuture<'a> {
        let filter = filter.clone();
        Box::pin(async move {
            let offset_s = offset.to_string();
            let count_s = count.to_string();
            let resp = client
                .flow_api()
                .query_history(
                    &offset_s,
                    &count_s,
                    filter.sort_column.as_deref(),
                    filter.sort_order.as_deref(),
                    filter.start_date.as_deref(),
                    filter.end_date.as_deref(),
                    filter.user_identity.as_deref(),
                    filter.source_id.as_deref(),
                )
                .await?;
            let actions = require!(resp.actions).clone();
            let total = *require!(resp.total);
            Ok(HistoryPage { actions, total })
        })
    };
    HistoryPaginator::from_fetcher(fetch, page_size)
}

/// Build a [`HistoryPaginator`] backed by a dynamic-mode
/// [`crate::dynamic::DynamicClient`].
///
/// Same iteration semantics as `flow_history`. Missing `actions` /
/// `total` fields in the response surface as
/// [`NifiError::MissingField`] via the [`crate::require!`] macro.
#[cfg(feature = "dynamic")]
pub fn flow_history_dynamic<'a>(
    client: &'a crate::dynamic::DynamicClient,
    filter: HistoryFilter,
    page_size: u32,
) -> HistoryPaginator<impl FnMut(u32, u32) -> BoxedFetchFuture<'a> + 'a> {
    use crate::dynamic::traits::FlowApi as _;
    use crate::require;
    let fetch = move |offset: u32, count: u32| -> BoxedFetchFuture<'a> {
        let filter = filter.clone();
        Box::pin(async move {
            let offset_s = offset.to_string();
            let count_s = count.to_string();
            let resp = client
                .flow_api()
                .query_history(
                    &offset_s,
                    &count_s,
                    filter.sort_column.as_deref(),
                    filter.sort_order.as_deref(),
                    filter.start_date.as_deref(),
                    filter.end_date.as_deref(),
                    filter.user_identity.as_deref(),
                    filter.source_id.as_deref(),
                )
                .await?;
            let actions = require!(resp.actions).clone();
            let total = *require!(resp.total);
            Ok(HistoryPage { actions, total })
        })
    };
    HistoryPaginator::from_fetcher(fetch, page_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an `ActionEntity` with a given id for identity in tests.
    fn make_action(id: i32) -> ActionEntity {
        ActionEntity {
            id: Some(id),
            ..ActionEntity::default()
        }
    }

    /// Build a fake fetcher that serves up to `total` actions with ids
    /// `0..total`, honoring `offset` and `count` arguments. Records
    /// the number of invocations in the returned `Arc<AtomicUsize>`.
    fn fake_fetcher(
        total: i32,
    ) -> (
        impl FnMut(u32, u32) -> BoxedFetchFuture<'static>,
        std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) {
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let calls_clone = std::sync::Arc::clone(&calls);
        let fetch = move |offset: u32, count: u32| {
            calls_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let start = offset as i32;
            let end = core::cmp::min(start.saturating_add(count as i32), total);
            let actions: Vec<ActionEntity> = if start >= total {
                Vec::new()
            } else {
                (start..end).map(make_action).collect()
            };
            let page = HistoryPage { actions, total };
            Box::pin(async move { Ok(page) })
                as std::pin::Pin<
                    Box<dyn core::future::Future<Output = Result<HistoryPage, NifiError>> + Send>,
                >
        };
        (fetch, calls)
    }

    #[tokio::test]
    async fn next_page_walks_all_pages_then_returns_none() {
        let (fetch, calls) = fake_fetcher(250);
        let mut pag = HistoryPaginator::from_fetcher(fetch, 100);

        let p1 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p1.len(), 100);
        let p2 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p2.len(), 100);
        let p3 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p3.len(), 50);
        assert!(pag.next_page().await.unwrap().is_none());
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn next_page_short_page_terminates() {
        let (fetch, calls) = fake_fetcher(150);
        let mut pag = HistoryPaginator::from_fetcher(fetch, 100);

        let p1 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p1.len(), 100);
        let p2 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p2.len(), 50);
        assert!(pag.next_page().await.unwrap().is_none());
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn next_page_empty_first_response_returns_none() {
        let (fetch, calls) = fake_fetcher(0);
        let mut pag = HistoryPaginator::from_fetcher(fetch, 100);

        assert!(pag.next_page().await.unwrap().is_none());
        assert!(pag.next_page().await.unwrap().is_none());
        // Second call after exhaustion must not re-invoke the fetcher.
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn next_page_is_idempotent_after_exhaustion() {
        let (fetch, calls) = fake_fetcher(50);
        let mut pag = HistoryPaginator::from_fetcher(fetch, 100);

        let p1 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p1.len(), 50);
        assert!(pag.next_page().await.unwrap().is_none());
        assert!(pag.next_page().await.unwrap().is_none());
        assert!(pag.next_page().await.unwrap().is_none());
        assert_eq!(
            calls.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "fetcher must not be called after exhaustion"
        );
    }

    #[tokio::test]
    async fn next_page_does_not_advance_on_error() {
        // Fake fetcher that fails on call 2, then recovers.
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let calls_clone = std::sync::Arc::clone(&calls);
        let fetch = move |offset: u32, count: u32| {
            let n = calls_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let actions: Vec<ActionEntity> = (offset..offset + count)
                .map(|i| make_action(i as i32))
                .collect();
            let fail = n == 1;
            Box::pin(async move {
                if fail {
                    Err(NifiError::Unauthorized {
                        message: "simulated".to_string(),
                    })
                } else {
                    Ok(HistoryPage {
                        actions,
                        total: 300,
                    })
                }
            })
                as std::pin::Pin<
                    Box<dyn core::future::Future<Output = Result<HistoryPage, NifiError>> + Send>,
                >
        };
        let mut pag = HistoryPaginator::from_fetcher(fetch, 100);

        let p1 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p1.len(), 100);
        assert!(pag.next_page().await.is_err());
        // After the error, offset must still be 100 (not 200).
        let p2 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p2.first().and_then(|a| a.id), Some(100));
    }

    #[tokio::test]
    async fn next_page_offset_overflow_saturates() {
        // Simulate `total = i32::MAX` with a fetcher that always returns
        // a full page. The paginator must eventually terminate via
        // saturation of the offset + i64-widened comparison.
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let calls_clone = std::sync::Arc::clone(&calls);
        let count = 100_000_u32;
        let fetch = move |offset: u32, _count: u32| {
            calls_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let actions: Vec<ActionEntity> = (0..count)
                .map(|i| make_action((offset as i32).wrapping_add(i as i32)))
                .collect();
            Box::pin(async move {
                Ok(HistoryPage {
                    actions,
                    total: i32::MAX,
                })
            })
                as std::pin::Pin<
                    Box<dyn core::future::Future<Output = Result<HistoryPage, NifiError>> + Send>,
                >
        };
        let mut pag = HistoryPaginator::from_fetcher(fetch, count);
        // Walk a bounded number of pages; the paginator must terminate
        // naturally via offset >= total (i64 comparison). Guard with a
        // hard cap so an infinite loop bug fails the test quickly.
        let mut pages = 0_usize;
        while pag.next_page().await.unwrap().is_some() {
            pages += 1;
            assert!(pages < 25_000, "paginator failed to terminate");
        }
        // Not asserting an exact page count — only that it terminated.
    }

    #[tokio::test]
    async fn item_next_buffers_pages_and_yields_all() {
        let (fetch, calls) = fake_fetcher(5);
        let mut pag = HistoryPaginator::from_fetcher(fetch, 2);

        let mut ids = Vec::new();
        while let Some(action) = pag.next().await.unwrap() {
            ids.push(action.id.unwrap());
        }
        assert_eq!(ids, vec![0, 1, 2, 3, 4]);
        // 5 items at page_size=2 → pages of 2/2/1 → 3 fetches.
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 3);
    }
}

#![deny(missing_docs)]
use std::sync::Arc;

use reqwest::header::{CONTENT_TYPE, HeaderName};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use snafu::ResultExt as _;
use tokio::sync::RwLock;
use url::Url;

use crate::NifiError;
use crate::config::auth::AuthProvider;
use crate::error::{AuthSnafu, HttpSnafu};

/// `application/octet-stream` MIME type for binary upload bodies.
const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

/// HTTP header NiFi reads behind a trusted proxy to identify the end user.
/// Stored lowercased because [`HeaderName::from_static`] requires it; HTTP
/// header names are case-insensitive on the wire.
const PROXIED_ENTITIES_CHAIN: HeaderName = HeaderName::from_static("x-proxiedentitieschain");

/// Internal handle to the bearer token. Stored as `Arc<Zeroizing<String>>` so
/// that snapshots used by [`NifiClient::with_auth_retry`] are cheap refcount
/// bumps with no plaintext heap copies, and the underlying string is wiped
/// when the last reference drops. `Arc::ptr_eq` distinguishes "same token"
/// from "rotated token" without comparing bytes.
type TokenHandle = Arc<zeroize::Zeroizing<String>>;

/// Client for the Apache NiFi REST API.
pub struct NifiClient {
    base_url: Url,
    http: Client,
    token: Arc<RwLock<Option<TokenHandle>>>,
    auth_provider: Option<Arc<dyn AuthProvider>>,
    proxied_entities_chain: Option<String>,
    retry_policy: Option<crate::config::retry::RetryPolicy>,
    request_id_header: Option<String>,
    auth_lock: Arc<tokio::sync::Mutex<()>>,
}

impl Clone for NifiClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            http: self.http.clone(),
            token: Arc::clone(&self.token),
            auth_provider: self.auth_provider.clone(),
            proxied_entities_chain: self.proxied_entities_chain.clone(),
            retry_policy: self.retry_policy.clone(),
            request_id_header: self.request_id_header.clone(),
            auth_lock: Arc::clone(&self.auth_lock),
        }
    }
}

impl std::fmt::Debug for NifiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NifiClient")
            .field("base_url", &self.base_url)
            .field(
                "auth_provider",
                &self.auth_provider.as_ref().map(|c| format!("{c:?}")),
            )
            .field("proxied_entities_chain", &self.proxied_entities_chain)
            .field("retry_policy", &self.retry_policy)
            .field("request_id_header", &self.request_id_header)
            .finish_non_exhaustive()
    }
}

impl NifiClient {
    /// Construct a client from pre-built parts. Used by [`crate::NifiClientBuilder`].
    pub(crate) fn from_parts(
        base_url: Url,
        http: Client,
        auth_provider: Option<Arc<dyn AuthProvider>>,
        proxied_entities_chain: Option<String>,
        retry_policy: Option<crate::config::retry::RetryPolicy>,
        request_id_header: Option<String>,
    ) -> Self {
        Self {
            base_url,
            http,
            token: Arc::new(RwLock::new(None)),
            auth_provider,
            proxied_entities_chain,
            retry_policy,
            request_id_header,
            auth_lock: Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    /// Return the current bearer token, if one has been set.
    ///
    /// The token is a NiFi-issued JWT. The returned [`zeroize::Zeroizing<String>`]
    /// is a fresh allocation that is wiped from memory when it drops, so the
    /// JWT bytes do not leak through deallocation. Callers that want a plain
    /// `String` can call [`ToString::to_string`] or move out of the wrapper
    /// via `Zeroizing::into_inner`-equivalent dereferencing — at which point
    /// the zeroize-on-drop guarantee is the caller's to keep.
    ///
    /// The in-client copy is independent of the returned value and is zeroized
    /// when cleared (via [`Self::logout`]) or when the client is dropped.
    pub async fn token(&self) -> Option<zeroize::Zeroizing<String>> {
        self.token
            .read()
            .await
            .as_ref()
            .map(|t| zeroize::Zeroizing::new(t.as_str().to_owned()))
    }

    /// Internal: cheap refcount-bump snapshot of the current token handle.
    ///
    /// Used by [`Self::with_auth_retry`] to detect "did the token rotate while
    /// we were in `f`" via [`Arc::ptr_eq`] — no plaintext clone, no equality
    /// comparison over bytes. Token rotation in [`Self::set_token`] /
    /// [`Self::login`] always replaces the cell with a fresh `Arc::new(...)`,
    /// so pointer identity is a sound proxy for "same token instance".
    pub(crate) async fn token_handle(&self) -> Option<TokenHandle> {
        self.token.read().await.as_ref().map(Arc::clone)
    }

    /// Restore a previously obtained bearer token.
    ///
    /// Useful for CLI tools that persist the token in a file between sessions.
    /// Accepts both a plain `String` and a [`zeroize::Zeroizing<String>`] via
    /// [`Into`] — callers that already own a zeroizing wrapper don't have to
    /// unwrap it before handing the token to the client. Plain `String`
    /// callers continue to compile thanks to `From<String> for Zeroizing<String>`.
    ///
    /// If the token has expired, the next API call will return
    /// [`NifiError::Unauthorized`]; re-call [`login`][Self::login]
    /// to obtain a fresh one.
    pub async fn set_token(&self, token: impl Into<zeroize::Zeroizing<String>>) {
        let zeroizing: zeroize::Zeroizing<String> = token.into();
        *self.token.write().await = Some(Arc::new(zeroizing));
    }

    /// Invalidate the current bearer token and clear it from the client.
    ///
    /// Sends `DELETE /nifi-api/access/logout` to invalidate the token server-side,
    /// then clears the local token unconditionally so that subsequent requests are
    /// not sent with a stale credential.
    ///
    /// If the server returns an error (e.g. `401` because the token had already
    /// expired) the local token is still cleared and the error is returned to the
    /// caller.
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub async fn logout(&self) -> Result<(), NifiError> {
        let result = self.delete_inner("/access/logout").await;
        *self.token.write().await = None;
        if result.is_ok() {
            tracing::info!("NiFi logout successful");
        }
        result
    }

    /// Authenticate with NiFi using single-user credentials.
    ///
    /// Obtains a JWT token from `/nifi-api/access/token` and stores it on the
    /// client for all subsequent requests.
    ///
    /// # Token lifetime and expiry
    ///
    /// NiFi JWTs expire after 12 hours by default (configurable server-side via
    /// `nifi.security.user.login.identity.provider.expiration`). Once expired,
    /// any API call returns [`NifiError::Unauthorized`]. Configure an
    /// [`AuthProvider`] on the builder to enable
    /// automatic token refresh on 401 responses.
    #[tracing::instrument(skip(self, username, password), fields(request_id = tracing::field::Empty))]
    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {
        let method = Method::POST;
        tracing::debug!(method = %method, path = "/access/token", "NiFi API request");
        let url = self.api_url("/access/token");
        let req = self.apply_request_id(self.http.post(url));
        let resp = req
            .form(&[("username", username), ("password", password)])
            .send()
            .await
            .context(HttpSnafu)?;

        let status = resp.status();
        tracing::debug!(
            method = %method,
            path = "/access/token",
            status = status.as_u16(),
            "NiFi API response"
        );
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_else(|_| status.to_string());
            tracing::debug!(
                method = %method,
                path = "/access/token",
                status = status.as_u16(),
                %body,
                "NiFi API raw error body"
            );
            let message = extract_error_message(&body);
            tracing::warn!(
                method = %method,
                path = "/access/token",
                status = status.as_u16(),
                %message,
                "NiFi API error"
            );
            return AuthSnafu { message }.fail();
        }

        let token = resp.text().await.context(HttpSnafu)?;
        *self.token.write().await = Some(Arc::new(zeroize::Zeroizing::new(token)));
        tracing::info!("NiFi login successful for {username}");
        Ok(())
    }

    /// Authenticate using the configured [`AuthProvider`].
    ///
    /// Returns [`NifiError::Auth`] if no auth provider has been configured.
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub async fn authenticate(&self) -> Result<(), NifiError> {
        let provider = self.auth_provider.as_ref().ok_or_else(|| NifiError::Auth {
            message: "no auth provider configured".to_string(),
        })?;
        provider.authenticate(self).await
    }

    // ── Auth-retry wrapper ────────────────────────────────────────────────────

    /// Execute `f`, and if it returns `NifiError::Unauthorized` and a credential
    /// provider is configured, refresh the token and retry once.
    ///
    /// Uses a mutex + token-snapshot check to ensure that concurrent 401
    /// responses only trigger a single re-authentication: whichever task wins
    /// the lock re-auths; tasks that arrive later skip re-auth because they
    /// observe a changed token.
    ///
    /// The snapshot is an [`Arc`] handle, not a cloned `String` — comparison
    /// is [`Arc::ptr_eq`] so the token's plaintext bytes never leak into a
    /// non-zeroized heap allocation just to drive the retry decision (audit
    /// follow-up B9.1).
    #[tracing::instrument(skip_all)]
    async fn with_auth_retry<T, F, Fut>(&self, f: F) -> Result<T, NifiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, NifiError>>,
    {
        // Snapshot the token at entry so we can detect whether a concurrent
        // task already re-authed while we were waiting on the lock.
        let token_before = self.token_handle().await;

        match f().await {
            Err(NifiError::Unauthorized { .. }) if self.auth_provider.is_some() => {
                let _guard = self.auth_lock.lock().await;
                let token_now = self.token_handle().await;
                if token_handle_eq(&token_before, &token_now) {
                    tracing::info!("received 401, refreshing token via auth provider");
                    self.authenticate().await?;
                } else {
                    tracing::debug!("token already refreshed by concurrent task, skipping re-auth");
                }
                // Release the auth lock BEFORE retrying the request — otherwise
                // the retry's `f().await` would hold the lock across its entire
                // HTTP round-trip, serializing every concurrent request through
                // a single critical section.
                drop(_guard);
                f().await
            }
            other => other,
        }
    }

    // ── Transient-error retry wrapper ──────────────────────────────────────────

    /// Execute `f` with optional transient-error retry using exponential backoff.
    ///
    /// When a [`RetryPolicy`](crate::config::retry::RetryPolicy) is configured, retries
    /// [retryable](NifiError::is_retryable) errors up to `max_retries` times.
    /// Each attempt goes through [`with_auth_retry`] so 401 handling still works.
    #[tracing::instrument(skip_all)]
    async fn with_retry<T, F, Fut>(&self, f: F) -> Result<T, NifiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, NifiError>>,
    {
        let Some(policy) = &self.retry_policy else {
            return self.with_auth_retry(&f).await;
        };

        let mut last_err: Option<NifiError> = None;
        for attempt in 0..=policy.max_retries {
            if attempt > 0 {
                let backoff = policy.backoff_for(attempt - 1);
                tracing::info!(
                    attempt,
                    backoff_ms = backoff.as_millis() as u64,
                    "retrying after transient error"
                );
                tokio::time::sleep(backoff).await;
            }
            match self.with_auth_retry(&f).await {
                Ok(v) => return Ok(v),
                Err(e) if e.is_retryable() => {
                    tracing::warn!(attempt, error = %e, "transient error, will retry");
                    last_err = Some(e);
                }
                Err(e) => return Err(e),
            }
        }
        // The loop always executes at least once (range `0..=max_retries`).
        // Each iteration either returns directly (`Ok` or non-retryable `Err`)
        // or sets `last_err` to `Some(_)`. So if we reach here, `last_err` is
        // populated. Encoding the invariant with `unreachable!` prevents a
        // future maintainer from silently re-issuing an extra HTTP request if
        // the loop logic ever changes.
        match last_err {
            Some(e) => Err(e),
            None => unreachable!(
                "with_retry loop ran at least once and every retryable error stores last_err"
            ),
        }
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::GET, path, self.http.get(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::GET, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, body), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post<B, T>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
    ) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, body), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn put<B, T>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
    ) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::PUT, path, self.http.put(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::PUT, path, resp).await
        })
        .await
    }

    /// POST with no request body; deserializes the JSON response.
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST with no request body; ignores the response body.
    ///
    /// Called by both the static per-version emitter (for POST endpoints
    /// with no body and an empty response) and the dynamic canonical
    /// emitter. No current NiFi 2.x spec triggers the static path, so
    /// this helper is only reached via generated code in `$OUT_DIR` that
    /// clippy's dead-code lint cannot see. Kept available via
    /// `#[allow(dead_code)]` rather than deleted.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_void_no_body(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST with a JSON body; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(POST, Json body, Empty response)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, body), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_void<B: serde::Serialize>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::check_void(&Method::POST, path, resp).await
        })
        .await
    }

    /// PUT with no request body; deserializes the JSON response.
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn put_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::PUT, path, self.http.put(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::PUT, path, resp).await
        })
        .await
    }

    /// PUT with a JSON body; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(PUT, Json body, Empty response)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, body), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn put_void<B: serde::Serialize>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::PUT, path, self.http.put(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::check_void(&Method::PUT, path, resp).await
        })
        .await
    }

    /// PUT with no request body; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(PUT, no body, Empty response)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn put_void_no_body(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::PUT, path, self.http.put(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void(&Method::PUT, path, resp).await
        })
        .await
    }

    /// POST with `application/octet-stream` body.
    ///
    /// Used for binary upload endpoints (e.g. NAR upload).
    /// Pass `("Filename", name)` in `extra_headers` when the endpoint
    /// requires a filename header.
    #[tracing::instrument(skip(self, data), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_octet_stream<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        data: bytes::Bytes,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await
                .header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
                .body(data.clone());
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST with `multipart/form-data` body.
    ///
    /// Used for file-upload endpoints such as
    /// `POST /process-groups/{id}/process-groups/upload`. Sends a single form
    /// part named `"file"` carrying the given filename and raw bytes. The
    /// `Content-Type` header (including the generated boundary) is set by
    /// reqwest when `.multipart(form)` is called.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, data), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        filename: &str,
        data: bytes::Bytes,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let len = data.len() as u64;
            let part = reqwest::multipart::Part::stream_with_length(data.clone(), len)
                .file_name(filename.to_string());
            let form = reqwest::multipart::Form::new().part("file", part);
            let resp = req.multipart(form).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST `multipart/form-data` with additional text fields alongside
    /// the `file` part.
    ///
    /// Used by endpoints whose multipart schema declares required text
    /// properties beyond the file part — for example
    /// `POST /process-groups/{id}/process-groups/upload` requires
    /// `clientId`, `groupName`, `positionX`, and `positionY`.
    ///
    /// Each `(name, value)` tuple is emitted as a `form.text(...)` part
    /// before the `file` part. The order mirrors the slice order; the
    /// generator emits it in alphabetic order by wire name for
    /// determinism across regenerations.
    #[tracing::instrument(
        skip(self, text_fields, data),
        fields(request_id = tracing::field::Empty)
    )]
    pub(crate) async fn post_multipart_with_fields<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        text_fields: &[(&str, String)],
        filename: &str,
        data: bytes::Bytes,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let mut form = reqwest::multipart::Form::new();
            for (name, value) in text_fields {
                form = form.text((*name).to_string(), value.clone());
            }
            let len = data.len() as u64;
            let part = reqwest::multipart::Part::stream_with_length(data.clone(), len)
                .file_name(filename.to_string());
            form = form.part("file", part);
            let resp = req.multipart(form).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    /// GET that ignores the response body (for endpoints with no JSON response).
    ///
    /// Treats 302 as success in addition to 2xx: NiFi's `GET /access/logout/complete`
    /// responds with a redirect once the logout is complete.
    ///
    /// Called from generated static-emitter code; clippy cannot see those
    /// call sites, and dynamic-only builds skip per-version emission so the
    /// helper appears unused there.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_void(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::GET, path, self.http.get(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void_with_redirect(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET with query parameters; ignores the response body.
    ///
    /// Called by both the static per-version emitter (for GET endpoints
    /// with query params and an empty response) and the dynamic canonical
    /// emitter. No current NiFi 2.x spec triggers the static path, so
    /// this helper is only reached via generated code in `$OUT_DIR` that
    /// clippy's dead-code lint cannot see. Kept available via
    /// `#[allow(dead_code)]` rather than deleted.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_void_with_query(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::GET,
                    path,
                    self.http.get(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void_with_redirect(&Method::GET, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::GET,
                    path,
                    self.http.get(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET returning raw text (`text/plain`).
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_text(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<String, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::GET, path, self.http.get(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::text(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET returning raw bytes (`application/octet-stream` or `*/*`).
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_bytes(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<Vec<u8>, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::GET, path, self.http.get(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::bytes(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET with query parameters returning raw bytes.
    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_bytes_with_query(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<Vec<u8>, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::GET,
                    path,
                    self.http.get(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::bytes(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET returning a stream of body chunks.
    ///
    /// The initial request is subject to the configured `AuthProvider`
    /// (401→re-auth-once) and `RetryPolicy`. Once the response has been
    /// accepted (status-line read, 2xx/206), the stream takes over and
    /// transport errors during body delivery propagate directly to the
    /// caller — they are not retried.
    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_bytes_stream(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<crate::BytesStream, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::GET, path, self.http.get(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::bytes_stream(&Method::GET, path, resp).await
        })
        .await
    }

    /// GET with query parameters returning a stream of body chunks.
    /// See [`Self::get_bytes_stream`] for retry/streaming semantics.
    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn get_bytes_stream_with_query(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<crate::BytesStream, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::GET,
                    path,
                    self.http.get(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::bytes_stream(&Method::GET, path, resp).await
        })
        .await
    }

    /// POST with `application/octet-stream` body; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(POST, OctetStream body, Empty response)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, data), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_void_octet_stream(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        data: bytes::Bytes,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await
                .header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
                .body(data.clone());
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST with `multipart/form-data` body; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(POST, Multipart body, Empty response)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, data), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_void_multipart(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        filename: &str,
        data: bytes::Bytes,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let len = data.len() as u64;
            let part = reqwest::multipart::Part::stream_with_length(data.clone(), len)
                .file_name(filename.to_string());
            let form = reqwest::multipart::Form::new().part("file", part);
            let resp = req.multipart(form).send().await.context(HttpSnafu)?;
            Self::check_void(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST a JSON body and return the `text/plain` response body.
    ///
    /// Called from generated static-emitter code; clippy cannot see those
    /// call sites, and dynamic-only builds skip per-version emission so the
    /// helper appears unused there.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, body), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_returning_text<B: serde::Serialize>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
    ) -> Result<String, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::text(&Method::POST, path, resp).await
        })
        .await
    }

    /// POST an `application/octet-stream` body and return the `text/plain` response body.
    ///
    /// Called from generated static-emitter code; clippy cannot see those
    /// call sites, and dynamic-only builds skip per-version emission so the
    /// helper appears unused there.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, data), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_octet_stream_returning_text(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        data: bytes::Bytes,
    ) -> Result<String, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::POST, path, self.http.post(self.api_url(path)))
                .await
                .header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
                .body(data.clone());
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::text(&Method::POST, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn delete_returning_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::DELETE,
                    path,
                    self.http.delete(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::DELETE, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn delete_with_query(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::DELETE,
                    path,
                    self.http.delete(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void(&Method::DELETE, path, resp).await
        })
        .await
    }

    /// POST with a JSON body and query parameters; ignores the response body.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(POST, any body, Empty response, query params)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, body, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_void_with_query<B: serde::Serialize>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::POST,
                    path,
                    self.http.post(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::check_void(&Method::POST, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self, body, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn post_with_query<B, T>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
        query: &[(&str, String)],
    ) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::POST,
                    path,
                    self.http.post(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::POST, path, resp).await
        })
        .await
    }

    /// PUT with a JSON body and query parameters; deserializes the JSON response.
    ///
    /// Kept available for forward compatibility — the emitter dispatch table at
    /// `emit::method` references this helper for the `(PUT, Json body, Non-empty response, query params)`
    /// combination, but no current NiFi 2.x spec triggers that path.
    #[allow(dead_code)]
    #[tracing::instrument(skip(self, body, query), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn put_with_query<B, T>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
        body: &B,
        query: &[(&str, String)],
    ) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            let req = self
                .build_request(
                    &Method::PUT,
                    path,
                    self.http.put(self.api_url(path)).query(query),
                )
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.json(body).send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::PUT, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn delete_returning<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::DELETE, path, self.http.delete(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::deserialize(&Method::DELETE, path, resp).await
        })
        .await
    }

    #[tracing::instrument(skip(self), fields(request_id = tracing::field::Empty))]
    pub(crate) async fn delete(
        &self,
        path: &str,
        extra_headers: &[(&str, &str)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            let req = self
                .build_request(&Method::DELETE, path, self.http.delete(self.api_url(path)))
                .await;
            let req = apply_extra_headers(req, extra_headers);
            let resp = req.send().await.context(HttpSnafu)?;
            Self::check_void(&Method::DELETE, path, resp).await
        })
        .await
    }

    /// Inner delete without auth retry, used by `logout` to avoid retrying
    /// the logout call itself.
    async fn delete_inner(&self, path: &str) -> Result<(), NifiError> {
        let resp = self
            .build_request(&Method::DELETE, path, self.http.delete(self.api_url(path)))
            .await
            .send()
            .await
            .context(HttpSnafu)?;
        Self::check_void(&Method::DELETE, path, resp).await
    }

    /// Attach a fresh UUIDv4 to the request as the configured request-id
    /// header AND record it on the current tracing span. No-op if the client
    /// has not been configured with `request_id_header`.
    ///
    /// Called from `build_request` (which covers every HTTP helper including
    /// `delete_inner`) and directly from `login`, which bypasses
    /// `build_request` because it runs pre-authentication with a form-encoded
    /// body.
    fn apply_request_id(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let Some(header) = self.request_id_header.as_deref() else {
            return req;
        };
        let id = uuid::Uuid::new_v4().to_string();
        tracing::Span::current().record("request_id", id.as_str());
        req.header(header, id)
    }

    /// Apply the auth header, proxied-entities chain, and per-request debug
    /// log to a `RequestBuilder`. All HTTP helpers route through this method so
    /// the plumbing lives in exactly one place.
    async fn build_request(
        &self,
        method: &Method,
        path: &str,
        req: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        let req = self.apply_request_id(req);
        tracing::debug!(method = %method, path, "NiFi API request");

        let guard = self.token.read().await;
        let mut req = match guard.as_ref() {
            Some(token) => req.bearer_auth(token.as_str()),
            None => {
                tracing::warn!(
                    "sending NiFi API request without a bearer token — call login() first"
                );
                req
            }
        };
        if let Some(chain) = &self.proxied_entities_chain {
            req = req.header(PROXIED_ENTITIES_CHAIN, chain);
        }
        req
    }

    async fn deserialize<T: DeserializeOwned>(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<T, NifiError> {
        let resp = handle_response_status(method, path, resp).await?;
        resp.json::<T>().await.context(HttpSnafu)
    }

    /// Check a void response (no JSON body expected). Returns `Ok(())` on success,
    /// or the appropriate error.
    async fn check_void(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<(), NifiError> {
        handle_response_status(method, path, resp).await?;
        Ok(())
    }

    /// Read a raw `text/plain` (or equivalent) response body as a `String`.
    async fn text(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<String, NifiError> {
        let resp = handle_response_status(method, path, resp).await?;
        resp.text().await.context(HttpSnafu)
    }

    /// Read a raw `application/octet-stream` (or equivalent) response body as bytes.
    async fn bytes(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<Vec<u8>, NifiError> {
        let resp = handle_response_status(method, path, resp).await?;
        let b = resp.bytes().await.context(HttpSnafu)?;
        Ok(b.to_vec())
    }

    /// Turn a successful `application/octet-stream` (or `*/*`) response into
    /// a [`crate::BytesStream`]. Non-2xx status codes are converted into the
    /// same `NifiError` shape that [`Self::bytes`] produces.
    async fn bytes_stream(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<crate::BytesStream, NifiError> {
        use futures_util::TryStreamExt;
        let resp = handle_response_status(method, path, resp).await?;
        let s = resp
            .bytes_stream()
            .map_err(|source| NifiError::Http { source });
        Ok(Box::pin(s))
    }

    /// Like `check_void`, but also treats 302 as success.
    ///
    /// Does NOT delegate to [`handle_response_status`] because its
    /// success predicate also admits `StatusCode::FOUND` (302). Keeping
    /// the redirect semantics out of the shared helper means
    /// [`handle_response_status`] stays a plain "2xx-or-error" gate.
    async fn check_void_with_redirect(
        method: &Method,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<(), NifiError> {
        let status = resp.status();
        tracing::debug!(method = %method, path, status = status.as_u16(), "NiFi API response");
        if status.is_success() || status == StatusCode::FOUND {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        let logged_body = truncate_for_log(&body);
        tracing::debug!(method = %method, path, status = status.as_u16(), body = %logged_body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = %method, path, status = status.as_u16(), %message, "NiFi API error");
        Err(crate::error::api_error(status.as_u16(), message))
    }

    pub(crate) fn api_url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(&format!("/nifi-api{path}"));
        url
    }
}

/// Compare two token snapshots for "same token instance" using
/// [`Arc::ptr_eq`]. Used by [`NifiClient::with_auth_retry`] to decide
/// whether a concurrent task already rotated the token while the current
/// task was waiting on the auth lock — without ever cloning the token's
/// plaintext bytes into a non-zeroized `String` (audit follow-up B9.1).
///
/// Token rotation in [`NifiClient::set_token`] / [`NifiClient::login`]
/// always installs a fresh `Arc::new(Zeroizing::new(...))`, so pointer
/// identity is a sound proxy for byte-equality here: two snapshots that
/// point at the same allocation cannot have observed an intervening
/// rotation.
fn token_handle_eq(a: &Option<TokenHandle>, b: &Option<TokenHandle>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Shared preamble for response helpers.
///
/// Emits the single `tracing::debug!` response line every helper used
/// to emit inline, and on non-2xx statuses consumes the body, logs the
/// raw body at `debug!` plus the extracted message at `warn!`, and
/// returns `Err(NifiError::Api)`. On 2xx the response is handed back to
/// the caller so it can read the body however it needs (json / text /
/// bytes / stream).
///
/// [`NifiClient::check_void_with_redirect`] deliberately does NOT
/// delegate here — its success predicate admits `StatusCode::FOUND`
/// (302) in addition to 2xx, and folding that branch into the shared
/// helper would leak redirect semantics into every caller.
async fn handle_response_status(
    method: &Method,
    path: &str,
    resp: reqwest::Response,
) -> Result<reqwest::Response, NifiError> {
    let status = resp.status();
    tracing::debug!(method = %method, path, status = status.as_u16(), "NiFi API response");
    if status.is_success() {
        return Ok(resp);
    }
    let body = resp.text().await.unwrap_or_else(|_| status.to_string());
    let logged_body = truncate_for_log(&body);
    tracing::debug!(method = %method, path, status = status.as_u16(), body = %logged_body, "NiFi API raw error body");
    let message = extract_error_message(&body);
    tracing::warn!(method = %method, path, status = status.as_u16(), %message, "NiFi API error");
    Err(crate::error::api_error(status.as_u16(), message))
}

/// Truncate a (potentially huge) response body for `tracing::debug!`.
///
/// NiFi error responses are usually small JSON envelopes, but the body can
/// occasionally be a multi-megabyte stack trace or HTML page from an upstream
/// proxy. Logging the full text balloons trace output, eats disk on
/// long-running services, and slows tracing subscribers that copy strings.
///
/// The cap (`MAX`) is chosen so a single truncated line — including the
/// `... (truncated, N bytes)` suffix — fits comfortably in a 300-byte
/// debug-line budget.
///
/// Truncation is byte-based but kept on a UTF-8 char boundary so the
/// returned slice is always valid UTF-8. The suffix records the *original*
/// byte length so a reader can tell how much was elided.
pub(crate) fn truncate_for_log(body: &str) -> std::borrow::Cow<'_, str> {
    const MAX: usize = 256;
    if body.len() <= MAX {
        return std::borrow::Cow::Borrowed(body);
    }
    // Walk back to a char boundary at or below MAX so we never split a UTF-8
    // sequence. `is_char_boundary` is constant-time.
    let mut cut = MAX;
    while cut > 0 && !body.is_char_boundary(cut) {
        cut -= 1;
    }
    std::borrow::Cow::Owned(format!(
        "{} ... (truncated, {} bytes)",
        &body[..cut],
        body.len()
    ))
}

/// Apply a fold of `(name, value)` header pairs to a `RequestBuilder`.
/// Kept as a free helper so HTTP methods stay one-liners after the refactor.
fn apply_extra_headers(
    mut req: reqwest::RequestBuilder,
    extra: &[(&str, &str)],
) -> reqwest::RequestBuilder {
    for (name, value) in extra {
        req = req.header(*name, *value);
    }
    req
}

/// Extract a human-readable message from a NiFi error response body.
///
/// NiFi returns either a JSON object with a `"message"` field or plain text.
/// Logs the raw body at `debug` level before extracting.
///
/// Only the **top-level** `"message"` is consulted. The previous
/// `serde_json::Value` indexing (`v["message"]`) silently fell through arrays
/// and nested objects, so a body like
/// `{"errors": [{"message": "wrong"}], "message": "right"}` could surface
/// the wrong text. Restricting the lookup to the top-level object keeps the
/// behaviour predictable.
pub fn extract_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.as_object()
                .and_then(|o| o.get("message"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| body.to_owned())
}

#[cfg(test)]
mod tests {
    /// Pins the invariant the upload helpers rely on: cloning a `bytes::Bytes`
    /// handle is a refcount bump, not a heap allocation. The retry closures in
    /// `post_octet_stream`, `post_void_octet_stream`, `post_multipart`,
    /// `post_void_multipart`, `post_multipart_with_fields`, and
    /// `post_octet_stream_returning_text` call `data.clone()` on every attempt;
    /// if someone ever switches the parameter back to `Vec<u8>`, a multi-GB
    /// NAR upload would reallocate the entire buffer on each retry. This test
    /// fails loudly in that case.
    #[test]
    fn bytes_clone_is_refcount_only() {
        use bytes::Bytes;
        let data = Bytes::from(vec![0u8; 1024]);
        let before = data.len();
        let clone1 = data.clone();
        let clone2 = data.clone();
        assert_eq!(clone1.len(), before);
        assert_eq!(clone2.len(), before);
        assert_eq!(
            data.as_ptr(),
            clone1.as_ptr(),
            "Bytes::clone should share buffer"
        );
        assert_eq!(
            data.as_ptr(),
            clone2.as_ptr(),
            "Bytes::clone should share buffer"
        );
    }

    /// Pins audit follow-up B9.1: the auth-retry token snapshot must be a
    /// cheap [`Arc`] handle, not a cloned `String`. Two snapshots taken
    /// before any rotation must be `Arc::ptr_eq`. After [`set_token`], a
    /// fresh `Arc` is installed and `ptr_eq` against the pre-rotation
    /// handle must return `false` — that's how `with_auth_retry` decides
    /// whether a concurrent task already re-authed.
    ///
    /// If anyone ever swaps `token_handle` back to a byte-cloning approach,
    /// this test still passes for the equality semantics, but the leak path
    /// it guards (plaintext JWT in non-zeroized heap) returns. The doc
    /// comment is the canary; please don't bypass the test by reverting the
    /// internals.
    #[tokio::test]
    async fn token_handle_uses_arc_ptr_eq() {
        use std::sync::Arc;

        let client = crate::NifiClientBuilder::new("https://nifi.example")
            .expect("valid base url")
            .build()
            .expect("client builds without network");

        client.set_token("alpha".to_string()).await;
        let h1 = client.token_handle().await;
        let h2 = client.token_handle().await;
        match (&h1, &h2) {
            (Some(a), Some(b)) => assert!(
                Arc::ptr_eq(a, b),
                "two snapshots of the same token must be ptr-equal"
            ),
            _ => panic!("expected Some/Some after set_token"),
        }

        // Rotate the token: ptr_eq with the pre-rotation handle must fail.
        client.set_token("beta".to_string()).await;
        let h3 = client.token_handle().await;
        match (&h1, &h3) {
            (Some(a), Some(b)) => assert!(
                !Arc::ptr_eq(a, b),
                "rotated token must NOT be ptr-equal to the pre-rotation handle"
            ),
            _ => panic!("expected Some/Some after rotation"),
        }

        // None/None handles compare equal via the helper.
        assert!(super::token_handle_eq(&None, &None));
        assert!(!super::token_handle_eq(&None, &h3));
        assert!(!super::token_handle_eq(&h1, &h3));
        assert!(super::token_handle_eq(&h1, &h2));
    }

    /// Pins audit follow-up B9.2: the public [`NifiClient::token`] return type
    /// is [`zeroize::Zeroizing<String>`] (not plain `String`). The byte buffer
    /// returned to callers is wiped on drop, instead of leaking through
    /// reallocation. The type assertion in the body fails to compile if
    /// anyone widens the signature back to `Option<String>`.
    #[tokio::test]
    async fn token_returns_zeroizing_wrapper() {
        let client = crate::NifiClientBuilder::new("https://nifi.example")
            .expect("valid base url")
            .build()
            .expect("client builds without network");

        client.set_token("alpha".to_string()).await;
        let token = client.token().await.expect("set_token then token");
        assert_eq!(token.as_str(), "alpha");
        // Compile-time assertion: the public return type is Zeroizing<String>.
        let _: zeroize::Zeroizing<String> = token;
    }

    /// Pins audit follow-up B9.2: [`NifiClient::set_token`] takes
    /// `impl Into<Zeroizing<String>>` and therefore accepts both a plain
    /// `String` (via the blanket `From<T> for Zeroizing<T>` impl in the
    /// `zeroize` crate) and an existing `Zeroizing<String>`. The latter is
    /// the path used by [`crate::config::auth::StaticTokenAuth`] to avoid
    /// detouring through a non-zeroized `String`.
    #[tokio::test]
    async fn set_token_accepts_string_or_zeroizing() {
        let client = crate::NifiClientBuilder::new("https://nifi.example")
            .expect("valid base url")
            .build()
            .expect("client builds without network");

        // Plain String — exercised by all existing call sites.
        client.set_token("from_string".to_string()).await;
        assert_eq!(
            client.token().await.as_deref().map(String::as_str),
            Some("from_string")
        );

        // Pre-wrapped Zeroizing<String> — passes through without an extra clone.
        client
            .set_token(zeroize::Zeroizing::new("from_zeroizing".to_string()))
            .await;
        assert_eq!(
            client.token().await.as_deref().map(String::as_str),
            Some("from_zeroizing")
        );
    }

    /// Pins audit follow-up A13: `extract_error_message` must look up
    /// `"message"` only on the top-level JSON object. The previous
    /// implementation used `serde_json::Value`'s index operator, which
    /// silently traverses nested objects and arrays — a body with
    /// `{"errors": [{"message": "wrong"}], "message": "right"}` could surface
    /// the wrong string.
    #[test]
    fn extract_error_message_uses_top_level_only() {
        use super::extract_error_message;

        assert_eq!(
            extract_error_message(r#"{"errors": [{"message": "wrong"}], "message": "right"}"#),
            "right"
        );
    }

    /// Top-level `message` is still found when no other candidate exists.
    #[test]
    fn extract_error_message_finds_simple_top_level() {
        use super::extract_error_message;

        assert_eq!(extract_error_message(r#"{"message": "boom"}"#), "boom");
    }

    /// JSON without a top-level `message` (e.g. only nested) falls back to
    /// the raw body — we no longer dig into nested objects/arrays.
    #[test]
    fn extract_error_message_no_top_level_returns_body() {
        use super::extract_error_message;

        let body = r#"{"errors": [{"message": "deep"}]}"#;
        assert_eq!(extract_error_message(body), body);
    }

    /// Plain-text bodies are returned unchanged.
    #[test]
    fn extract_error_message_plain_text_passthrough() {
        use super::extract_error_message;

        assert_eq!(extract_error_message("not json at all"), "not json at all");
    }

    /// JSON arrays / scalars at the top level have no `message` field; fall
    /// back to the body.
    #[test]
    fn extract_error_message_top_level_array_returns_body() {
        use super::extract_error_message;

        let body = r#"[{"message": "ignored"}]"#;
        assert_eq!(extract_error_message(body), body);
    }

    /// Pins audit follow-up B7: `truncate_for_log` caps debug output so a
    /// multi-MB error body cannot blow up tracing buffers / log files.
    /// The truncated string fits inside a ~300-byte debug-line budget and
    /// records the original size so readers know how much was elided.
    #[test]
    fn truncate_for_log_caps_long_body() {
        use super::truncate_for_log;

        let body = "x".repeat(10 * 1024 * 1024);
        let logged = truncate_for_log(&body);
        assert!(
            logged.len() <= 300,
            "truncated log line was {} bytes; expected ≤ 300",
            logged.len()
        );
        assert!(
            logged.ends_with(&format!("(truncated, {} bytes)", body.len())),
            "missing/incorrect truncation suffix: {logged}"
        );
    }

    /// Short bodies pass through untouched (no allocation, no suffix).
    #[test]
    fn truncate_for_log_passthrough_short() {
        use super::truncate_for_log;

        let body = r#"{"message":"boom"}"#;
        let logged = truncate_for_log(body);
        assert_eq!(&*logged, body);
        assert!(matches!(logged, std::borrow::Cow::Borrowed(_)));
    }

    /// Truncation respects UTF-8 char boundaries — splitting in the middle
    /// of a multi-byte sequence would produce invalid UTF-8 and panic on the
    /// `format!`.
    #[test]
    fn truncate_for_log_respects_utf8_boundary() {
        use super::truncate_for_log;

        // 4-byte char repeated past the cap; cut MUST land on a boundary.
        let body = "💥".repeat(100);
        let logged = truncate_for_log(&body);
        assert!(logged.len() <= 300);
        assert!(logged.contains("(truncated"));
    }
}

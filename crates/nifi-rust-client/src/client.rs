use std::sync::Arc;

use reqwest::Client;
use serde::de::DeserializeOwned;
use snafu::ResultExt as _;
use tokio::sync::RwLock;
use url::Url;

use crate::NifiError;
use crate::config::credentials::CredentialProvider;
use crate::error::{AuthSnafu, HttpSnafu};

/// Client for the Apache NiFi REST API.
pub struct NifiClient {
    base_url: Url,
    http: Client,
    token: Arc<RwLock<Option<String>>>,
    credentials: Option<Arc<dyn CredentialProvider>>,
    #[allow(dead_code)]
    retry_policy: Option<crate::config::retry::RetryPolicy>,
}

impl Clone for NifiClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            http: self.http.clone(),
            token: Arc::clone(&self.token),
            credentials: self.credentials.clone(),
            retry_policy: self.retry_policy.clone(),
        }
    }
}

impl std::fmt::Debug for NifiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NifiClient")
            .field("base_url", &self.base_url)
            .field(
                "credentials",
                &self.credentials.as_ref().map(|c| format!("{c:?}")),
            )
            .field("retry_policy", &self.retry_policy)
            .finish_non_exhaustive()
    }
}

impl NifiClient {
    /// Construct a client from pre-built parts. Used by [`crate::NifiClientBuilder`].
    pub(crate) fn from_parts(
        base_url: Url,
        http: Client,
        credentials: Option<Arc<dyn CredentialProvider>>,
        retry_policy: Option<crate::config::retry::RetryPolicy>,
    ) -> Self {
        Self {
            base_url,
            http,
            token: Arc::new(RwLock::new(None)),
            credentials,
            retry_policy,
        }
    }

    /// Return the current bearer token, if one has been set.
    ///
    /// The token is a NiFi-issued JWT. You can persist it between process restarts
    /// and restore it with [`set_token`][Self::set_token] to avoid re-authenticating.
    /// The token will eventually expire (NiFi default: 12 hours); when it does, any
    /// API call returns [`NifiError::Unauthorized`]. Re-call
    /// [`login`][Self::login] to obtain a fresh token.
    pub async fn token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    /// Restore a previously obtained bearer token.
    ///
    /// Useful for CLI tools that persist the token in a file between sessions.
    /// If the token has expired, the next API call will return
    /// [`NifiError::Unauthorized`]; re-call [`login`][Self::login]
    /// to obtain a fresh one.
    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Some(token);
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
    /// any API call returns [`NifiError::Unauthorized`]. Configure a
    /// [`CredentialProvider`] on the builder to enable
    /// automatic token refresh on 401 responses.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "POST", path = "/access/token", "NiFi API request");
        let url = self.api_url("/access/token");
        let resp = self
            .http
            .post(url)
            .form(&[("username", username), ("password", password)])
            .send()
            .await
            .context(HttpSnafu)?;

        let status = resp.status();
        tracing::debug!(
            method = "POST",
            path = "/access/token",
            status = status.as_u16(),
            "NiFi API response"
        );
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_else(|_| status.to_string());
            tracing::debug!(
                method = "POST",
                path = "/access/token",
                status = status.as_u16(),
                %body,
                "NiFi API raw error body"
            );
            let message = extract_error_message(&body);
            tracing::warn!(
                method = "POST",
                path = "/access/token",
                status = status.as_u16(),
                %message,
                "NiFi API error"
            );
            return AuthSnafu { message }.fail();
        }

        let token = resp.text().await.context(HttpSnafu)?;
        *self.token.write().await = Some(token);
        tracing::info!("NiFi login successful for {username}");
        Ok(())
    }

    /// Authenticate using the configured [`CredentialProvider`].
    ///
    /// Returns [`NifiError::Auth`] if no credential provider has been configured.
    pub async fn login_with_provider(&self) -> Result<(), NifiError> {
        let creds = self.credentials.as_ref().ok_or_else(|| NifiError::Auth {
            message: "no credential provider configured".to_string(),
        })?;
        let (username, password) = creds.credentials().await?;
        self.login(&username, &password).await
    }

    // ── Auth-retry wrapper ────────────────────────────────────────────────────

    /// Execute `f`, and if it returns `NifiError::Unauthorized` and a credential
    /// provider is configured, refresh the token and retry once.
    #[tracing::instrument(skip_all)]
    async fn with_auth_retry<T, F, Fut>(&self, f: F) -> Result<T, NifiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, NifiError>>,
    {
        match f().await {
            Err(NifiError::Unauthorized { .. }) if self.credentials.is_some() => {
                tracing::info!("received 401, refreshing token via credential provider");
                self.login_with_provider().await?;
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
        // Safety: the loop always executes at least once (attempt 0..=max_retries),
        // and every iteration that reaches here sets `last_err`.
        match last_err {
            Some(e) => Err(e),
            // unreachable: loop runs at least once and non-retryable errors return early
            None => self.with_auth_retry(&f).await,
        }
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "GET", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.get(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("GET", path, resp).await
        })
        .await
    }

    pub(crate) async fn post<B, T>(&self, path: &str, body: &B) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("POST", path, resp).await
        })
        .await
    }

    pub(crate) async fn put<B, T>(&self, path: &str, body: &B) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            tracing::debug!(method = "PUT", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.put(url))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("PUT", path, resp).await
        })
        .await
    }

    /// POST that ignores the response body (for endpoints with no JSON response).
    pub(crate) async fn post_void<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("POST", path, resp).await
        })
        .await
    }

    /// PUT that ignores the response body (for endpoints with no JSON response).
    #[allow(dead_code)]
    pub(crate) async fn put_void<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "PUT", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.put(url))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("PUT", path, resp).await
        })
        .await
    }

    /// POST with no request body; deserializes the JSON response.
    pub(crate) async fn post_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("POST", path, resp).await
        })
        .await
    }

    /// POST with no request body; ignores the response body.
    // Used by the code generator for void no-body POST endpoints without query params.
    // No current NiFi 2.x endpoint triggers this path, but keep it for forward compatibility.
    #[allow(dead_code)]
    pub(crate) async fn post_void_no_body(&self, path: &str) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("POST", path, resp).await
        })
        .await
    }

    /// PUT with no request body; deserializes the JSON response.
    pub(crate) async fn put_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "PUT", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.put(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("PUT", path, resp).await
        })
        .await
    }

    /// PUT with no request body; ignores the response body.
    #[allow(dead_code)]
    pub(crate) async fn put_void_no_body(&self, path: &str) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "PUT", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.put(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("PUT", path, resp).await
        })
        .await
    }

    /// POST with `application/octet-stream` body.
    ///
    /// Used for binary upload endpoints (e.g. NAR upload).
    /// `filename` is sent as the `Filename` request header when provided.
    pub(crate) async fn post_octet_stream<T: DeserializeOwned>(
        &self,
        path: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let builder = self
                .authenticated(self.http.post(url))
                .await
                .header("Content-Type", "application/octet-stream")
                .body(data.clone());
            let builder = if let Some(name) = filename {
                builder.header("Filename", name)
            } else {
                builder
            };
            let resp = builder.send().await.context(HttpSnafu)?;
            Self::deserialize("POST", path, resp).await
        })
        .await
    }

    /// POST with `application/octet-stream` body; ignores the response body.
    ///
    /// Used for binary upload endpoints that return no JSON response.
    /// `filename` is sent as the `Filename` request header when provided.
    pub(crate) async fn post_void_octet_stream(
        &self,
        path: &str,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let builder = self
                .authenticated(self.http.post(url))
                .await
                .header("Content-Type", "application/octet-stream")
                .body(data.clone());
            let builder = if let Some(name) = filename {
                builder.header("Filename", name)
            } else {
                builder
            };
            let resp = builder.send().await.context(HttpSnafu)?;
            Self::check_void("POST", path, resp).await
        })
        .await
    }

    /// POST with query parameters; ignores the response body.
    ///
    /// Used for endpoints that accept query parameters and have no JSON response body.
    #[allow(dead_code)]
    pub(crate) async fn post_void_with_query<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url).query(query))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("POST", path, resp).await
        })
        .await
    }

    /// GET that ignores the response body (for endpoints with no JSON response).
    ///
    /// Treats 302 as success in addition to 2xx: NiFi's `GET /access/logout/complete`
    /// responds with a redirect once the logout is complete.
    pub(crate) async fn get_void(&self, path: &str) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "GET", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.get(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void_with_redirect("GET", path, resp).await
        })
        .await
    }

    pub(crate) async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "GET", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.get(url).query(query))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("GET", path, resp).await
        })
        .await
    }

    pub(crate) async fn get_void_with_query(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "GET", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.get(url).query(query))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void_with_redirect("GET", path, resp).await
        })
        .await
    }

    pub(crate) async fn delete_returning_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "DELETE", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.delete(url).query(query))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("DELETE", path, resp).await
        })
        .await
    }

    pub(crate) async fn delete_with_query(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "DELETE", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.delete(url).query(query))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("DELETE", path, resp).await
        })
        .await
    }

    pub(crate) async fn post_with_query<B, T>(
        &self,
        path: &str,
        body: &B,
        query: &[(&str, String)],
    ) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        self.with_retry(|| async {
            tracing::debug!(method = "POST", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.post(url).query(query))
                .await
                .json(body)
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("POST", path, resp).await
        })
        .await
    }

    pub(crate) async fn delete_returning<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "DELETE", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.delete(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::deserialize("DELETE", path, resp).await
        })
        .await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), NifiError> {
        self.with_retry(|| async {
            tracing::debug!(method = "DELETE", path, "NiFi API request");
            let url = self.api_url(path);
            let resp = self
                .authenticated(self.http.delete(url))
                .await
                .send()
                .await
                .context(HttpSnafu)?;
            Self::check_void("DELETE", path, resp).await
        })
        .await
    }

    /// Inner delete without auth retry, used by `logout` to avoid retrying
    /// the logout call itself.
    async fn delete_inner(&self, path: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "DELETE", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.delete(url))
            .await
            .send()
            .await
            .context(HttpSnafu)?;
        Self::check_void("DELETE", path, resp).await
    }

    async fn authenticated(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let guard = self.token.read().await;
        match guard.as_deref() {
            Some(token) => req.bearer_auth(token),
            None => {
                tracing::warn!(
                    "sending NiFi API request without a bearer token — call login() first"
                );
                req
            }
        }
    }

    async fn deserialize<T: DeserializeOwned>(
        method: &str,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<T, NifiError> {
        let status = resp.status();
        tracing::debug!(method, path, status = status.as_u16(), "NiFi API response");
        if status.is_success() {
            return resp.json::<T>().await.context(HttpSnafu);
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method, path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method, path, status = status.as_u16(), %message, "NiFi API error");
        Err(crate::error::api_error(status.as_u16(), message))
    }

    /// Check a void response (no JSON body expected). Returns `Ok(())` on success,
    /// or the appropriate error.
    async fn check_void(
        method: &str,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<(), NifiError> {
        let status = resp.status();
        tracing::debug!(method, path, status = status.as_u16(), "NiFi API response");
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method, path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method, path, status = status.as_u16(), %message, "NiFi API error");
        Err(crate::error::api_error(status.as_u16(), message))
    }

    /// Like `check_void`, but also treats 302 as success.
    async fn check_void_with_redirect(
        method: &str,
        path: &str,
        resp: reqwest::Response,
    ) -> Result<(), NifiError> {
        let status = resp.status();
        tracing::debug!(method, path, status = status.as_u16(), "NiFi API response");
        if status.is_success() || status.as_u16() == 302 {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method, path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method, path, status = status.as_u16(), %message, "NiFi API error");
        Err(crate::error::api_error(status.as_u16(), message))
    }

    pub(crate) fn api_url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(&format!("/nifi-api{path}"));
        url
    }
}

/// Extract a human-readable message from a NiFi error response body.
///
/// NiFi returns either a JSON object with a `"message"` field or plain text.
/// Logs the raw body at `debug` level before extracting.
pub fn extract_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v["message"].as_str().map(str::to_owned))
        .unwrap_or_else(|| body.to_owned())
}

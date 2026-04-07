use reqwest::Client;
use serde::de::DeserializeOwned;
use snafu::ResultExt as _;
use url::Url;

use crate::NifiError;
use crate::error::{ApiSnafu, AuthSnafu, HttpSnafu};

/// Client for the Apache NiFi REST API.
#[derive(Debug, Clone)]
pub struct NifiClient {
    base_url: Url,
    http: Client,
    token: Option<String>,
}

impl NifiClient {
    /// Construct a client from pre-built parts. Used by [`crate::NifiClientBuilder`].
    pub(crate) fn from_parts(base_url: Url, http: Client) -> Self {
        Self {
            base_url,
            http,
            token: None,
        }
    }

    /// Return the current bearer token, if one has been set.
    ///
    /// The token is a NiFi-issued JWT. You can persist it between process restarts
    /// and restore it with [`set_token`][Self::set_token] to avoid re-authenticating.
    /// The token will eventually expire (NiFi default: 12 hours); when it does, any
    /// API call returns [`NifiError::Api`] with `status == 401`. Re-call
    /// [`login`][Self::login] to obtain a fresh token.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Restore a previously obtained bearer token.
    ///
    /// Useful for CLI tools that persist the token in a file between sessions.
    /// If the token has expired, the next API call will return
    /// [`NifiError::Api`] with `status == 401`; re-call [`login`][Self::login]
    /// to obtain a fresh one.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
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
    pub async fn logout(&mut self) -> Result<(), NifiError> {
        let result = self.delete("/access/logout").await;
        self.token = None;
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
    /// any API call returns [`NifiError::Api`] with `status == 401`. The
    /// recommended pattern:
    ///
    /// ```no_run
    /// # #[cfg(not(feature = "dynamic"))]
    /// # async fn example(client: &mut nifi_rust_client::NifiClient) -> Result<(), nifi_rust_client::NifiError> {
    /// use nifi_rust_client::NifiError;
    /// match client.flow_api().get_about_info().await {
    ///     Err(NifiError::Api { status: 401, .. }) => {
    ///         client.login("admin", "password").await?;
    ///         // retry the call
    ///     }
    ///     other => { other?; }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Automatic re-login is not built in: storing credentials on the client
    /// would keep plaintext passwords in memory for the lifetime of the process,
    /// which is undesirable in most contexts.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), NifiError> {
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
        self.token = Some(token);
        tracing::info!("NiFi login successful for {username}");
        Ok(())
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, NifiError> {
        tracing::debug!(method = "GET", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.get(url))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("GET", path, resp).await
    }

    pub(crate) async fn post<B, T>(&self, path: &str, body: &B) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("POST", path, resp).await
    }

    pub(crate) async fn put<B, T>(&self, path: &str, body: &B) -> Result<T, NifiError>
    where
        B: serde::Serialize,
        T: DeserializeOwned,
    {
        tracing::debug!(method = "PUT", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.put(url))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("PUT", path, resp).await
    }

    /// POST that ignores the response body (for endpoints with no JSON response).
    pub(crate) async fn post_void<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), NifiError> {
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "POST",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "POST", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "POST", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    /// PUT that ignores the response body (for endpoints with no JSON response).
    #[allow(dead_code)]
    pub(crate) async fn put_void<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), NifiError> {
        tracing::debug!(method = "PUT", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.put(url))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "PUT",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "PUT", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "PUT", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    /// POST with no request body; deserializes the JSON response.
    pub(crate) async fn post_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("POST", path, resp).await
    }

    /// POST with no request body; ignores the response body.
    // Used by the code generator for void no-body POST endpoints without query params.
    // No current NiFi 2.x endpoint triggers this path, but keep it for forward compatibility.
    #[allow(dead_code)]
    pub(crate) async fn post_void_no_body(&self, path: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "POST",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "POST", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "POST", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    /// PUT with no request body; deserializes the JSON response.
    pub(crate) async fn put_no_body<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        tracing::debug!(method = "PUT", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.put(url))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("PUT", path, resp).await
    }

    /// PUT with no request body; ignores the response body.
    #[allow(dead_code)]
    pub(crate) async fn put_void_no_body(&self, path: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "PUT", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.put(url))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "PUT",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "PUT", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "PUT", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
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
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let builder = self
            .authenticated(self.http.post(url))
            .header("Content-Type", "application/octet-stream")
            .body(data);
        let builder = if let Some(name) = filename {
            builder.header("Filename", name)
        } else {
            builder
        };
        let resp = builder.send().await.context(HttpSnafu)?;
        Self::deserialize("POST", path, resp).await
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
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let builder = self
            .authenticated(self.http.post(url))
            .header("Content-Type", "application/octet-stream")
            .body(data);
        let builder = if let Some(name) = filename {
            builder.header("Filename", name)
        } else {
            builder
        };
        let resp = builder.send().await.context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "POST",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "POST", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "POST", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
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
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url).query(query))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "POST",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "POST", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "POST", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    /// GET that ignores the response body (for endpoints with no JSON response).
    ///
    /// Treats 302 as success in addition to 2xx: NiFi's `GET /access/logout/complete`
    /// responds with a redirect once the logout is complete.
    pub(crate) async fn get_void(&self, path: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "GET", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.get(url))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "GET",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() || status.as_u16() == 302 {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "GET", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "GET", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    pub(crate) async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        tracing::debug!(method = "GET", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.get(url).query(query))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("GET", path, resp).await
    }

    pub(crate) async fn get_void_with_query(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        tracing::debug!(method = "GET", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.get(url).query(query))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "GET",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() || status.as_u16() == 302 {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "GET", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "GET", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    pub(crate) async fn delete_returning_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, NifiError> {
        tracing::debug!(method = "DELETE", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.delete(url).query(query))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("DELETE", path, resp).await
    }

    pub(crate) async fn delete_with_query(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<(), NifiError> {
        tracing::debug!(method = "DELETE", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.delete(url).query(query))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "DELETE",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "DELETE", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "DELETE", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
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
        tracing::debug!(method = "POST", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.post(url).query(query))
            .json(body)
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("POST", path, resp).await
    }

    pub(crate) async fn delete_returning<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, NifiError> {
        tracing::debug!(method = "DELETE", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.delete(url))
            .send()
            .await
            .context(HttpSnafu)?;
        Self::deserialize("DELETE", path, resp).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), NifiError> {
        tracing::debug!(method = "DELETE", path, "NiFi API request");
        let url = self.api_url(path);
        let resp = self
            .authenticated(self.http.delete(url))
            .send()
            .await
            .context(HttpSnafu)?;
        let status = resp.status();
        tracing::debug!(
            method = "DELETE",
            path,
            status = status.as_u16(),
            "NiFi API response"
        );
        if status.is_success() {
            return Ok(());
        }
        let body = resp.text().await.unwrap_or_else(|_| status.to_string());
        tracing::debug!(method = "DELETE", path, status = status.as_u16(), %body, "NiFi API raw error body");
        let message = extract_error_message(&body);
        tracing::warn!(method = "DELETE", path, status = status.as_u16(), %message, "NiFi API error");
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
    }

    fn authenticated(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.token {
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
        ApiSnafu {
            status: status.as_u16(),
            message,
        }
        .fail()
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

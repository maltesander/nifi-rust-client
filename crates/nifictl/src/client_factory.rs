use nifi_rust_client::NifiClientBuilder;
use nifi_rust_client::dynamic::DynamicClient;
use nifi_rust_client::dynamic::VersionResolutionStrategy;

use crate::config::{AuthConfig, Context};
use crate::error::CliError;

// ---------------------------------------------------------------------------
// Auth resolution
// ---------------------------------------------------------------------------

/// Resolved authentication credentials, ready to hand to the NiFi client.
#[derive(Debug)]
pub enum ResolvedAuth {
    /// Password auth. `password` is `None` when no password source was
    /// available at resolve time — `build_client` prompts on TTY.
    Password {
        username: String,
        password: Option<String>,
    },
    Token(String),
    Mtls {
        identity_path: String,
    },
}

// ---------------------------------------------------------------------------
// ResolvedParams
// ---------------------------------------------------------------------------

/// All connection parameters resolved from flags and/or the active context.
#[derive(Debug)]
pub struct ResolvedParams {
    pub url: String,
    pub auth: ResolvedAuth,
    pub insecure: bool,
    pub ca_cert_path: Option<String>,
    pub proxied_entities_chain: Option<String>,
    pub version_strategy: VersionResolutionStrategy,
}

impl ResolvedParams {
    /// Resolve connection parameters by merging CLI flags with the active context.
    ///
    /// Precedence: explicit flags > context values.
    ///
    /// Auth resolution order:
    /// 1. `--token` flag present → `Token` auth.
    /// 2. Both `--username` and `--password` present → `Password` auth.
    /// 3. Active context auth config.
    /// 4. No auth available → `CliError::User`.
    #[allow(clippy::too_many_arguments)]
    pub fn resolve(
        url: Option<String>,
        username: Option<String>,
        password: Option<String>,
        token: Option<String>,
        insecure: bool,
        context: Option<&Context>,
    ) -> Result<Self, CliError> {
        // URL: flag > context > error
        let url = url
            .or_else(|| context.map(|c| c.url.clone()))
            .ok_or_else(|| {
                CliError::User(
                    "no NiFi URL provided; use --url or set a context in the config file"
                        .to_string(),
                )
            })?;

        // Auth: flag token > flag username+password > context auth
        let auth = if let Some(t) = token {
            ResolvedAuth::Token(t)
        } else if let Some(u) = username {
            // Password may be `None` (no --password, NIFI_PASSWORD unset) —
            // build_client will prompt on TTY or error off-TTY.
            ResolvedAuth::Password {
                username: u,
                password,
            }
        } else if let Some(ctx) = context {
            resolve_auth_from_context(ctx)?
        } else {
            return Err(CliError::User(
                "no authentication provided; use --token, --username/--password, \
                 or configure a context with auth"
                    .to_string(),
            ));
        };

        // insecure: flag takes precedence (true overrides context false; context can set true too)
        let resolved_insecure = insecure || context.map(|c| c.insecure).unwrap_or(false);

        let ca_cert_path = context.and_then(|c| c.ca_cert_path.clone());
        let proxied_entities_chain = context.and_then(|c| c.proxied_entities_chain.clone());

        let version_strategy = context
            .and_then(|c| c.version_strategy.as_deref())
            .map(parse_strategy)
            .transpose()?
            .unwrap_or_default();

        Ok(ResolvedParams {
            url,
            auth,
            insecure: resolved_insecure,
            ca_cert_path,
            proxied_entities_chain,
            version_strategy,
        })
    }

    /// Build and authenticate a `DynamicClient` from the resolved parameters.
    pub async fn build_client(&self) -> Result<DynamicClient, CliError> {
        let mut builder = NifiClientBuilder::new(&self.url)?;

        builder = builder
            .danger_accept_invalid_certs(self.insecure)
            .version_strategy(self.version_strategy);

        if let Some(ca_path) = &self.ca_cert_path {
            let pem = std::fs::read(ca_path)?;
            builder = builder.add_root_certificate(&pem);
        }

        if let Some(chain) = &self.proxied_entities_chain {
            builder = builder.proxied_entities_chain(chain.clone());
        }

        if let ResolvedAuth::Mtls { identity_path } = &self.auth {
            let pem = std::fs::read(identity_path)?;
            builder = builder.client_identity_pem(&pem)?;
        }

        let client = builder.build_dynamic()?;

        match &self.auth {
            ResolvedAuth::Password { username, password } => {
                let pw = match password {
                    Some(p) => p.clone(),
                    None => crate::prompt::prompt_password(username, &self.url)?,
                };
                client.login(username, &pw).await?;
            }
            ResolvedAuth::Token(token) => {
                client.set_token(token.clone()).await;
                client.detect_version().await?;
            }
            ResolvedAuth::Mtls { .. } => {
                client.detect_version().await?;
            }
        }

        Ok(client)
    }

    /// Build a `DynamicClient` that prefers the cached JWT for
    /// `context_name` over re-authentication.
    ///
    /// For `Password` auth: if `~/.nifictl/tokens/<context_name>`
    /// holds a JWT whose `exp` is at least 60 seconds in the future,
    /// install it via `set_token` and call `detect_version` instead
    /// of `login`. On cache miss, malformed token, or expired token,
    /// fall back to [`Self::build_client`].
    ///
    /// For `Token` and `Mtls` auth: behaviour is identical to
    /// [`Self::build_client`] — those paths don't burn a
    /// `/access/token` round-trip per call, so the cache is irrelevant.
    #[allow(dead_code)] // dispatch arms route through this in Task 3
    pub async fn build_client_with_cache(
        &self,
        context_name: &str,
    ) -> Result<DynamicClient, CliError> {
        // Cache only helps the Password path; everything else delegates
        // straight through to build_client.
        if !matches!(self.auth, ResolvedAuth::Password { .. }) {
            return self.build_client().await;
        }

        let Some(token) = crate::porcelain::token_cache::read_cached_token(context_name) else {
            return self.build_client().await;
        };
        if !crate::porcelain::token_cache::is_token_fresh(&token, std::time::SystemTime::now()) {
            return self.build_client().await;
        }

        // Build an unauthenticated client and install the cached token.
        let mut builder = NifiClientBuilder::new(&self.url)?
            .danger_accept_invalid_certs(self.insecure)
            .version_strategy(self.version_strategy);
        if let Some(ca_path) = &self.ca_cert_path {
            let pem = std::fs::read(ca_path)?;
            builder = builder.add_root_certificate(&pem);
        }
        if let Some(chain) = &self.proxied_entities_chain {
            builder = builder.proxied_entities_chain(chain.clone());
        }

        let client = builder.build_dynamic()?;
        client.set_token(token).await;
        client.detect_version().await?;
        Ok(client)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a version strategy string ("strict" | "closest" | "latest").
pub fn parse_strategy(s: &str) -> Result<VersionResolutionStrategy, CliError> {
    match s {
        "strict" => Ok(VersionResolutionStrategy::Strict),
        "closest" => Ok(VersionResolutionStrategy::Closest),
        "latest" => Ok(VersionResolutionStrategy::Latest),
        other => Err(CliError::User(format!(
            "unknown version_strategy '{other}'; expected strict, closest, or latest"
        ))),
    }
}

/// Resolve `ResolvedAuth` from a context's `AuthConfig`, reading env vars where needed.
pub fn resolve_auth_from_context(ctx: &Context) -> Result<ResolvedAuth, CliError> {
    match &ctx.auth {
        AuthConfig::Password {
            username,
            password,
            password_env,
        } => {
            let resolved_password = if let Some(p) = password {
                Some(p.clone())
            } else if let Some(env_var) = password_env {
                // `password_env` is an explicit pointer at an env var —
                // if the env var is missing, surface that specific error
                // rather than silently falling through to a prompt.
                Some(std::env::var(env_var).map_err(|_| {
                    CliError::User(format!(
                        "env var '{env_var}' (password_env for context '{}') is not set",
                        ctx.name
                    ))
                })?)
            } else {
                // No password, no password_env — defer to build_client
                // which prompts on TTY or refuses off-TTY.
                None
            };
            Ok(ResolvedAuth::Password {
                username: username.clone(),
                password: resolved_password,
            })
        }
        AuthConfig::Token { token, token_env } => {
            let resolved_token = if let Some(t) = token {
                t.clone()
            } else if let Some(env_var) = token_env {
                std::env::var(env_var).map_err(|_| {
                    CliError::User(format!(
                        "env var '{env_var}' (token_env for context '{}') is not set",
                        ctx.name
                    ))
                })?
            } else {
                return Err(CliError::User(format!(
                    "context '{}' has token auth but neither 'token' nor 'token_env' is set",
                    ctx.name
                )));
            };
            Ok(ResolvedAuth::Token(resolved_token))
        }
        AuthConfig::Mtls {
            client_identity_path,
        } => Ok(ResolvedAuth::Mtls {
            identity_path: client_identity_path.clone(),
        }),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Serializes tests that mutate HOME so they don't race each other
    // under cargo's default parallel runner. `std::env::set_var` is
    // process-global; only build_client_with_cache tests mutate HOME.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn jwt_with_exp_in(seconds: u64) -> String {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let exp = now_secs + seconds;
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256"}"#);
        let payload = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{exp}}}"#));
        let sig = URL_SAFE_NO_PAD.encode(b"sig");
        format!("{header}.{payload}.{sig}")
    }

    // Held intentionally across awaits to serialize HOME mutation
    // against concurrent tests in this module.
    #[allow(clippy::await_holding_lock)]
    #[tokio::test]
    async fn cache_short_circuits_login_for_password_auth() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        let server = MockServer::start().await;
        // /access/token must NOT be hit — the cached token short-circuits login.
        Mock::given(method("POST"))
            .and(path("/nifi-api/access/token"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&server)
            .await;
        // detect_version() probes /flow/about.
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "about": {
                    "title": "NiFi",
                    "version": "2.9.0",
                    "uri": "https://localhost/nifi-api",
                    "contentViewerUrl": "",
                    "timezone": "UTC",
                    "buildTag": "",
                    "buildRevision": "",
                    "buildBranch": "",
                    "buildTimestamp": ""
                }
            })))
            .mount(&server)
            .await;

        // Set HOME to a tempdir for cache lookup.
        let tmp = tempfile::tempdir().expect("tempdir creation failed");
        // SAFETY: tests are serialized via ENV_LOCK; HOME mutation is
        // visible to the in-process file lookup only.
        unsafe {
            std::env::set_var("HOME", tmp.path());
        }
        let cache_dir = tmp.path().join(".nifictl/tokens");
        std::fs::create_dir_all(&cache_dir).expect("cache dir creation failed");
        let token = jwt_with_exp_in(3600);
        std::fs::write(cache_dir.join("test-ctx"), &token).expect("token write failed");

        let params = ResolvedParams {
            url: server.uri(),
            auth: ResolvedAuth::Password {
                username: "admin".into(),
                password: Some("ignored".into()),
            },
            insecure: false,
            ca_cert_path: None,
            proxied_entities_chain: None,
            version_strategy: VersionResolutionStrategy::default(),
        };

        let client = params
            .build_client_with_cache("test-ctx")
            .await
            .expect("build_client_with_cache failed");
        assert_eq!(client.token().await.as_deref(), Some(token.as_str()));
    }

    #[allow(clippy::await_holding_lock)]
    #[tokio::test]
    async fn cache_falls_back_on_stale_token() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/nifi-api/access/token"))
            .respond_with(ResponseTemplate::new(201).set_body_string("fresh-jwt"))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "about": {
                    "title": "NiFi",
                    "version": "2.9.0",
                    "uri": "https://localhost/nifi-api",
                    "contentViewerUrl": "",
                    "timezone": "UTC",
                    "buildTag": "",
                    "buildRevision": "",
                    "buildBranch": "",
                    "buildTimestamp": ""
                }
            })))
            .mount(&server)
            .await;

        let tmp = tempfile::tempdir().expect("tempdir creation failed");
        // SAFETY: tests are serialized via ENV_LOCK.
        unsafe {
            std::env::set_var("HOME", tmp.path());
        }
        let cache_dir = tmp.path().join(".nifictl/tokens");
        std::fs::create_dir_all(&cache_dir).expect("cache dir creation failed");
        // Token already expired by 60s.
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256"}"#);
        let payload = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{}}}"#, now_secs - 60));
        let sig = URL_SAFE_NO_PAD.encode(b"sig");
        let stale = format!("{header}.{payload}.{sig}");
        std::fs::write(cache_dir.join("stale-ctx"), &stale).expect("token write failed");

        let params = ResolvedParams {
            url: server.uri(),
            auth: ResolvedAuth::Password {
                username: "admin".into(),
                password: Some("hunter2".into()),
            },
            insecure: false,
            ca_cert_path: None,
            proxied_entities_chain: None,
            version_strategy: VersionResolutionStrategy::default(),
        };

        let client = params
            .build_client_with_cache("stale-ctx")
            .await
            .expect("build_client_with_cache failed");
        // build_client's Password path called login() and got "fresh-jwt".
        assert_eq!(client.token().await.as_deref(), Some("fresh-jwt"));
    }
}

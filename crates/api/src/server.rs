use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderValue, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use xenochat_common::{config::XenochatConfig, secrets::resolve_master_key};

use crate::{ApiService, Route};

const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60);
const RATE_LIMIT_MAX: usize = 120;

#[derive(Clone)]
pub struct ApiRuntime {
    config: XenochatConfig,
    resolved_api_keys: Arc<Vec<String>>,
    service: Arc<Mutex<ApiService>>,
    limiter: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
}

impl ApiRuntime {
    pub fn new(config: XenochatConfig, master_key: Option<&str>) -> Result<Self, String> {
        let resolved_api_keys = config
            .resolve_api_keys(master_key)
            .map_err(|error| format!("failed to resolve encrypted api keys: {error:?}"))?;
        let service = ApiService::new(config.clone());
        Ok(Self {
            config,
            resolved_api_keys: Arc::new(resolved_api_keys),
            service: Arc::new(Mutex::new(service)),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn with_service_mut<T>(&self, f: impl FnOnce(&mut ApiService) -> T) -> T {
        let mut service = self
            .service
            .lock()
            .expect("api service mutex poisoned while processing request");
        f(&mut service)
    }

    fn config(&self) -> &XenochatConfig {
        &self.config
    }

    fn resolved_api_keys(&self) -> &[String] {
        self.resolved_api_keys.as_ref()
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    reply: String,
}

#[derive(Debug, Serialize)]
struct ConfigResponse {
    host: String,
    port: u16,
    allowed_origins: usize,
    api_keys_configured: bool,
}

#[derive(Debug, Serialize)]
struct LogsResponse {
    records: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PluginsResponse {
    plugins: Vec<String>,
}

pub fn build_router(config: XenochatConfig) -> Result<Router, String> {
    let resolved = resolve_master_key().map_err(|error| {
        format!("failed to resolve master key from environment/keychain: {error:?}")
    })?;
    build_router_with_master(config, resolved.map(|item| item.value))
}

pub fn build_router_with_master(
    config: XenochatConfig,
    master_key: Option<String>,
) -> Result<Router, String> {
    let state = ApiRuntime::new(config, master_key.as_deref())?;

    let router = Router::new()
        .route("/health", get(health))
        .route("/api/v1/chat", post(chat))
        .route("/api/v1/config", get(config_snapshot))
        .route("/api/v1/logs", get(logs))
        .route("/api/v1/plugins", get(plugins))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            reject_query_token,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            enforce_rate_limit,
        ))
        .layer(middleware::from_fn_with_state(state.clone(), enforce_auth))
        .layer(middleware::from_fn_with_state(state.clone(), enforce_cors))
        .with_state(state);

    Ok(router)
}

pub async fn serve(config: XenochatConfig) -> Result<(), String> {
    config
        .validate()
        .map_err(|error| format!("invalid runtime config: {error:?}"))?;

    let bind = format!("{}:{}", config.api.host, config.api.port);
    let listener = TcpListener::bind(&bind)
        .await
        .map_err(|error| format!("failed to bind {bind}: {error}"))?;

    let router = build_router(config)?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|error| format!("api server terminated with error: {error}"))
}

async fn health(State(state): State<ApiRuntime>) -> impl IntoResponse {
    state.with_service_mut(|service| service.handle_route(Route::Health));

    let response = HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    };

    (StatusCode::OK, Json(response))
}

async fn chat(
    State(state): State<ApiRuntime>,
    Json(request): Json<ChatRequest>,
) -> impl IntoResponse {
    state.with_service_mut(|service| service.handle_route(Route::Chat));

    let reply = if request.message.trim().is_empty() {
        "Please provide a non-empty message.".to_owned()
    } else {
        format!("Xenochat received: {}", request.message.trim())
    };

    (StatusCode::OK, Json(ChatResponse { reply }))
}

async fn config_snapshot(State(state): State<ApiRuntime>) -> impl IntoResponse {
    state.with_service_mut(|service| service.handle_route(Route::Config));

    let config = state.config();
    (
        StatusCode::OK,
        Json(ConfigResponse {
            host: config.api.host.clone(),
            port: config.api.port,
            allowed_origins: config.api.allowed_origins.len(),
            api_keys_configured: !config.api.api_keys.is_empty(),
        }),
    )
}

async fn logs(State(state): State<ApiRuntime>) -> impl IntoResponse {
    state.with_service_mut(|service| service.handle_route(Route::Logs));

    (
        StatusCode::OK,
        Json(LogsResponse {
            records: vec!["runtime initialized".to_owned()],
        }),
    )
}

async fn plugins(State(state): State<ApiRuntime>) -> impl IntoResponse {
    state.with_service_mut(|service| service.handle_route(Route::Plugins));

    (
        StatusCode::OK,
        Json(PluginsResponse {
            plugins: vec!["core-safe-mode".to_owned()],
        }),
    )
}

async fn reject_query_token(
    State(_state): State<ApiRuntime>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if request
        .uri()
        .query()
        .map(|query| query.contains("access_token=") || query.contains("token="))
        .unwrap_or(false)
    {
        return (StatusCode::BAD_REQUEST, "query token is not supported").into_response();
    }

    next.run(request).await
}

async fn enforce_rate_limit(
    State(state): State<ApiRuntime>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let client = client_key(
        request.headers(),
        request.extensions().get::<SocketAddr>().copied(),
    );
    let now = Instant::now();

    {
        let mut limiter = state
            .limiter
            .lock()
            .expect("rate limiter mutex poisoned while processing request");

        let history = limiter.entry(client).or_default();
        while history
            .front()
            .map(|entry| now.duration_since(*entry) > RATE_LIMIT_WINDOW)
            .unwrap_or(false)
        {
            let _ = history.pop_front();
        }

        if history.len() >= RATE_LIMIT_MAX {
            return (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded").into_response();
        }

        history.push_back(now);
    }

    next.run(request).await
}

async fn enforce_auth(
    State(state): State<ApiRuntime>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }

    let keys = state.resolved_api_keys();
    if keys.is_empty() {
        return next.run(request).await;
    }

    let bearer = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    let Some(token) = bearer else {
        return (StatusCode::UNAUTHORIZED, "missing bearer token").into_response();
    };

    if !keys.iter().any(|key| key == token) {
        return (StatusCode::UNAUTHORIZED, "invalid bearer token").into_response();
    }

    next.run(request).await
}

async fn enforce_cors(
    State(state): State<ApiRuntime>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let origin = request
        .headers()
        .get("origin")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);

    let Some(origin_value) = origin else {
        return next.run(request).await;
    };

    let allowed = state
        .config()
        .api
        .allowed_origins
        .iter()
        .any(|entry| entry == &origin_value);

    if !allowed {
        return (StatusCode::FORBIDDEN, "origin is not allowed").into_response();
    }

    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        apply_cors_headers(response.headers_mut(), &origin_value);
        return response;
    }

    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), &origin_value);
    response
}

fn apply_cors_headers(headers: &mut HeaderMap, origin: &str) {
    headers.insert(
        "access-control-allow-origin",
        HeaderValue::from_str(origin)
            .unwrap_or_else(|_| HeaderValue::from_static("https://invalid-origin")),
    );
    headers.insert("vary", HeaderValue::from_static("Origin"));
    headers.insert(
        "access-control-allow-methods",
        HeaderValue::from_static("GET,POST,OPTIONS"),
    );
    headers.insert(
        "access-control-allow-headers",
        HeaderValue::from_static("Authorization,Content-Type"),
    );
    headers.insert(
        "access-control-allow-credentials",
        HeaderValue::from_static("true"),
    );
}

fn client_key(headers: &HeaderMap, socket_addr: Option<SocketAddr>) -> String {
    if let Some(forwarded) = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
    {
        let first = forwarded.split(',').next().unwrap_or("unknown").trim();
        if !first.is_empty() {
            return first.to_owned();
        }
    }

    if let Some(addr) = socket_addr {
        return addr.ip().to_string();
    }

    "local".to_owned()
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::util::ServiceExt;
    use xenochat_common::config::XenochatConfig;

    use crate::server::build_router_with_master;

    fn secured_config() -> XenochatConfig {
        let sealed =
            xenochat_common::crypto::seal_secret("secret-key", "unit-test-master").expect("seal");
        let mut config = XenochatConfig::default();
        config.api.api_keys = vec![sealed];
        config.api.allowed_origins = vec!["https://console.xenochat.local".to_owned()];
        config
    }

    #[tokio::test]
    async fn rejects_query_token() {
        let app = build_router_with_master(secured_config(), Some("unit-test-master".to_owned()))
            .expect("router");

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/config?access_token=abc")
                    .method(Method::GET)
                    .header("authorization", "Bearer secret-key")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn rejects_missing_bearer_on_protected_route() {
        let app = build_router_with_master(secured_config(), Some("unit-test-master".to_owned()))
            .expect("router");

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/plugins")
                    .method(Method::GET)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejects_unapproved_origin() {
        let app = build_router_with_master(secured_config(), Some("unit-test-master".to_owned()))
            .expect("router");

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/config")
                    .method(Method::GET)
                    .header("origin", "https://evil.example")
                    .header("authorization", "Bearer secret-key")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn accepts_valid_origin_and_bearer() {
        let app = build_router_with_master(secured_config(), Some("unit-test-master".to_owned()))
            .expect("router");

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/config")
                    .method(Method::GET)
                    .header("origin", "https://console.xenochat.local")
                    .header("authorization", "Bearer secret-key")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn health_is_public() {
        let app = build_router_with_master(secured_config(), Some("unit-test-master".to_owned()))
            .expect("router");

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .method(Method::GET)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, warn};

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    upstream: String,
    token_cache: Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
}

#[derive(Deserialize)]
struct TokenResponse {
    token: Option<String>,
    access_token: Option<String>,
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("docker_proxy=debug,tower_http=debug")
        .init();

    let upstream = std::env::var("UPSTREAM_REGISTRY")
        .unwrap_or_else(|_| "https://registry-1.docker.io".to_string());

    info!("Using upstream registry: {}", upstream);

    let state = AppState {
        client: reqwest::Client::builder()
            .build()
            .expect("Failed to create HTTP client"),
        upstream,
        token_cache: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
    };

    let app = Router::new()
        .route("/v2/", get(handle_version))
        .route("/v2/*path", get(proxy_request).head(proxy_request))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    let addr = "0.0.0.0:8080";
    info!("Docker registry proxy listening on {}", addr);
    info!("Usage: docker pull 127.0.0.1:8080/library/ubuntu:latest");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// 处理 /v2/ 版本检查
async fn handle_version() -> impl IntoResponse {
    info!("Version check requested");
    (
        StatusCode::OK,
        [("Docker-Distribution-API-Version", "registry/2.0")],
        "{}",
    )
}

// 代理所有其他请求到上游 registry
async fn proxy_request(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Response, AppError> {
    let path = request.uri().path();
    let query = request.uri().query().unwrap_or("");
    
    info!("Proxying request: {} {}", request.method(), path);

    let processed_path = path.to_string();

    // 构建上游 URL
    let upstream_url = if query.is_empty() {
        format!("{}{}", state.upstream, processed_path)
    } else {
        format!("{}{}?{}", state.upstream, processed_path, query)
    };

    info!("Upstream URL: {}", upstream_url);

    // 准备请求头
    let mut headers = build_headers(&request);

    // 提取镜像名称用于 token 获取
    let image_name = extract_image_name(&processed_path);
    
    // 检查是否已有 token
    let token = state.token_cache.read().await.get(&image_name).cloned();
    if let Some(ref token_value) = token {
        debug!("Using cached token for {}", image_name);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );
    }

    // 发送请求到上游
    let method = convert_method(request.method());
    
    let mut response = state
        .client
        .request(method.clone(), &upstream_url)
        .headers(headers.clone())
        .send()
        .await
        .map_err(|e| {
            warn!("Failed to proxy request: {}", e);
            AppError::UpstreamError(e.to_string())
        })?;

    info!("Upstream response status: {}", response.status());

    // 处理 401 - 需要认证
    if response.status().as_u16() == 401 {
        info!("Got 401, attempting to get anonymous token");
        
        // 解析 WWW-Authenticate header
        if let Some(auth_header) = response.headers().get("www-authenticate") {
            if let Ok(auth_str) = auth_header.to_str() {
                debug!("WWW-Authenticate: {}", auth_str);
                
                // 尝试获取匿名 token
                if let Ok(new_token) = get_anonymous_token(&state.client, auth_str, &image_name).await {
                    // 缓存 token
                    state.token_cache.write().await.insert(image_name.clone(), new_token.clone());
                    
                    // 使用新 token 重试
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
                    );
                    
                    response = state
                        .client
                        .request(method, &upstream_url)
                        .headers(headers)
                        .send()
                        .await
                        .map_err(|e| {
                            warn!("Failed to retry request: {}", e);
                            AppError::UpstreamError(e.to_string())
                        })?;
                    
                    info!("Retry response status: {}", response.status());
                }
            }
        }
    }

    // 构建响应
    build_response(response).await
}

// 构建请求头
fn build_headers(request: &Request) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    for (key, value) in request.headers().iter() {
        let key_str = key.as_str().to_lowercase();
        if key_str != "host" && key_str != "connection" && key_str != "content-length" {
            if let Ok(name) = reqwest::header::HeaderName::from_bytes(key.as_str().as_bytes()) {
                if let Ok(val) = reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
                    headers.insert(name, val);
                }
            }
        }
    }

    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("Docker-Client/24.0.0 (linux)"),
    );
    
    headers
}

// 转换 HTTP 方法
fn convert_method(method: &axum::http::Method) -> reqwest::Method {
    match method {
        &axum::http::Method::GET => reqwest::Method::GET,
        &axum::http::Method::HEAD => reqwest::Method::HEAD,
        _ => reqwest::Method::GET,
    }
}

// 提取镜像名称
fn extract_image_name(path: &str) -> String {
    // 从路径中提取镜像名称，例如 /v2/library/ubuntu/manifests/latest -> library/ubuntu
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 3 && parts[1] == "v2" {
        // 找到 manifests 或 blobs 之前的部分
        let mut name_parts = Vec::new();
        for part in parts.iter().skip(2) {
            if *part == "manifests" || *part == "blobs" || *part == "tags" {
                break;
            }
            name_parts.push(*part);
        }
        return name_parts.join("/");
    }
    "unknown".to_string()
}

// 获取匿名 token
async fn get_anonymous_token(
    client: &reqwest::Client,
    auth_header: &str,
    image_name: &str,
) -> Result<String, AppError> {
    // 解析 WWW-Authenticate header
    // 格式: Bearer realm="https://auth.docker.io/token",service="registry.docker.io",scope="repository:library/ubuntu:pull"
    
    let realm = extract_param(auth_header, "realm");
    let service = extract_param(auth_header, "service");
    let scope = format!("repository:{}:pull", image_name);
    
    if realm.is_empty() || service.is_empty() {
        warn!("Failed to parse auth header: {}", auth_header);
        return Err(AppError::UpstreamError("Invalid auth header".to_string()));
    }
    
    let token_url = format!("{}?service={}&scope={}", 
        realm.trim_matches('"'),
        service.trim_matches('"'),
        urlencoding::encode(&scope)
    );
    
    debug!("Requesting token from: {}", token_url);
    
    let response = client
        .get(&token_url)
        .send()
        .await
        .map_err(|e| AppError::UpstreamError(format!("Token request failed: {}", e)))?;
    
    if !response.status().is_success() {
        warn!("Token request failed with status: {}", response.status());
        return Err(AppError::UpstreamError("Token request failed".to_string()));
    }
    
    let token_resp: TokenResponse = response
        .json()
        .await
        .map_err(|e| AppError::UpstreamError(format!("Token parse failed: {}", e)))?;
    
    let token = token_resp.token.or(token_resp.access_token)
        .ok_or_else(|| AppError::UpstreamError("No token in response".to_string()))?;
    
    info!("Successfully obtained anonymous token for {}", image_name);
    Ok(token)
}

// 从认证头中提取参数
fn extract_param(auth_header: &str, param: &str) -> String {
    let param_prefix = format!("{}=", param);
    if let Some(start) = auth_header.find(&param_prefix) {
        let value_start = start + param_prefix.len();
        let rest = &auth_header[value_start..];
        
        // 处理引号包围的值
        if rest.starts_with('"') {
            if let Some(end) = rest[1..].find('"') {
                return rest[1..end + 1].to_string();
            }
        } else {
            // 没有引号，找到逗号或结尾
            if let Some(end) = rest.find(',') {
                return rest[..end].to_string();
            } else {
                return rest.to_string();
            }
        }
    }
    String::new()
}

// 构建响应
async fn build_response(response: reqwest::Response) -> Result<Response, AppError> {
    let status_code = response.status().as_u16();
    let status = StatusCode::from_u16(status_code)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut response_builder = Response::builder().status(status);
    
    for (key, value) in response.headers().iter() {
        let key_str = key.as_str().to_lowercase();
        if key_str != "transfer-encoding" && key_str != "connection" {
            if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_str().as_bytes()) {
                if let Ok(header_value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
                    response_builder = response_builder.header(header_name, header_value);
                }
            }
        }
    }

    let stream = response.bytes_stream();
    let body = Body::from_stream(stream);

    Ok(response_builder.body(body).unwrap())
}

// 错误处理
#[derive(Debug)]
enum AppError {
    UpstreamError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UpstreamError(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        (status, message).into_response()
    }
}


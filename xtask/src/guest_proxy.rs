use axum::{
    Router,
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct ProxyParams {
    url: String,
}

pub fn run(port: u16) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_run(port))
}

async fn async_run(port: u16) -> anyhow::Result<()> {
    let app = Router::new().route("/", get(proxy_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("[GUEST-PROXY] HTTP-to-HTTPS Proxy running on port {}", port);
    println!(
        "[GUEST-PROXY] Guest should use: http://10.0.2.2:{}/?url=https://...",
        port
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn proxy_handler(Query(params): Query<ProxyParams>) -> Response {
    println!("[PROXY] Fetching: {}", params.url);

    let client = reqwest::Client::builder()
        .user_agent("ThingOS-Proxy/1.0")
        .build()
        .unwrap_or_default();

    match client.get(&params.url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let headers = resp.headers().clone();
            let body = match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    return (
                        StatusCode::BAD_GATEWAY,
                        format!("Failed to read body: {}", e),
                    )
                        .into_response();
                }
            };

            println!("[PROXY] OK: {} bytes", body.len());

            let mut response_headers = HeaderMap::new();
            if let Some(ct) = headers.get(reqwest::header::CONTENT_TYPE) {
                response_headers.insert(reqwest::header::CONTENT_TYPE, ct.clone());
            }
            // Handling header value conversion safely
            if let Ok(val) = params.url.parse() {
                response_headers.insert("X-Original-URL", val);
            }

            (status, response_headers, body).into_response()
        }
        Err(e) => {
            println!("[PROXY] Error: {}", e);
            (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)).into_response()
        }
    }
}

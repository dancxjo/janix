use crate::common::project_root;
use anyhow::{Context, Result};
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::any,
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

struct ProxyState {
    target_port: u16,
    client: reqwest::Client,
}

pub fn https_proxy(https_port: u16, target_port: u16) -> Result<()> {
    let cert_dir = project_root().join("target").join(".certs");
    let cert_file = cert_dir.join("server.crt");
    let key_file = cert_dir.join("server.key");

    ensure_certificates(&cert_dir, &cert_file, &key_file)?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_run(https_port, target_port, cert_file, key_file))
}

fn ensure_certificates(cert_dir: &PathBuf, cert_file: &PathBuf, key_file: &PathBuf) -> Result<()> {
    if cert_file.exists() && key_file.exists() {
        println!("[HTTPS-PROXY] Using existing certificates");
        return Ok(());
    }

    std::fs::create_dir_all(cert_dir).context("Failed to create cert dir")?;
    println!("[HTTPS-PROXY] Generating self-signed certificates...");

    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names)?;

    std::fs::write(cert_file, cert.serialize_pem()?)?;
    std::fs::write(key_file, cert.serialize_private_key_pem())?;

    println!("[HTTPS-PROXY] Generated certificates");
    Ok(())
}

async fn async_run(
    port: u16,
    target_port: u16,
    cert_file: PathBuf,
    key_file: PathBuf,
) -> Result<()> {
    let config = RustlsConfig::from_pem_file(cert_file, key_file).await?;

    let state = Arc::new(ProxyState {
        target_port,
        client: reqwest::Client::new(),
    });

    let app = Router::new()
        .route("/*path", any(proxy_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!(
        "[HTTPS-PROXY] Starting HTTPS reverse proxy on port {}",
        port
    );
    println!("[HTTPS-PROXY] Target: http://localhost:{}", target_port);
    println!("[HTTPS-PROXY] Browser access: https://localhost:{}/", port);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn proxy_handler(
    State(state): State<Arc<ProxyState>>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = uri.path();
    let path_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(path);

    let target_uri = format!("http://localhost:{}{}", state.target_port, path_query);

    let resp = state
        .client
        .request(method, &target_uri)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let status = resp.status();
    let resp_headers = resp.headers().clone();
    let resp_body = resp
        .bytes()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut response = Response::new(Body::from(resp_body));
    *response.status_mut() = status;
    *response.headers_mut() = resp_headers;

    Ok(response)
}

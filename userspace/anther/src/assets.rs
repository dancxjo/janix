//! Static asset serving for anther
//!
//! Embeds explorer assets at compile time and serves them as static files.

/// Embedded asset data
pub struct Asset {
    pub content: &'static [u8],
    pub content_type: &'static str,
}

/// Get embedded asset by path
pub fn get_asset(path: &str) -> Option<Asset> {
    match path {
        "/" | "/index.html" => Some(Asset {
            content: include_bytes!("../assets/graph/graph.html"),
            content_type: "text/html; charset=utf-8",
        }),
        "/explorer.html" => Some(Asset {
            content: include_bytes!("../assets/explorer/index.html"),
            content_type: "text/html; charset=utf-8",
        }),
        "/explorer.js" => Some(Asset {
            content: include_bytes!("../assets/explorer/explorer.js"),
            content_type: "application/javascript; charset=utf-8",
        }),
        "/explorer.css" => Some(Asset {
            content: include_bytes!("../assets/explorer/explorer.css"),
            content_type: "text/css; charset=utf-8",
        }),
        // Graph viewer assets
        "/graph.js" => Some(Asset {
            content: include_bytes!("../assets/graph/graph.js"),
            content_type: "application/javascript; charset=utf-8",
        }),
        "/graph.css" => Some(Asset {
            content: include_bytes!("../assets/graph/graph.css"),
            content_type: "text/css; charset=utf-8",
        }),
        "/elk-worker.js" => Some(Asset {
            content: include_bytes!("../assets/graph/elk-worker.js"),
            content_type: "application/javascript; charset=utf-8",
        }),
        _ => None,
    }
}

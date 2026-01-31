//! API v1 Router
//!
//! Routes requests to /api/v1/... endpoints.

extern crate alloc;

use alloc::string::String;

use crate::http::Method;

/// Parsed API route
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiRoute<'a> {
    /// GET /api/v1/
    Discovery,
    
    /// GET /api/v1/things/{id}
    GetThing { id: &'a str },
    
    /// POST /api/v1/things
    CreateThing,
    
    /// DELETE /api/v1/things/{id}
    DeleteThing { id: &'a str },
    
    /// PATCH /api/v1/things/{id}
    PatchThing { id: &'a str },
    
    /// GET /api/v1/things/{id}/props
    GetThingProps { id: &'a str },
    
    /// GET /api/v1/things/{id}/bytespaces/{key}
    GetBytespace { thing_id: &'a str, key: &'a str },
    
    /// GET /api/v1/things/{id}/bytespaces/{key}/meta
    GetBytespaceMetadata { thing_id: &'a str, key: &'a str },
    
    /// PUT /api/v1/things/{id}/bytespaces/{key}
    PutBytespace { thing_id: &'a str, key: &'a str },
    
    /// GET /api/v1/path/{path}
    ResolvePath { path: &'a str },
    
    /// GET /api/v1/watch
    Watch,
    
    /// GET /api/v1/subgraph?root=...&depth=...
    GetSubgraph { query: &'a str },
    
    /// PATCH /api/v1/layout
    PatchLayout,
    
    /// Route not found in API
    NotFound,
    
    /// Method not allowed for this route
    MethodNotAllowed,
}

/// Match a request path and method to an API route
pub fn match_route<'a>(method: Method, path: &'a str) -> Option<ApiRoute<'a>> {
    // Must start with /api/v1
    let rest = path.strip_prefix("/api/v1")?;
    
    // Handle empty path after prefix or just "/"
    if rest.is_empty() || rest == "/" {
        return match method {
            Method::Get => Some(ApiRoute::Discovery),
            _ => Some(ApiRoute::MethodNotAllowed),
        };
    }
    
    // Split into segments (strip query string first)
    let path_part = rest.split('?').next().unwrap_or(rest);
    let segments: alloc::vec::Vec<&str> = path_part
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    
    match (method, segments.as_slice()) {
        // /api/v1/things
        (Method::Post, ["things"]) => Some(ApiRoute::CreateThing),
        (Method::Get, ["things"]) => Some(ApiRoute::MethodNotAllowed), // List not yet supported
        
        // /api/v1/things/{id}
        (Method::Get, ["things", id]) => Some(ApiRoute::GetThing { id }),
        (Method::Delete, ["things", id]) => Some(ApiRoute::DeleteThing { id }),
        (Method::Patch, ["things", id]) => Some(ApiRoute::PatchThing { id }),
        
        // /api/v1/things/{id}/props
        (Method::Get, ["things", id, "props"]) => Some(ApiRoute::GetThingProps { id }),
        
        // /api/v1/things/{id}/bytespaces/{key}
        (Method::Get, ["things", thing_id, "bytespaces", key]) => {
            Some(ApiRoute::GetBytespace { thing_id, key })
        }
        (Method::Put, ["things", thing_id, "bytespaces", key]) => {
            Some(ApiRoute::PutBytespace { thing_id, key })
        }
        
        // /api/v1/things/{id}/bytespaces/{key}/meta
        (Method::Get, ["things", thing_id, "bytespaces", key, "meta"]) => {
            Some(ApiRoute::GetBytespaceMetadata { thing_id, key })
        }
        
        // /api/v1/path/{path...}
        (Method::Get, ["path", rest @ ..]) if !rest.is_empty() => {
            // Reconstruct the path from remaining segments
            // Note: This loses the original path encoding, but for now it works
            let full_path = &path["/api/v1/path".len()..];
            Some(ApiRoute::ResolvePath { path: full_path })
        }
        
        // /api/v1/watch
        (Method::Get, ["watch"]) => Some(ApiRoute::Watch),
        
        // /api/v1/subgraph?root=...&depth=...
        (Method::Get, ["subgraph"]) => {
            // Pass the query string portion (after ?) to the handler
            let query = path.find('?').map(|i| &path[i+1..]).unwrap_or("");
            Some(ApiRoute::GetSubgraph { query })
        }
        
        // /api/v1/layout
        (Method::Patch, ["layout"]) => Some(ApiRoute::PatchLayout),
        
        // Method not allowed variants
        (_, ["things"]) |
        (_, ["things", _]) |
        (_, ["things", _, "props"]) |
        (_, ["things", _, "bytespaces", _]) |
        (_, ["things", _, "bytespaces", _, "meta"]) |
        (_, ["watch"]) |
        (_, ["subgraph"]) |
        (_, ["layout"]) => Some(ApiRoute::MethodNotAllowed),
        
        // Not found
        _ => Some(ApiRoute::NotFound),
    }
}

/// Check if a path is an API v1 path
pub fn is_api_v1_path(path: &str) -> bool {
    path.starts_with("/api/v1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_route() {
        assert_eq!(match_route(Method::Get, "/api/v1/"), Some(ApiRoute::Discovery));
        assert_eq!(match_route(Method::Get, "/api/v1"), Some(ApiRoute::Discovery));
    }

    #[test]
    fn test_thing_routes() {
        assert_eq!(
            match_route(Method::Get, "/api/v1/things/123"),
            Some(ApiRoute::GetThing { id: "123" })
        );
        assert_eq!(
            match_route(Method::Post, "/api/v1/things"),
            Some(ApiRoute::CreateThing)
        );
        assert_eq!(
            match_route(Method::Patch, "/api/v1/things/456"),
            Some(ApiRoute::PatchThing { id: "456" })
        );
    }

    #[test]
    fn test_bytespace_routes() {
        assert_eq!(
            match_route(Method::Get, "/api/v1/things/1/bytespaces/content"),
            Some(ApiRoute::GetBytespace { thing_id: "1", key: "content" })
        );
        assert_eq!(
            match_route(Method::Get, "/api/v1/things/1/bytespaces/content/meta"),
            Some(ApiRoute::GetBytespaceMetadata { thing_id: "1", key: "content" })
        );
    }

    #[test]
    fn test_non_api_path() {
        assert_eq!(match_route(Method::Get, "/health"), None);
        assert_eq!(match_route(Method::Get, "/graph/123"), None);
    }
}

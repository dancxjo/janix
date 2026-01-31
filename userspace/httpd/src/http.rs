//! HTTP/1.1 request parsing module
//!
//! Minimal HTTP request parser with defensive limits.

#![allow(dead_code)]

const MAX_REQUEST_LINE: usize = 8192;
const MAX_HEADER_LINE: usize = 8192;
const MAX_HEADERS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub version: HttpVersion,
    pub headers: [(Option<&'a str>, Option<&'a str>); MAX_HEADERS],
    pub header_count: usize,
}

impl<'a> Request<'a> {
    pub fn get_header(&self, name: &str) -> Option<&'a str> {
        for i in 0..self.header_count {
            if let (Some(key), Some(value)) = (self.headers[i].0, self.headers[i].1) {
                if key.eq_ignore_ascii_case(name) {
                    return Some(value);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum ParseError {
    TooLong,
    InvalidMethod,
    InvalidVersion,
    MalformedRequestLine,
    TooManyHeaders,
    MalformedHeader,
}

pub fn parse_method(s: &str) -> Result<Method, ParseError> {
    match s {
        "GET" => Ok(Method::Get),
        "HEAD" => Ok(Method::Head),
        "POST" => Ok(Method::Post),
        _ => Ok(Method::Other),
    }
}

pub fn parse_version(s: &str) -> Result<HttpVersion, ParseError> {
    match s {
        "HTTP/1.0" => Ok(HttpVersion::Http10),
        "HTTP/1.1" => Ok(HttpVersion::Http11),
        _ => Err(ParseError::InvalidVersion),
    }
}

/// Parse a single HTTP request from a buffer
pub fn parse_request(buf: &str) -> Result<Request<'_>, ParseError> {
    let mut lines = buf.lines();
    
    // Parse request line
    let request_line = lines.next().ok_or(ParseError::MalformedRequestLine)?;
    if request_line.len() > MAX_REQUEST_LINE {
        return Err(ParseError::TooLong);
    }
    
    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().ok_or(ParseError::MalformedRequestLine)?;
    let path = parts.next().ok_or(ParseError::MalformedRequestLine)?;
    let version_str = parts.next().ok_or(ParseError::MalformedRequestLine)?;
    
    let method = parse_method(method_str)?;
    let version = parse_version(version_str)?;
    
    // Parse headers
    let mut headers = [(None, None); MAX_HEADERS];
    let mut header_count = 0;
    
    for line in lines {
        if line.is_empty() {
            // End of headers
            break;
        }
        
        if line.len() > MAX_HEADER_LINE {
            return Err(ParseError::TooLong);
        }
        
        if header_count >= MAX_HEADERS {
            return Err(ParseError::TooManyHeaders);
        }
        
        // Parse "Key: Value"
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();
            headers[header_count] = (Some(key), Some(value));
            header_count += 1;
        } else {
            return Err(ParseError::MalformedHeader);
        }
    }
    
    Ok(Request {
        method,
        path,
        version,
        headers,
        header_count,
    })
}

/// Decode percent-encoded path components
/// Returns None if path contains dangerous sequences like ".."
pub fn decode_path(path: &str) -> Option<&str> {
    // Simple validation: reject paths with ".."
    if path.contains("..") {
        return None;
    }
    
    // For now, we don't do full percent decoding to keep it simple
    // and avoid allocations. Just validate and return the path.
    // A full implementation would decode %XX sequences.
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_get() {
        let input = "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let req = parse_request(input).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, "/health");
        assert_eq!(req.version, HttpVersion::Http11);
        assert_eq!(req.header_count, 1);
        assert_eq!(req.get_header("Host"), Some("localhost"));
    }

    #[test]
    fn test_parse_multiple_headers() {
        let input = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
        let req = parse_request(input).unwrap();
        assert_eq!(req.header_count, 2);
        assert_eq!(req.get_header("Host"), Some("localhost"));
        assert_eq!(req.get_header("Connection"), Some("close"));
    }

    #[test]
    fn test_parse_head_request() {
        let input = "HEAD /graph HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let req = parse_request(input).unwrap();
        assert_eq!(req.method, Method::Head);
        assert_eq!(req.path, "/graph");
    }

    #[test]
    fn test_invalid_version() {
        let input = "GET / HTTP/2.0\r\n\r\n";
        assert!(matches!(parse_request(input), Err(ParseError::InvalidVersion)));
    }

    #[test]
    fn test_decode_path_rejects_dotdot() {
        assert_eq!(decode_path("/safe/path"), Some("/safe/path"));
        assert_eq!(decode_path("/../etc/passwd"), None);
        assert_eq!(decode_path("/foo/../bar"), None);
    }

    #[test]
    fn test_case_insensitive_headers() {
        let input = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let req = parse_request(input).unwrap();
        assert_eq!(req.get_header("host"), Some("localhost"));
        assert_eq!(req.get_header("HOST"), Some("localhost"));
    }
}

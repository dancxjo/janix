//! HTTP/1.1 request parsing module
//!
//! Minimal HTTP request parser with defensive limits.

#![allow(dead_code)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

const MAX_REQUEST_LINE: usize = 8192;
const MAX_HEADER_LINE: usize = 8192;
const MAX_HEADERS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
}

pub enum ResponseBody {
    Static(&'static [u8]),
    Owned(Vec<u8>),
    Stream(Box<dyn llm::ChatStream + Send>),
}

impl ResponseBody {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ResponseBody::Static(s) => s,
            ResponseBody::Owned(o) => o.as_slice(),
            ResponseBody::Stream(_) => &[],
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            ResponseBody::Static(s) => Some(s.len()),
            ResponseBody::Owned(o) => Some(o.len()),
            ResponseBody::Stream(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub version: HttpVersion,
    pub headers: [(Option<&'a str>, Option<&'a str>); MAX_HEADERS],
    pub header_count: usize,
    pub header_len: usize,
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

    pub fn is_keep_alive(&self) -> bool {
        match self.get_header("Connection") {
            Some(v) => v.eq_ignore_ascii_case("keep-alive"),
            None => self.version == HttpVersion::Http11,
        }
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
        "PUT" => Ok(Method::Put),
        "PATCH" => Ok(Method::Patch),
        "DELETE" => Ok(Method::Delete),
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
    if parts.next().is_some() {
        return Err(ParseError::MalformedRequestLine);
    }

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

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();
            headers[header_count] = (Some(key), Some(value));
            header_count += 1;
        } else {
            return Err(ParseError::MalformedHeader);
        }
    }

    // Calculate header length (offset to end of \r\n\r\n)
    let header_len = if let Some(pos) = buf.find("\r\n\r\n") {
        pos + 4
    } else if let Some(pos) = buf.find("\n\n") {
        pos + 2
    } else {
        buf.len()
    };

    Ok(Request {
        method,
        path,
        version,
        headers,
        header_count,
        header_len,
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

/// Parse Content-Length header value
pub fn parse_content_length(req: &Request<'_>) -> Option<usize> {
    req.get_header("Content-Length")
        .and_then(|v| v.parse::<usize>().ok())
}

/// Parsed byte range from Range header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: usize,
    pub end: Option<usize>, // None means "to end of content"
}

/// Parse Range header (supports single byte range only)
/// Format: "bytes=start-end" or "bytes=start-"
pub fn parse_range_header(req: &Request<'_>) -> Option<ByteRange> {
    let value = req.get_header("Range")?;

    // Must start with "bytes="
    let rest = value.strip_prefix("bytes=")?;

    // Find the hyphen separator
    let hyphen_pos = rest.find('-')?;

    // Parse start
    let start_str = &rest[..hyphen_pos];
    let start: usize = start_str.parse().ok()?;

    // Parse end (optional)
    let end_str = &rest[hyphen_pos + 1..];
    let end = if end_str.is_empty() {
        None
    } else {
        Some(end_str.parse().ok()?)
    };

    Some(ByteRange { start, end })
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
        assert_eq!(req.header_len, 42); // "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n".len()
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
        assert!(matches!(
            parse_request(input),
            Err(ParseError::InvalidVersion)
        ));
    }

    #[test]
    fn test_reject_extra_request_line_tokens() {
        let input = "GET / HTTP/1.1 extra\r\nHost: localhost\r\n\r\n";
        assert!(matches!(
            parse_request(input),
            Err(ParseError::MalformedRequestLine)
        ));
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

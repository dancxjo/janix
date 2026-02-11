use core::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct LogParts<'a> {
    pub time: Option<u64>,
    pub level: Option<&'a str>,
    pub source: Option<&'a str>,
    pub message: &'a str,
}

pub fn parse_log_line(line: &str) -> LogParts<'_> {
    let mut rest = line.trim();

    // Parse time
    let time = if rest.starts_with('[') {
        if let Some(end_idx) = rest.find(']') {
            let content = &rest[1..end_idx];
            if let Ok(t) = u64::from_str(content) {
                rest = rest[end_idx + 1..].trim();
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // If time was found, we proceed to look for level.
    let level = if time.is_some() && rest.starts_with('[') {
        if let Some(end_idx) = rest.find(']') {
            let content = &rest[1..end_idx];
            rest = rest[end_idx + 1..].trim();
            Some(content)
        } else {
            None
        }
    } else {
        None
    };

    // If level was found, we proceed to look for source.
    let source = if level.is_some() && rest.starts_with('[') {
        if let Some(end_idx) = rest.find(']') {
            let content = &rest[1..end_idx];
            rest = rest[end_idx + 1..].trim();
            Some(content)
        } else {
            None
        }
    } else {
        None
    };

    LogParts {
        time,
        level,
        source,
        message: rest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let line = "[12345] [INFO] [kernel::main] Hello World";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: Some(12345),
                level: Some("INFO"),
                source: Some("kernel::main"),
                message: "Hello World"
            }
        );
    }

    #[test]
    fn test_missing_brackets() {
        let line = "Just a message";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: None,
                level: None,
                source: None,
                message: "Just a message"
            }
        );
    }

    #[test]
    fn test_non_numeric_time() {
        let line = "[abc] [INFO] [kernel] Message";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: None,
                level: None,
                source: None,
                message: "[abc] [INFO] [kernel] Message"
            }
        );
    }

    #[test]
    fn test_extra_spaces() {
        let line = "  [100]  [DEBUG]  [source]    Message   ";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: Some(100),
                level: Some("DEBUG"),
                source: Some("source"),
                message: "Message"
            }
        );
    }

    #[test]
    fn test_malformed_time_bracket() {
        let line = "[123 [INFO] [source] msg";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: None,
                level: None,
                source: None,
                message: "[123 [INFO] [source] msg"
            }
        );
    }

    #[test]
    fn test_complex_source() {
        let line = "[999] [WARN] [a::b::c] Complex source";
        let parts = parse_log_line(line);
        assert_eq!(
            parts,
            LogParts {
                time: Some(999),
                level: Some("WARN"),
                source: Some("a::b::c"),
                message: "Complex source"
            }
        );
    }
}

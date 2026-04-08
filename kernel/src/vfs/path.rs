//! Path resolution engine for the VFS.
//!
//! Implements iterative resolution of absolute paths with support for:
//! - Multi-component paths: `/a/b/c`
//! - Current-directory component (`.`): ignored
//! - Parent-directory component (`..`): pops the last resolved component
//! - Mount-point crossings: delegated to the global mount table
//!
//! # Design
//! Rather than building a full in-kernel `dentry` cache, this engine
//! normalises the path string first and then passes the cleaned path to
//! [`crate::vfs::mount::lookup`].  This is intentionally simple for ACT III:
//! a richer cache can be layered in later.
//!
//! The public entry point is [`resolve`].

use abi::errors::{Errno, SysResult};
use alloc::string::String;

/// Maximum number of components allowed in a path before returning `ENAMETOOLONG`.
const MAX_COMPONENTS: usize = 64;

/// Resolve an absolute path to a VFS node.
///
/// Normalises `path` (collapsing `.` and `..` components) and delegates
/// to the mount table for the final lookup.
///
/// # Errors
/// - `EINVAL`  — `path` is not absolute (does not start with `/`).
/// - `ENAMETOOLONG` — too many path components.
/// - `ENOENT`  — the path does not resolve to any mounted node.
pub fn resolve(path: &str) -> SysResult<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    let normalised = normalise(path)?;
    crate::vfs::mount::lookup(&normalised)
}

/// Normalise an absolute path, resolving `.` and `..` components.
///
/// Returns the canonical absolute path string.
///
/// # Examples
/// - `/a/./b` → `/a/b`
/// - `/a/b/../c` → `/a/c`
/// - `/a/b/../../c` → `/c`
/// - `/../..` → `/` (cannot go above root)
pub fn normalise(path: &str) -> SysResult<String> {
    if !path.starts_with('/') {
        return Err(Errno::EINVAL);
    }

    let mut components: alloc::vec::Vec<&str> = alloc::vec::Vec::new();

    for component in path.split('/') {
        match component {
            "" | "." => {
                // Skip empty segments (consecutive slashes) and current-dir.
            }
            ".." => {
                // Go up — ignore if already at root.
                components.pop();
            }
            name => {
                if components.len() >= MAX_COMPONENTS {
                    return Err(Errno::ENAMETOOLONG);
                }
                components.push(name);
            }
        }
    }

    if components.is_empty() {
        return Ok(String::from("/"));
    }

    // Exact capacity: one '/' per component plus each component's length.
    let capacity: usize = components.iter().map(|c| c.len() + 1).sum();
    let mut result = String::with_capacity(capacity);
    for c in &components {
        result.push('/');
        result.push_str(c);
    }
    Ok(result)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        assert_eq!(normalise("/a/b/c").unwrap(), "/a/b/c");
    }

    #[test]
    fn test_dot_components_removed() {
        assert_eq!(normalise("/a/./b").unwrap(), "/a/b");
        assert_eq!(normalise("/./a").unwrap(), "/a");
    }

    #[test]
    fn test_dot_dot_goes_up() {
        assert_eq!(normalise("/a/b/../c").unwrap(), "/a/c");
        assert_eq!(normalise("/a/b/../../c").unwrap(), "/c");
    }

    #[test]
    fn test_dot_dot_at_root_stays() {
        assert_eq!(normalise("/../..").unwrap(), "/");
        assert_eq!(normalise("/..").unwrap(), "/");
    }

    #[test]
    fn test_root_path() {
        assert_eq!(normalise("/").unwrap(), "/");
    }

    #[test]
    fn test_trailing_slash() {
        assert_eq!(normalise("/a/b/").unwrap(), "/a/b");
    }

    #[test]
    fn test_double_slash() {
        assert_eq!(normalise("//a//b").unwrap(), "/a/b");
    }

    #[test]
    fn test_relative_path_returns_einval() {
        assert!(matches!(normalise("relative/path"), Err(Errno::EINVAL)));
        assert!(matches!(normalise(""), Err(Errno::EINVAL)));
    }

    #[test]
    fn test_complex_normalisation() {
        assert_eq!(normalise("/a/b/c/../../d").unwrap(), "/a/d");
        assert_eq!(normalise("/a/./b/./c").unwrap(), "/a/b/c");
    }
}

//! Service Contract Schema
//!
//! This module defines the canonical, machine-readable contract for every long-running
//! service in Thing-OS. The contract formalizes what the system already believes:
//! - Services are graph-native watchers, not boot-time scanners
//! - The graph is the only source of truth
//! - Services declare their dependencies and outputs explicitly
//!
//! ## Contract Fields
//!
//! - **Service Name**: Canonical name of the service
//! - **Watched Kinds**: Node kinds this service watches (input)
//! - **Published Kinds**: Node kinds this service publishes (output)
//! - **Published Properties**: Property keys this service sets
//! - **Idempotent**: Whether repeated operations produce the same result
//! - **Boot Assumptions**: MUST be empty for graph-native services
//!
//! ## Runtime Enforcement
//!
//! Services MUST:
//! 1. Declare their contract before startup
//! 2. Register their contract node in the graph at `/sys/services/{name}`
//! 3. Only watch declared kinds
//! 4. Only publish declared kinds and properties
//!
//! ## Example Contract
//!
//! ```rust,ignore
//! ServiceContract {
//!     name: "ingestd",
//!     watched_kinds: &["boot.Module", "CONTENT_SOURCE"],
//!     published_kinds: &["Asset"],
//!     published_properties: &[
//!         "asset.name", "asset.kind", "asset.hash",
//!         "asset.size", "asset.bytespace", "asset.generation",
//!         "asset.source", "asset.ready"
//!     ],
//!     idempotent: true,
//!     boot_assumptions: &[],
//! }
//! ```

#![allow(dead_code)]

/// A service contract declaration
///
/// This struct defines the complete interface contract for a graph-native service.
/// Services MUST declare their contract at startup and register it in the graph.
#[derive(Debug, Clone)]
pub struct ServiceContract {
    /// Canonical service name (e.g., "ingestd", "blossom")
    pub name: &'static str,
    
    /// Node kinds this service watches (input dependencies)
    ///
    /// Service MUST NOT watch kinds not declared here.
    /// Empty array means service doesn't watch any nodes (clock-driven, etc.)
    pub watched_kinds: &'static [&'static str],
    
    /// Node kinds this service publishes (output)
    ///
    /// Service MUST NOT create nodes of kinds not declared here.
    /// Empty array means service doesn't create nodes (pure transformer, etc.)
    pub published_kinds: &'static [&'static str],
    
    /// Property keys this service sets on published nodes
    ///
    /// Service MUST NOT set properties not declared here on its published kinds.
    /// May also set properties on watched nodes (transformations).
    pub published_properties: &'static [&'static str],
    
    /// Whether this service's operations are idempotent
    ///
    /// `true` means:
    /// - Processing the same input multiple times produces identical output
    /// - Service can be safely restarted without corrupting state
    /// - Watches can be replayed without side effects
    ///
    /// `false` means:
    /// - Service maintains internal state that cannot be reconstructed
    /// - Duplicate events may cause incorrect behavior
    /// - Special recovery procedures needed on restart
    pub idempotent: bool,
    
    /// Boot-time assumptions that MUST exist before service starts
    ///
    /// For graph-native services, this MUST be empty!
    /// Non-empty values indicate a service that violates the watch-driven model.
    ///
    /// Example of INVALID assumptions (boot-shaped thinking):
    /// - "All fonts are loaded"
    /// - "Framebuffer exists"
    /// - "Network is available"
    ///
    /// Instead, services MUST watch for these things and react when they appear.
    pub boot_assumptions: &'static [&'static str],
}

impl ServiceContract {
    /// Validate that a service contract is well-formed
    ///
    /// Returns `Ok(())` if the contract is valid, or an error string describing
    /// the violation.
    pub fn validate(&self) -> Result<(), &'static str> {
        // Service name must not be empty
        if self.name.is_empty() {
            return Err("Service name cannot be empty");
        }
        
        // Graph-native services MUST NOT have boot assumptions
        if !self.boot_assumptions.is_empty() {
            return Err("Graph-native services MUST NOT have boot assumptions");
        }
        
        // Service must either watch or publish (or both)
        if self.watched_kinds.is_empty() && self.published_kinds.is_empty() {
            return Err("Service must watch and/or publish nodes");
        }
        
        Ok(())
    }
    
    /// Check if this service watches a given node kind
    pub fn watches_kind(&self, kind: &str) -> bool {
        self.watched_kinds.iter().any(|k| *k == kind)
    }
    
    /// Check if this service publishes a given node kind
    pub fn publishes_kind(&self, kind: &str) -> bool {
        self.published_kinds.iter().any(|k| *k == kind)
    }
    
    /// Check if this service publishes a given property
    pub fn publishes_property(&self, property: &str) -> bool {
        self.published_properties.iter().any(|p| *p == property)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_contract() {
        let contract = ServiceContract {
            name: "test_service",
            watched_kinds: &["boot.Module"],
            published_kinds: &["Asset"],
            published_properties: &["asset.name", "asset.hash"],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert!(contract.validate().is_ok());
    }
    
    #[test]
    fn test_empty_name() {
        let contract = ServiceContract {
            name: "",
            watched_kinds: &["boot.Module"],
            published_kinds: &["Asset"],
            published_properties: &["asset.name"],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert!(contract.validate().is_err());
    }
    
    #[test]
    fn test_boot_assumptions_forbidden() {
        let contract = ServiceContract {
            name: "bad_service",
            watched_kinds: &["boot.Module"],
            published_kinds: &["Asset"],
            published_properties: &["asset.name"],
            idempotent: true,
            boot_assumptions: &["Framebuffer exists"],
        };
        
        assert_eq!(
            contract.validate(),
            Err("Graph-native services MUST NOT have boot assumptions")
        );
    }
    
    #[test]
    fn test_must_watch_or_publish() {
        let contract = ServiceContract {
            name: "useless_service",
            watched_kinds: &[],
            published_kinds: &[],
            published_properties: &[],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert_eq!(
            contract.validate(),
            Err("Service must watch and/or publish nodes")
        );
    }
    
    #[test]
    fn test_watches_kind() {
        let contract = ServiceContract {
            name: "test",
            watched_kinds: &["boot.Module", "Asset"],
            published_kinds: &[],
            published_properties: &[],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert!(contract.watches_kind("boot.Module"));
        assert!(contract.watches_kind("Asset"));
        assert!(!contract.watches_kind("Unknown"));
    }
    
    #[test]
    fn test_publishes_kind() {
        let contract = ServiceContract {
            name: "test",
            watched_kinds: &[],
            published_kinds: &["Asset", "Font"],
            published_properties: &[],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert!(contract.publishes_kind("Asset"));
        assert!(contract.publishes_kind("Font"));
        assert!(!contract.publishes_kind("Unknown"));
    }
    
    #[test]
    fn test_publishes_property() {
        let contract = ServiceContract {
            name: "test",
            watched_kinds: &[],
            published_kinds: &["Asset"],
            published_properties: &["asset.name", "asset.hash"],
            idempotent: true,
            boot_assumptions: &[],
        };
        
        assert!(contract.publishes_property("asset.name"));
        assert!(contract.publishes_property("asset.hash"));
        assert!(!contract.publishes_property("unknown.prop"));
    }
}

//! Boot-time Capability Grants Table
//!
//! This module provides a centralized registry for default capabilities granted to modules
//! at boot time. It replaces ad-hoc capability injection scattered across syscalls.
//!
//! # Design
//!
//! The boot grants table maps module identifiers (name or ThingId) to a set of capabilities
//! that should be granted when the module is spawned. This provides:
//!
//! 1. **Predictability**: All capability grants are declared in one place
//! 2. **Transparency**: Easy to audit what capabilities each module receives
//! 3. **Consistency**: No hidden capability injections in syscall implementations
//!
//! # Usage
//!
//! During boot:
//! 1. `register_module_grants()` is called to populate the table from Module bodies
//! 2. When spawning a task, `apply_boot_grants_by_name()` grants capabilities to the task
//!
//! # Policy
//!
//! Boot grants are intended for:
//! - Privileged system services (bloom, inputd, etc.)
//! - Initial capability set for modules loaded from Seed
//!
//! Creator ownership (e.g., surface creator gets read/write on the surface) is handled
//! separately as a default policy in object creation syscalls.

use abi::cap::Cap;
use abi::ids::ThingId;
use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;

/// A grant entry mapping a module to its default capabilities
#[derive(Clone, Debug)]
pub struct BootGrant {
    pub module_thing: ThingId,
    pub module_name: Option<String>,
    pub caps: Vec<Cap>,
}

/// Global registry of boot-time capability grants
static BOOT_GRANTS: Mutex<Vec<BootGrant>> = Mutex::new(Vec::new());

/// Register capabilities for a module by ThingId and optional name
///
/// This should be called during boot seeding for each module that needs default capabilities.
pub fn register_module_grants(module_thing: ThingId, module_name: Option<String>, caps: Vec<Cap>) {
    let mut grants = BOOT_GRANTS.lock();
    
    // Remove any existing entry for this module (in case of re-registration)
    grants.retain(|g| g.module_thing != module_thing);
    
    if !caps.is_empty() {
        grants.push(BootGrant {
            module_thing,
            module_name,
            caps,
        });
    }
}

/// Apply boot grants to a task based on its module ThingId
///
/// This looks up the task's module in the boot grants table and applies
/// the registered capabilities to the task.
///
/// Returns the number of capabilities granted.
pub fn apply_boot_grants(task_thing: ThingId, task_caps: &mut Vec<Cap>) -> usize {
    let grants = BOOT_GRANTS.lock();
    
    if let Some(grant) = grants.iter().find(|g| g.module_thing == task_thing) {
        let initial_count = task_caps.len();
        task_caps.extend_from_slice(&grant.caps);
        task_caps.len() - initial_count
    } else {
        0
    }
}

/// Apply boot grants to a task based on module name/path
///
/// This searches for a module whose name matches or is contained in the given path.
/// Returns the number of capabilities granted.
pub fn apply_boot_grants_by_name(module_path: &str, task_caps: &mut Vec<Cap>) -> usize {
    let grants = BOOT_GRANTS.lock();
    
    // Try to find a matching module by name
    if let Some(grant) = grants.iter().find(|g| {
        if let Some(ref name) = g.module_name {
            module_path.contains(name.as_str())
        } else {
            false
        }
    }) {
        let initial_count = task_caps.len();
        task_caps.extend_from_slice(&grant.caps);
        task_caps.len() - initial_count
    } else {
        0
    }
}

/// Get a copy of all registered boot grants (for debugging/testing)
#[allow(dead_code)]
pub fn get_all_grants() -> Vec<BootGrant> {
    BOOT_GRANTS.lock().clone()
}

/// Clear all boot grants (for testing)
#[cfg(test)]
pub fn clear_grants() {
    BOOT_GRANTS.lock().clear()
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::cap::{CapOp, CapScope};

    #[test]
    fn test_register_and_apply() {
        clear_grants();
        
        let module_id = ThingId(0x1234);
        let caps = alloc::vec![
            Cap { op: CapOp::Log, scope: CapScope::Global },
            Cap { op: CapOp::GraphRead, scope: CapScope::Global },
        ];
        
        register_module_grants(module_id, None, caps.clone());
        
        let mut task_caps = Vec::new();
        let count = apply_boot_grants(module_id, &mut task_caps);
        
        assert_eq!(count, 2);
        assert_eq!(task_caps, caps);
    }

    #[test]
    fn test_apply_by_name() {
        clear_grants();
        
        let module_id = ThingId(0x1234);
        let caps = alloc::vec![
            Cap { op: CapOp::Log, scope: CapScope::Global },
        ];
        
        register_module_grants(module_id, Some("sprout".into()), caps.clone());
        
        let mut task_caps = Vec::new();
        let count = apply_boot_grants_by_name("/modules/sprout", &mut task_caps);
        
        assert_eq!(count, 1);
        assert_eq!(task_caps, caps);
    }

    #[test]
    fn test_no_grants_for_unknown_module() {
        clear_grants();
        
        let mut task_caps = Vec::new();
        let count = apply_boot_grants(ThingId(0x9999), &mut task_caps);
        
        assert_eq!(count, 0);
        assert!(task_caps.is_empty());
    }

    #[test]
    fn test_re_registration_replaces() {
        clear_grants();
        
        let module_id = ThingId(0x1234);
        
        // First registration
        let caps1 = alloc::vec![
            Cap { op: CapOp::Log, scope: CapScope::Global },
        ];
        register_module_grants(module_id, None, caps1);
        
        // Second registration should replace
        let caps2 = alloc::vec![
            Cap { op: CapOp::GraphRead, scope: CapScope::Global },
            Cap { op: CapOp::GraphWrite, scope: CapScope::Global },
        ];
        register_module_grants(module_id, None, caps2.clone());
        
        let mut task_caps = Vec::new();
        apply_boot_grants(module_id, &mut task_caps);
        
        assert_eq!(task_caps, caps2);
    }
}

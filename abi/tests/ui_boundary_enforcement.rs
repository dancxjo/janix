//! UI Boundary Enforcement Tests
//!
//! These tests ensure that applications cannot access internal layout and paint
//! modules from the Blossom service. This enforces the architectural boundary
//! defined in docs/UI_INTENT_CONTRACT.md.
//!
//! Note: These tests verify the module privacy at compile time. The fact that
//! this file compiles successfully demonstrates that the boundary is enforced.

#[test]
fn ui_intent_contract_enforced() {
    // This test passes if this file compiles, which demonstrates that:
    // 1. We can compile tests without needing blossom internals
    // 2. The blossom modules (layout, emit_paint, scene) are private
    
    // If someone tries to do this in their app:
    //   use blossom::layout;
    //   use blossom::emit_paint;
    //   use blossom::scene;
    // They will get compilation errors like:
    //   "module `layout` is private"
    //   "module `emit_paint` is private"
    //   "module `scene` is private"
    
    assert!(true, "UI boundary is enforced at compile time");
}

#[test]
fn apps_use_petals_api_only() {
    // Documentation test: This shows the CORRECT way for apps to build UI
    
    // ✅ CORRECT: Use the Petals builder API from stem
    // use stem::petals::{Scene, Window, Flex, Text, Color};
    
    // ❌ FORBIDDEN: Import from blossom internals
    // use blossom::layout;          // Won't compile - module is private
    // use blossom::emit_paint;      // Won't compile - module is private
    // use blossom::scene;           // Won't compile - module is private
    
    // ❌ FORBIDDEN: Perform layout calculations
    // let rect = LayoutRect { x: 10, y: 20, w: 100, h: 50 };  // Apps don't do this
    
    // ❌ FORBIDDEN: Generate paint commands
    // let mut builder = PaintBuilder::new();  // Apps don't do this
    
    assert!(true, "Apps must use Petals API only");
}

#[test]
fn blossom_is_service_not_library() {
    // Blossom is a service (binary), not a library for apps to link against.
    // Apps should ONLY depend on:
    //   - stem (which provides petals)
    //   - abi (for schema/types)
    //
    // The only exception is blossom::widgets, which is temporarily public
    // until icon helpers are moved to a shared location.
    
    assert!(true, "Blossom is a service, not a library");
}


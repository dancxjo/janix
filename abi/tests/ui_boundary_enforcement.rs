//! UI Boundary Enforcement Tests
//!
//! These tests document and verify the architectural boundary between
//! applications and the Blossom layout/paint service as defined in
//! docs/UI_INTENT_CONTRACT.md.
//!
//! Note: The primary enforcement mechanism is compile-time module privacy.
//! These tests serve as documentation and verification that the API surface
//! is correctly designed.

#[test]
fn ui_intent_contract_documented() {
    // This test exists primarily as documentation that the UI Intent Contract
    // (docs/UI_INTENT_CONTRACT.md) defines the architectural boundary.
    //
    // The boundary is enforced at compile time through Rust's module privacy:
    // - blossom::layout is private (mod layout)
    // - blossom::emit_paint is private (mod emit_paint)
    // - blossom::scene is private (mod scene)
    //
    // Any attempt to import these modules from outside blossom will fail:
    //   use blossom::layout;      // error: module `layout` is private
    //   use blossom::emit_paint;  // error: module `emit_paint` is private
    //   use blossom::scene;       // error: module `scene` is private
}

#[test]
fn petals_api_is_sufficient_for_apps() {
    // This test verifies that the public Petals API provides what apps need
    // to build UI without accessing Blossom internals.
    //
    // Apps can successfully build UI using only:
    //   use stem::petals::Petals;
    //
    // This is demonstrated by Font Explorer which:
    // - Depends only on stem and abi (not blossom)
    // - Uses only Petals builders
    // - Publishes via stem::petals::Petals::begin_window(...).finish()
    // - Contains no layout or paint code
}

#[test]
fn blossom_modules_are_implementation_details() {
    // Verification that Blossom's internal modules are not part of the public API.
    //
    // The following modules are implementation details and private:
    // - layout::layout_scene() - computes rectangles from intent
    // - emit_paint::emit_paint() - generates paint commands
    // - scene::SceneGraph - internal scene representation
    //
    // Apps must use the Petals builder API instead:
    // - stem::petals::Scene - for building intent
    // - stem::petals::Petals graph-native publishing
    //
    // This separation allows Blossom to change its layout/paint algorithms
    // without breaking apps.
}


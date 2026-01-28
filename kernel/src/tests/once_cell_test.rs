//! Unit tests for OnceCell.
//!
//! These tests verify that OnceCell enforces single-assignment semantics:
//! - Single init succeeds
//! - Double init panics
//! - Access before init panics

use crate::once_cell::OnceCell;

#[test]
fn test_once_cell_single_init() {
    let cell: OnceCell<i32> = OnceCell::new();
    assert!(!cell.is_initialized());

    cell.set(42);

    assert!(cell.is_initialized());
    assert_eq!(*cell.get(), 42);
}

#[test]
#[should_panic(expected = "called on already-initialized")]
fn test_once_cell_double_init_panics() {
    let cell: OnceCell<i32> = OnceCell::new();
    cell.set(1);
    cell.set(2); // Should panic
}

#[test]
#[should_panic(expected = "called before initialization")]
fn test_once_cell_get_before_init_panics() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.get(); // Should panic
}

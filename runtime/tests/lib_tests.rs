use runtime::init_user_heap;

#[test]
fn init_user_heap_is_noop_on_host() {
    // Should not panic or require allocation setup when running in hosted tests.
    init_user_heap();
}

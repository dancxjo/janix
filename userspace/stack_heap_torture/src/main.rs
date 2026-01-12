#![no_std]
#![no_main]

extern crate alloc;

use stem::info;

mod tests;

#[stem::main]
fn main() -> ! {
    info!("[torture] Starting stack_heap_torture");
    
    // Run all tests in sequence
    // In the future, this could be parameterized via kernel args
    
    // Stack tests
    tests::stack_recursion(256);
    tests::stack_context_stress();
    
    // Heap tests
    tests::heap_churn(500);
    tests::heap_realloc();
    tests::heap_fuzz(123);
    
    info!("[torture] All tests completed successfully");
    
    loop { 
        stem::yield_now(); 
    }
}

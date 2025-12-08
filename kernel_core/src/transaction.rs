use abi::TransactionId;

// SAFETY: TX_COUNTER is only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut TX_COUNTER: u64 = 0;

/// Initialize the transaction subsystem
pub fn init() {
    // For now, just a stub
}

/// Create a new transaction
pub fn create_transaction() -> TransactionId {
    unsafe {
        TX_COUNTER += 1;
        TransactionId(TX_COUNTER)
    }
}

/// Commit a transaction
pub fn commit_transaction(tx_id: TransactionId) -> Result<(), &'static str> {
    // Stub implementation - just validate tx_id
    if tx_id.0 > 0 && tx_id.0 <= unsafe { TX_COUNTER } {
        Ok(())
    } else {
        Err("Invalid transaction ID")
    }
}

/// Rollback a transaction
pub fn rollback_transaction(tx_id: TransactionId) -> Result<(), &'static str> {
    // Stub implementation
    if tx_id.0 > 0 {
        Ok(())
    } else {
        Err("Invalid transaction ID")
    }
}

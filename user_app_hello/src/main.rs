use abi::NodeId;

fn main() {
    // Log a hello message
    userland_std::println("Hello from user_app_hello!");
    
    // Query some nodes in the graph
    userland_std::println("Querying graph nodes:");
    for i in 0..5 {
        let node_id = NodeId(i);
        match userland_std::graph_query(node_id) {
            Some(_value) => userland_std::println("  node found"),
            None => userland_std::println("  node not found"),
        }
    }
    
    // Create and commit a transaction
    userland_std::println("Creating transaction...");
    if let Some(_tx_id) = userland_std::create_transaction() {
        userland_std::println("  Transaction created");
        if userland_std::commit_transaction(_tx_id) {
            userland_std::println("  Transaction committed successfully");
        } else {
            userland_std::println("  Failed to commit transaction");
        }
    } else {
        userland_std::println("  Failed to create transaction");
    }
    
    userland_std::println("Goodbye from user_app_hello!");
}

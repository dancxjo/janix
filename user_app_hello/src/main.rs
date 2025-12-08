use abi::NodeId;

fn main() {
    // Log a hello message
    userland_std::println("Hello from user_app_hello!");
    
    // Query some nodes in the graph
    println!("Querying graph nodes:");
    for i in 0..5 {
        let node_id = NodeId(i);
        match userland_std::graph_query(node_id) {
            Some(value) => println!("  Node {}: value = {}", i, value),
            None => println!("  Node {}: not found", i),
        }
    }
    
    // Create and commit a transaction
    println!("\nCreating transaction...");
    if let Some(tx_id) = userland_std::create_transaction() {
        println!("  Transaction created: {:?}", tx_id);
        if userland_std::commit_transaction(tx_id) {
            println!("  Transaction committed successfully");
        } else {
            println!("  Failed to commit transaction");
        }
    } else {
        println!("  Failed to create transaction");
    }
    
    userland_std::println("Goodbye from user_app_hello!");
}

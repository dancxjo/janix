use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use phloem::{GraphExecutor, parse};

/// Execute a GQL query and return the result as a string
pub fn execute_gql_query(query: &str) -> Result<String, String> {
    // Parse the query
    let command = parse(query)?;
    
    // Execute the command
    let mut executor = GraphExecutor::new();
    let result = executor.execute(command);
    
    Ok(result)
}

/// Handle a GQL query from HTTP request body
pub fn handle_gql_post(body: &str) -> Vec<u8> {
    match execute_gql_query(body) {
        Ok(result) => {
            // Return as plain text for now
            result.into_bytes()
        }
        Err(e) => {
            format!("error: {}\n", e).into_bytes()
        }
    }
}

/// Handle a GQL query from query parameter
pub fn handle_gql_get(query_param: &str) -> Vec<u8> {
    // URL decode if needed (for now, assume it's already decoded)
    handle_gql_post(query_param)
}

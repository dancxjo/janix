use alloc::string::{String, ToString};
use alloc::vec::Vec;
use phloem::{GraphExecutor, parse, ExecutionResult, ResultValue};
use crate::graph_api::JsonBuilder;

/// Execute a GQL query and return the result as a JSON string
pub fn execute_gql_query(query: &str) -> String {
    // Parse the query
    let command = match parse(query) {
        Ok(c) => c,
        Err(e) => {
            let mut json = JsonBuilder::new();
            json.start_object();
            json.key("success");
            json.bool_value(false);
            json.key("error");
            json.string_value(&e);
            json.end_object();
            return json.as_string().unwrap_or_else(|_| "{\"success\":false}".to_string());
        }
    };
    
    // Execute the command
    let mut executor = GraphExecutor::new();
    let result = executor.execute(command);
    
    execution_result_to_json(result)
}

fn execution_result_to_json(res: ExecutionResult) -> String {
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("success");
    json.bool_value(res.success);
    
    json.key("message");
    json.string_value(&res.message);
    
    if !res.columns.is_empty() {
        json.key("columns");
        json.start_array();
        for col in &res.columns {
            json.string_value(col);
        }
        json.end_array();
        json.buf.push(b',');
        
        json.key("rows");
        json.start_array();
        for row in res.rows {
            json.start_array();
            for val in row {
                match val {
                    ResultValue::Node(id) => {
                        json.start_object();
                        json.key("type");
                        json.string_value("node");
                        json.key("id");
                        json.number_value(id);
                        json.end_object();
                        json.buf.push(b',');
                    }
                    ResultValue::String(s) => json.string_value(&s),
                    ResultValue::Number(n) => json.number_value(n),
                }
            }
            json.end_array();
            json.buf.push(b',');
        }
        json.end_array();
    }
    
    json.end_object();
    json.as_string().unwrap_or_else(|_| "{\"success\":false}".to_string())
}

/// Handle a GQL query from HTTP request body
pub fn handle_gql_post(body: &str) -> Vec<u8> {
    let raw = body.trim();
    if raw.starts_with("VIEW ") {
        let view_id = raw[5..].trim();
        // Forward to view handler body (not full response)
        let json_body = crate::api_v1::handle_get_view_body(view_id, "");
        return json_body.into_bytes();
    }

    let result = execute_gql_query(body);
    result.into_bytes()
}

/// Handle a GQL query from query parameter
pub fn handle_gql_get(query_param: &str) -> Vec<u8> {
    handle_gql_post(query_param)
}

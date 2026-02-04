use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use crate::gql::{Command, Pattern, Value, NodePattern, ReturnExpression};
use stem::thing::sys as graph;
use stem::abi::ids::HandleId;
use stem::thing::ThingId;
use crate::{ExecutionResult, ResultValue};

pub struct GraphExecutor {
    bindings: BTreeMap<String, u64>,
    parameters: BTreeMap<String, Value>,
}

impl GraphExecutor {
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
            parameters: BTreeMap::new(),
        }
    }

    pub fn set_parameter(&mut self, name: String, value: Value) {
        self.parameters.insert(name, value);
    }

    pub fn execute(&mut self, cmd: Command) -> ExecutionResult {
        match cmd {
            Command::Help => self.help(),
            Command::Quit => ExecutionResult::message("Bye."),
            Command::Schema => ExecutionResult::error("Schema not implemented."),
            Command::Merge { pattern: _, returns: _, skip: _ } => {
                // TODO: Implement MERGE properly (for now it just MATCHes/CREATEs)
                ExecutionResult::error("MERGE not fully implemented")
            }
            Command::Match { pattern, where_clause, returns, limit, skip } => {
                self.execute_match(pattern, where_clause, returns, limit, skip)
            }
            Command::Set { var, key, value } => self.execute_set(var, key, value),
        }
    }

    fn help(&self) -> ExecutionResult {
        ExecutionResult::message(
            "Commands:\n\
             MERGE (n:Kind {key: \"val\"})\n\
             MERGE (a)-[:REL]->(b)\n\
             MATCH (n:Kind {key: \"val\"}) RETURN n\n\
             MATCH (a)-[:REL]->(b) RETURN a, b\n\
             SET n.key = \"val\"\n\
             HELP\n\
             QUIT"
        )
    }

    fn execute_merge(&mut self, pattern: Pattern, returns: Vec<ReturnExpression>) -> ExecutionResult {
        match pattern {
            Pattern::Node(node_pat) => {
                let id = match self.ensure_node(&node_pat) {
                    Ok(id) => id,
                    Err(e) => return ExecutionResult::error(&e),
                };
                self.bindings.insert(node_pat.var, id);

                if returns.is_empty() {
                    return ExecutionResult::success(&format!("ok: merged node (id:{})", id));
                } else {
                    return self.format_results_structured(&returns);
                }
            }
            Pattern::Edge { src_var, rel, dst_var } => {
                let src_id = match self.bindings.get(&src_var) {
                    Some(&id) => id,
                    None => return ExecutionResult::error(&format!("variable '{}' not bound", src_var)),
                };
                let dst_id = match self.bindings.get(&dst_var) {
                    Some(&id) => id,
                    None => return ExecutionResult::error(&format!("variable '{}' not bound", dst_var)),
                };

                match self.ensure_edge(src_id, &rel, dst_id) {
                    Ok(_) => {
                        if returns.is_empty() {
                            ExecutionResult::success(&format!("ok: merged edge ({})-[:{}]->({})", src_id, rel, dst_id))
                        } else {
                            self.format_results_structured(&returns)
                        }
                    }
                    Err(e) => ExecutionResult::error(&e),
                }
            }
        }
    }

    fn execute_match(&self, pattern: Pattern, where_clause: Option<crate::gql::Expression>, returns: Vec<ReturnExpression>, limit: usize, skip: usize) -> ExecutionResult {
        match pattern {
            Pattern::Node(node_pat) => {
                let limit_total = limit + skip;
                let mut matched_ids = Vec::new();

                // OPTIMIZATION: If WHERE id(n) = $id or id(n) = 123, just look up that node
                if let Some(ref expr) = where_clause {
                    if let crate::gql::Expression::Eq(left, right) = expr {
                        let mut target_id = None;
                        if let (crate::gql::Expression::IdFunc(var), crate::gql::Expression::Value(val)) = (&**left, &**right) {
                            if var == &node_pat.var {
                                target_id = self.resolve_value_as_u64(val);
                            }
                        } else if let (crate::gql::Expression::Value(val), crate::gql::Expression::IdFunc(var)) = (&**left, &**right) {
                            if var == &node_pat.var {
                                target_id = self.resolve_value_as_u64(val);
                            }
                        }

                        if let Some(id) = target_id {
                            // Verify kind and other props
                            let matches_kind = match &node_pat.kind {
                                Some(k) => match graph::get_kind(ThingId::from_u64(id)) {
                                    Ok(kind_id) => {
                                        let kind_name = self.resolve_symbol(kind_id.0 as u32).unwrap_or_default();
                                        &kind_name == k
                                    }
                                    Err(_) => false,
                                },
                                None => true,
                            };

                            if matches_kind && self.matches_props(id, &node_pat.props) {
                                matched_ids.push(id);
                            }
                            
                            // Skip discovery since we had a direct ID lookup
                            return self.format_match_results(&node_pat, matched_ids, returns, limit, skip);
                        }
                    }
                }

                match &node_pat.kind {
                    Some(_k) => { // Changed `k` to `_k` as it's not used directly here
                        let mut candidates = [ThingId::from_u64(0); 2048];
                        stem::info!("phloem: calling find for kind: {}", _k);
                        let count = match graph::find(_k.as_str(), &mut candidates) {
                            Ok(c) => c,
                            Err(e) => {
                                stem::info!("phloem: find failed for kind {}: {:?}", _k, e);
                                return ExecutionResult::error("find syscall failed")
                            },
                        };
                        stem::info!("phloem: find returned {} candidates", count);

                        for i in 0..count {
                            let id = candidates[i].to_u64_lossy();
                            if self.matches_props(id, &node_pat.props) {
                                if !matched_ids.contains(&id) {
                                    matched_ids.push(id);
                                }
                                if matched_ids.len() >= limit_total {
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        // BFS Discovery from well-known roots and exhaustive kinds
                        let mut queue = VecDeque::new();
                        let mut seen = BTreeSet::new();

                        // Well-known roots: Host(1), Root(2), Scheduler(3)
                        for &root_id in &[1u64, 2u64, 3u64] {
                            queue.push_back(root_id);
                            seen.insert(root_id);
                        }

                        // Exhaustive kinds to seed from
                        let fallback_kinds = [
                            "fs.File", "content.Source", "Asset", "ui.Window", "proc.Process", "dev.bus.Pci",
                            "dev.pci.Function", "dev.net.Nic", "dev.storage.Disk", "dev.display.Gpu",
                            "dev.Cpu", "ui.Crown", "font.Family", "font.Face", "boot.Module",
                            "svc.net.Stack", "svc.net.Driver", "Bytespace", "mem.Range", "proc.Thread",
                            "proc.Task", "proc.Kernel", "svc.Root", "svc.Scheduler", "dev.Host",
                            "mem.Page", "mem.Stack", "mem.Heap", "ui.Panel", "ui.Text", "font.Family",
                            "font.Face", "font.File", "xml.Document", "html.Document", "css.Stylesheet"
                        ];
                        for &kind in &fallback_kinds {
                            let mut seeds = [ThingId::from_u64(0); 512];
                            stem::info!("phloem: discovery find for kind: {}", kind);
                            match graph::find(kind, &mut seeds) {
                                Ok(count) => {
                                    if count > 0 {
                                        stem::info!("phloem: discovery found {} seeds for {}", count, kind);
                                    }
                                    for i in 0..count {
                                        let id = seeds[i].to_u64_lossy();
                                        if !seen.contains(&id) {
                                            seen.insert(id);
                                            queue.push_back(id);
                                        }
                                    }
                                }
                                Err(e) => {
                                    stem::info!("phloem: discovery find failed for kind {}: {:?}", kind, e);
                                }
                            }
                        }

                        while let Some(current_id) = queue.pop_front() {
                            if matched_ids.len() >= limit_total {
                                break;
                            }

                            if self.matches_props(current_id, &node_pat.props) {
                                // Apply where clause filter if present (and not already handled by optimization)
                                let matches_where = if let Some(ref expr) = where_clause {
                                    self.evaluate_expression(expr, current_id, &node_pat.var)
                                } else {
                                    true
                                };

                                if matches_where {
                                    if !matched_ids.contains(&current_id) {
                                        matched_ids.push(current_id);
                                    }
                                }
                            }

                            // Discover neighbors
                            if let Ok(edges) = self.get_outbound_edges(current_id) {
                                for (_rel_id, dst_id) in edges {
                                    if !seen.contains(&dst_id) {
                                        seen.insert(dst_id);
                                        queue.push_back(dst_id);
                                    }
                                }
                            }
                        }
                    }
                }
    
                self.format_match_results(&node_pat, matched_ids, returns, limit, skip)
            }
            Pattern::Edge { src_var, rel: _, dst_var } => {
                let src_id_opt = self.bindings.get(&src_var).cloned();
                let _limit_total = limit + skip;

                if let Some(src_id) = src_id_opt {
                    let edges = match self.get_outbound_edges(src_id) {
                        Ok(e) => e,
                        Err(_) => return ExecutionResult::error("failed to get edges"),
                    };

                    let mut all_rows = Vec::new();

                    for (_rel_id, dst_id) in edges {
                        let mut row = Vec::new();
                        for expr in &returns {
                            match expr {
                                ReturnExpression::Variable(var) => {
                                    if var == &src_var {
                                        row.push(ResultValue::Node(src_id));
                                    } else if var == &dst_var {
                                        row.push(ResultValue::Node(dst_id));
                                    } else {
                                        // TODO: Support property returns like RETURN n.prop
                                        row.push(ResultValue::String(format!("unsupported: {}", var)));
                                    }
                                }
                                ReturnExpression::Count(_) => {
                                    row.push(ResultValue::String("count() not supported in edge match yet".to_string()));
                                }
                            }
                        }
                        all_rows.push(row);
                    }

                    let rows: Vec<Vec<ResultValue>> = all_rows
                        .into_iter()
                        .skip(skip)
                        .take(limit)
                        .collect();

                    let cols: Vec<String> = returns.iter().map(|r| r.to_string()).collect();
                    ExecutionResult::rows(cols, rows)

                } else {
                     return ExecutionResult::error("MATCH edge requires bound source (e.g. use MERGE/MATCH node first)");
                }
            }
        }
    }

    fn execute_set(&mut self, var: String, key: String, value: Value) -> ExecutionResult {
        let id = match self.bindings.get(&var) {
            Some(&id) => id,
            None => return ExecutionResult::error(&format!("variable '{}' not bound", var)),
        };

        let val_u64 = match value {
            Value::String(s) => match graph::intern(&s) {
                Ok(id) => id as u64,
                Err(_) => return ExecutionResult::error("intern value failed"),
            },
            Value::Number(n) => n,
            Value::Parameter(name) => {
                match self.parameters.get(&name) {
                    Some(Value::Number(n)) => *n,
                    Some(Value::String(s)) => match graph::intern(s) {
                        Ok(id) => id as u64,
                        Err(_) => return ExecutionResult::error("intern parameter value failed"),
                    },
                    _ => return ExecutionResult::error(&format!("parameter '{}' not found or invalid type", name)),
                }
            }
        };

        match graph::prop_set(ThingId::from_u64(id), key.as_str(), val_u64) {
            Ok(_) => ExecutionResult::success("ok: property set"),
            Err(e) => ExecutionResult::error(&format!("prop_set failed {:?}", e)),
        }
    }

    fn ensure_node(&mut self, pat: &NodePattern) -> Result<u64, String> {
        let kind = pat.kind.as_ref().ok_or("MERGE requires a Kind")?;

        // 1. Try to find
        let mut candidates = [ThingId::from_u64(0); 128];
        let count = graph::find(kind.as_str(), &mut candidates).map_err(|_| "find failed")?;

        for i in 0..count {
            let id = candidates[i].to_u64_lossy();
            if self.matches_props(id, &pat.props) {
                return Ok(id);
            }
        }

        // 2. Create
        let id_new = graph::create_node(kind.as_str()).map_err(|_| "create_node failed")?;
        let id = id_new.to_u64_lossy();

        // Set props
        for (k, v) in &pat.props {
            let val_u64 = match v {
                Value::String(s) => graph::intern(s).map_err(|_| "intern val failed")? as u64,
                Value::Number(n) => *n,
                Value::Parameter(p) => {
                    match self.parameters.get(p) {
                        Some(Value::Number(n)) => *n,
                        Some(Value::String(s)) => graph::intern(s).map_err(|_| "intern val failed")? as u64,
                        _ => return Err(format!("parameter '{}' not found", p)),
                    }
                }
            };
            graph::prop_set(id_new, k.as_str(), val_u64).map_err(|_| "prop_set failed")?;
        }

        Ok(id)
    }

    fn ensure_edge(&mut self, src: u64, rel: &str, dst: u64) -> Result<(), String> {
        // Check existing edges to ensure idempotence
        let edges = self.get_outbound_edges(src).map_err(|_| "failed to scan edges")?;
        for (e_rel_id, e_dst) in edges {
            let rel_name = self.resolve_symbol(e_rel_id as u32).unwrap_or_default();
            if rel_name == rel && e_dst == dst {
                // Already exists
                return Ok(());
            }
        }

        graph::link(ThingId::from_u64(src), rel, ThingId::from_u64(dst)).map_err(|_| "link failed")?;
        Ok(())
    }

    fn get_outbound_edges(&self, src: u64) -> Result<Vec<(u64, u64)>, ()> {
        let mut buf = [abi::types::Edge::default(); 1024];
        match graph::get_edges(ThingId::from_u64(src), &mut buf) {
            Ok(count) => {
                let mut res = Vec::new();
                for i in 0..count {
                    res.push((buf[i].predicate.to_u64_lossy(), buf[i].to.to_u64_lossy()));
                }
                Ok(res)
            }
            Err(_) => Err(()),
        }
    }

    fn matches_props(&self, id: u64, props: &[(String, Value)]) -> bool {
        for (k, v) in props {
            let key_id = match graph::intern(k) {
                Ok(id) => id,
                Err(_) => return false,
            };

            let val_id = match graph::prop_get(ThingId::from_u64(id), key_id) {
                Ok(v) => v,
                Err(_) => return false, // Property missing
            };

            match v {
                Value::Number(n) => {
                    if val_id != *n { return false; }
                }
                Value::String(s) => {
                    match graph::intern(s) {
                        Ok(s_id) => {
                            if (s_id as u64) != val_id { return false; }
                        }
                        Err(_) => return false,
                    }
                }
                Value::Parameter(p) => {
                    match self.parameters.get(p) {
                        Some(Value::Number(n)) => { if val_id != *n { return false; } }
                        Some(Value::String(s)) => {
                            match graph::intern(s) {
                                Ok(s_id) => { if (s_id as u64) != val_id { return false; } }
                                Err(_) => return false,
                            }
                        }
                        _ => return false,
                    }
                }
            }
        }
        true
    }

    fn format_results_structured(&self, vars: &[ReturnExpression]) -> ExecutionResult {
        let mut row = Vec::new();
        let mut col_names = Vec::new();
        for expr in vars {
            col_names.push(expr.to_string());
            match expr {
                ReturnExpression::Variable(var) => {
                    if let Some(&id) = self.bindings.get(var) {
                        row.push(ResultValue::Node(id));
                    } else {
                        row.push(ResultValue::Number(0));
                    }
                }
                ReturnExpression::Count(var) => {
                    if let Some(_) = self.bindings.get(var) {
                        row.push(ResultValue::Number(1));
                    } else {
                        row.push(ResultValue::Number(0));
                    }
                }
            }
        }
        ExecutionResult::rows(col_names, alloc::vec![row])
    }

    fn format_node(&self, id: u64) -> String {
        let kind_id = match graph::get_kind(ThingId::from_u64(id)) {
            Ok(k) => k.0,
            Err(_) => return format!("(id:{})", id),
        };

        let kind_name = self.resolve_symbol(kind_id as u32).unwrap_or_else(|| format!("{}", kind_id));
        format!("(id:{} :{})", id, kind_name)
    }

    fn resolve_symbol(&self, id: u32) -> Option<String> {
        let mut buf = [0u8; 64];
        match graph::describe_symbol(id, &mut buf) {
            Ok(len) => {
                if len > buf.len() {
                    Some("...".to_string())
                } else {
                    core::str::from_utf8(&buf[..len]).ok().map(|s| s.to_string())
                }
            }
            Err(_) => None,
        }
    }

    fn format_match_results(&self, node_pat: &NodePattern, matched_ids: Vec<u64>, returns: Vec<ReturnExpression>, limit: usize, skip: usize) -> ExecutionResult {
        // Check if we have any aggregate functions
        let has_aggregate = returns.iter().any(|r| matches!(r, ReturnExpression::Count(_)));

        if has_aggregate {
            let mut row = Vec::new();
            for expr in &returns {
                match expr {
                    ReturnExpression::Count(var) => {
                        if var == &node_pat.var || var == "*" {
                            row.push(ResultValue::Number(matched_ids.len() as u64));
                        } else {
                            row.push(ResultValue::Number(0));
                        }
                    }
                    ReturnExpression::Variable(v) => {
                        if let Some(&id) = matched_ids.first() {
                            if v == &node_pat.var {
                                row.push(ResultValue::Node(id));
                            } else {
                                row.push(ResultValue::String(format!("unsupported: {}", v)));
                            }
                        } else {
                            row.push(ResultValue::Number(0));
                        }
                    }
                }
            }
            let cols = returns.iter().map(|r| r.to_string()).collect();
            ExecutionResult::rows(cols, alloc::vec![row])
        } else {
            let rows: Vec<Vec<ResultValue>> = matched_ids
                .into_iter()
                .skip(skip)
                .take(limit)
                .map(|id| {
                let mut row = Vec::new();
                for expr in &returns {
                    match expr {
                        ReturnExpression::Variable(col) => {
                            if col == &node_pat.var {
                                row.push(ResultValue::Node(id));
                            } else {
                                row.push(ResultValue::String(format!("unsupported: {}", col)));
                            }
                        }
                        ReturnExpression::Count(_) => unreachable!(),
                    }
                }
                row
            }).collect();

            let cols = returns.iter().map(|r| r.to_string()).collect();
            ExecutionResult::rows(cols, rows)
        }
    }

    fn resolve_value_as_u64(&self, val: &Value) -> Option<u64> {
        match val {
            Value::Number(n) => Some(*n),
            Value::Parameter(name) => {
                match self.parameters.get(name) {
                    Some(Value::Number(n)) => Some(*n),
                    _ => None,
                }
            }
            Value::String(_) => None,
        }
    }

    fn evaluate_expression(&self, expr: &crate::gql::Expression, node_id: u64, node_var: &str) -> bool {
        match expr {
            crate::gql::Expression::Eq(left, right) => {
                let l_val = self.evaluate_primary(left, node_id, node_var);
                let r_val = self.evaluate_primary(right, node_id, node_var);
                l_val == r_val && l_val.is_some()
            }
            _ => false, // Only Eq supported for now
        }
    }

    fn evaluate_primary(&self, expr: &crate::gql::Expression, node_id: u64, node_var: &str) -> Option<Value> {
        match expr {
            crate::gql::Expression::IdFunc(var) => {
                if var == node_var {
                    Some(Value::Number(node_id))
                } else {
                    None
                }
            }
            crate::gql::Expression::Value(v) => {
                match v {
                    Value::Parameter(name) => self.parameters.get(name).cloned(),
                    _ => Some(v.clone()),
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gql::{Expression, Value};
    use alloc::boxed::Box;

    #[test]
    fn test_executor_params() {
        let mut ex = GraphExecutor::new();
        ex.set_parameter("id".to_string(), Value::Number(42));
        assert_eq!(ex.parameters.get("id"), Some(&Value::Number(42)));
    }

    #[test]
    fn test_resolve_value_as_u64() {
        let mut ex = GraphExecutor::new();
        ex.set_parameter("pid".to_string(), Value::Number(100));
        
        assert_eq!(ex.resolve_value_as_u64(&Value::Number(50)), Some(50));
        assert_eq!(ex.resolve_value_as_u64(&Value::Parameter("pid".to_string())), Some(100));
        assert_eq!(ex.resolve_value_as_u64(&Value::Parameter("unknown".to_string())), None);
    }

    #[test]
    fn test_evaluate_expression() {
        let mut ex = GraphExecutor::new();
        ex.set_parameter("target".to_string(), Value::Number(123));
        
        let expr = Expression::Eq(
            Box::new(Expression::IdFunc("n".to_string())),
            Box::new(Expression::Value(Value::Parameter("target".to_string())))
        );
        
        // n = 123 -> true
        assert!(ex.evaluate_expression(&expr, 123, "n"));
        // n = 456 -> false
        assert!(!ex.evaluate_expression(&expr, 456, "n"));
        // wrong variable name -> false
        assert!(!ex.evaluate_expression(&expr, 123, "x"));
    }
}

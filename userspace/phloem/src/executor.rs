use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use crate::gql::{Command, Pattern, Value, NodePattern};
use stem::thing::sys as graph;
use stem::abi::ids::HandleId;
use stem::thing::ThingId;
use crate::{ExecutionResult, ResultValue};

pub struct GraphExecutor {
    bindings: BTreeMap<String, u64>,
}

impl GraphExecutor {
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
        }
    }

    pub fn execute(&mut self, cmd: Command) -> ExecutionResult {
        match cmd {
            Command::Help => self.help(),
            Command::Quit => ExecutionResult::message("Bye."),
            Command::Schema => ExecutionResult::error("Schema not implemented."),
            Command::Merge { pattern, returns } => self.execute_merge(pattern, returns),
            Command::Match { pattern, returns, limit } => self.execute_match(pattern, returns, limit),
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

    fn execute_merge(&mut self, pattern: Pattern, returns: Vec<String>) -> ExecutionResult {
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

    fn execute_match(&mut self, pattern: Pattern, returns: Vec<String>, limit: usize) -> ExecutionResult {
        match pattern {
            Pattern::Node(node_pat) => {
                let mut matched_ids = Vec::new();

                match &node_pat.kind {
                    Some(k) => {
                        let mut candidates = [ThingId::from_u64(0); 128];
                        let count = match graph::find(k.as_str(), &mut candidates) {
                            Ok(c) => c,
                            Err(_) => return ExecutionResult::error("find syscall failed"),
                        };

                        for i in 0..count {
                            let id = candidates[i].to_u64_lossy();
                            if self.matches_props(id, &node_pat.props) {
                                matched_ids.push(id);
                                if matched_ids.len() >= limit {
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        // MATCH (n) - discover from interesting kinds
                        let interesting_kinds = [
                            "svc.Root", "proc.Kernel", "svc.Scheduler",
                            "dev.Host", "dev.bus.Platform", "dev.bus.Pci",
                            "dev.pci.Function", "ui.Crown", "ui.Window",
                            "content.Source", "fs.File", "boot.Module",
                            "Asset", "svc.net.Stack"
                        ];

                        for &kind_name in &interesting_kinds {
                            if matched_ids.len() >= limit {
                                break;
                            }

                            let mut candidates = [ThingId::from_u64(0); 32]; // Small buffer for discovery
                            if let Ok(count) = graph::find(kind_name, &mut candidates) {
                                for i in 0..count {
                                    let id = candidates[i].to_u64_lossy();
                                    if self.matches_props(id, &node_pat.props) {
                                        if !matched_ids.contains(&id) {
                                            matched_ids.push(id);
                                        }
                                        if matched_ids.len() >= limit {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if matched_ids.len() == 1 {
                    self.bindings.insert(node_pat.var.clone(), matched_ids[0]);
                }

                let mut rows = Vec::new();
                for &id in &matched_ids {
                    let mut row = Vec::new();
                    for var in &returns {
                        if var == &node_pat.var {
                            row.push(ResultValue::Node(id));
                        } else {
                            if let Some(&bid) = self.bindings.get(var) {
                                row.push(ResultValue::Node(bid));
                            } else {
                                row.push(ResultValue::Number(0));
                            }
                        }
                    }
                    rows.push(row);
                }

                ExecutionResult::rows(returns, rows)
            }
            Pattern::Edge { src_var, rel: _, dst_var } => {
                let src_id_opt = self.bindings.get(&src_var).cloned();

                if let Some(src_id) = src_id_opt {
                    let edges = match self.get_outbound_edges(src_id) {
                        Ok(e) => e,
                        Err(_) => return ExecutionResult::error("failed to get edges"),
                    };

                    let mut rows = Vec::new();

                    for (_rel_id, dst_id) in edges {
                        self.bindings.insert(dst_var.clone(), dst_id);
                        
                        let mut row = Vec::new();
                        for var in &returns {
                            if var == &src_var {
                                row.push(ResultValue::Node(src_id));
                            } else if var == &dst_var {
                                row.push(ResultValue::Node(dst_id));
                            } else {
                                row.push(ResultValue::Number(0));
                            }
                        }
                        rows.push(row);
                    }

                    ExecutionResult::rows(returns, rows)

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
        let mut buf = [abi::types::Edge::default(); 128];
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
            }
        }
        true
    }

    fn format_results_structured(&self, vars: &[String]) -> ExecutionResult {
        let mut row = Vec::new();
        for var in vars {
            if let Some(&id) = self.bindings.get(var) {
                row.push(ResultValue::Node(id));
            } else {
                row.push(ResultValue::Number(0));
            }
        }
        ExecutionResult::rows(vars.to_vec(), alloc::vec![row])
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
}

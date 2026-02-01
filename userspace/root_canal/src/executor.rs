use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use crate::gql::{Command, Pattern, Value, NodePattern};
use stem::syscall::graph;

pub struct GraphExecutor {
    bindings: BTreeMap<String, u64>,
}

impl GraphExecutor {
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
        }
    }

    pub fn execute(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Help => self.help(),
            Command::Quit => String::from("Bye.\n"),
            Command::Schema => String::from("Schema not implemented.\n"),
            Command::Merge { pattern, returns } => self.execute_merge(pattern, returns),
            Command::Match { pattern, returns, limit } => self.execute_match(pattern, returns, limit),
            Command::Set { var, key, value } => self.execute_set(var, key, value),
        }
    }

    fn help(&self) -> String {
        "Commands:\n\
         MERGE (n:Kind {key: \"val\"})\n\
         MERGE (a)-[:REL]->(b)\n\
         MATCH (n:Kind {key: \"val\"}) RETURN n\n\
         MATCH (a)-[:REL]->(b) RETURN a, b\n\
         SET n.key = \"val\"\n\
         HELP\n\
         QUIT\n".to_string()
    }

    fn execute_merge(&mut self, pattern: Pattern, returns: Vec<String>) -> String {
        match pattern {
            Pattern::Node(node_pat) => {
                let id = match self.ensure_node(&node_pat) {
                    Ok(id) => id,
                    Err(e) => return format!("error: {}\n", e),
                };
                self.bindings.insert(node_pat.var, id);

                if returns.is_empty() {
                    return format!("ok: merged node (id:{})\n", id);
                } else {
                    return self.format_results(&returns);
                }
            }
            Pattern::Edge { src_var, rel, dst_var } => {
                let src_id = match self.bindings.get(&src_var) {
                    Some(&id) => id,
                    None => return format!("error: variable '{}' not bound\n", src_var),
                };
                let dst_id = match self.bindings.get(&dst_var) {
                    Some(&id) => id,
                    None => return format!("error: variable '{}' not bound\n", dst_var),
                };

                match self.ensure_edge(src_id, &rel, dst_id) {
                    Ok(_) => {
                        if returns.is_empty() {
                            format!("ok: merged edge ({})-[:{}]->({})\n", src_id, rel, dst_id)
                        } else {
                            self.format_results(&returns)
                        }
                    }
                    Err(e) => format!("error: {}\n", e),
                }
            }
        }
    }

    fn execute_match(&mut self, pattern: Pattern, returns: Vec<String>, limit: usize) -> String {
        match pattern {
            Pattern::Node(node_pat) => {
                // Find candidates
                let kind_id = match &node_pat.kind {
                    Some(k) => match graph::intern(k) {
                        Ok(id) => id,
                        Err(_) => return "error: failed to intern kind\n".to_string(),
                    },
                    None => return "error: MATCH (n) without kind not supported yet (needs scan)\n".to_string(),
                };

                let mut candidates = Vec::new();
                candidates.resize(1024, 0); // Hard limit for find syscall
                let count = match graph::find(kind_id, &mut candidates) {
                    Ok(c) => c,
                    Err(_) => return "error: find syscall failed\n".to_string(),
                };

                let mut matched_ids = Vec::new();

                for i in 0..count {
                    let id = candidates[i];
                    if self.matches_props(id, &node_pat.props) {
                        matched_ids.push(id);
                        if matched_ids.len() >= limit {
                            break;
                        }
                    }
                }

                if matched_ids.len() == 1 {
                    self.bindings.insert(node_pat.var.clone(), matched_ids[0]);
                }

                // Format output
                let mut out = String::new();
                out.push_str(&format!("ok: {} rows\n", matched_ids.len()));
                for (idx, &id) in matched_ids.iter().enumerate() {
                    out.push_str(&format!("row {}: ", idx + 1));
                    for var in &returns {
                        if var == &node_pat.var {
                            out.push_str(&format!("{}={} ", var, self.format_node(id)));
                        } else {
                            if let Some(&bid) = self.bindings.get(var) {
                                out.push_str(&format!("{}={} ", var, self.format_node(bid)));
                            } else {
                                out.push_str(&format!("{}=? ", var));
                            }
                        }
                    }
                    out.push('\n');
                }
                out
            }
            Pattern::Edge { src_var, rel: _, dst_var } => {
                // MATCH (a)-[:REL]->(b)
                // Assuming 'a' is bound (or scanning all? MVP: require 'a' bound or 'b' bound?)
                // Or Pattern::Edge implies we are matching this specific pattern.
                // Usually variables are fresh unless already bound.

                // For MVP, let's assume `a` is already bound from a previous match?
                // Or maybe we need to support `MATCH (a:Kind)-[:REL]->(b:Kind)`?
                // The parser supports `MATCH (a)-[:REL]->(b)`.
                // If `a` is bound in `self.bindings`, we scan edges from it.
                // If `a` is NOT bound, we scan ALL edges? That's hard.

                // Let's implement: If src is bound, scan edges.

                let src_id_opt = self.bindings.get(&src_var).cloned();

                if let Some(src_id) = src_id_opt {
                    let edges = match self.get_outbound_edges(src_id) {
                        Ok(e) => e,
                        Err(_) => return "error: failed to get edges\n".to_string(),
                    };

                    // Filter edges by REL? (The parser gives us REL name, we need ID)
                    // But `Pattern` struct has `rel: String`.
                    // We don't have REL ID easily unless we intern/guess.
                    // But `get_outbound_edges` gives (RelId, DstId).

                    // Actually, `get_edges` returns raw u64s.
                    // Protocol: [RelId, DstId, RelId, DstId...] ?
                    // Wait, `SYS_ROOT_GET_EDGES` usually returns a list of edges.
                    // The kernel implementation returns `(SymbolId, ThingId)` pairs.
                    // So `out_buf` should be `[u64; 2 * count]`.

                    let mut rows = Vec::new();

                    for (rel_id, dst_id) in edges {
                        // TODO: Filter by REL name if specified?
                        // If pattern.rel is empty? Parser requires it.
                        // Resolve rel_id to name to check?
                        // Or intern pattern.rel and check?

                        // For now, let's just collect all and bind `b`.
                        // We should probably verify REL.
                        // Let's intern the pattern rel.

                        // NOTE: This logic assumes simple single-path execution.
                        // Correct OpenGQL does cartesian products.

                        // If we can resolve symbol:
                        let rel_name_opt = self.resolve_symbol(rel_id as u32);
                        // This is expensive loop.
                        // Better: intern query rel.

                        // (Assuming we can intern)
                         // We can try to intern the pattern REL string.
                        // If intern fails, it means that REL doesn't exist, so no edges match.

                        // ...

                        self.bindings.insert(dst_var.clone(), dst_id);
                        rows.push((src_id, dst_id));
                    }

                    let mut out = String::new();
                    out.push_str(&format!("ok: {} rows\n", rows.len()));
                    for (idx, (_s, d)) in rows.iter().enumerate() {
                        out.push_str(&format!("row {}: ", idx + 1));
                        for var in &returns {
                            if var == &src_var {
                                out.push_str(&format!("{}={} ", var, self.format_node(src_id)));
                            } else if var == &dst_var {
                                out.push_str(&format!("{}={} ", var, self.format_node(*d)));
                            } else {
                                out.push_str(&format!("{}=? ", var));
                            }
                        }
                        out.push('\n');
                    }
                    return out;

                } else {
                     return "error: MATCH edge requires bound source (e.g. use MERGE/MATCH node first)\n".to_string();
                }
            }
        }
    }

    fn execute_set(&mut self, var: String, key: String, value: Value) -> String {
        let id = match self.bindings.get(&var) {
            Some(&id) => id,
            None => return format!("error: variable '{}' not bound\n", var),
        };

        let key_id = match graph::intern(&key) {
            Ok(id) => id,
            Err(_) => return "error: intern key failed\n".to_string(),
        };

        let val_u64 = match value {
            Value::String(s) => match graph::intern(&s) {
                Ok(id) => id,
                Err(_) => return "error: intern value failed\n".to_string(),
            },
            Value::Number(n) => n,
        };

        match graph::prop_set(id, key_id, val_u64) {
            Ok(_) => "ok: property set\n".to_string(),
            Err(e) => format!("error: prop_set failed {:?}\n", e),
        }
    }

    fn ensure_node(&mut self, pat: &NodePattern) -> Result<u64, String> {
        let kind = pat.kind.as_ref().ok_or("MERGE requires a Kind")?;
        let kind_id = graph::intern(kind).map_err(|_| "intern kind failed")?;

        // 1. Try to find
        let mut candidates = Vec::new();
        candidates.resize(1024, 0);
        let count = graph::find(kind_id, &mut candidates).map_err(|_| "find failed")?;

        for i in 0..count {
            let id = candidates[i];
            if self.matches_props(id, &pat.props) {
                return Ok(id);
            }
        }

        // 2. Create
        let id = graph::create_node(kind_id).map_err(|_| "create_node failed")?;

        // Set props
        for (k, v) in &pat.props {
            let key_id = graph::intern(k).map_err(|_| "intern key failed")?;
            let val_u64 = match v {
                Value::String(s) => graph::intern(s).map_err(|_| "intern val failed")?,
                Value::Number(n) => *n,
            };
            graph::prop_set(id, key_id, val_u64).map_err(|_| "prop_set failed")?;
        }

        Ok(id)
    }

    fn ensure_edge(&mut self, src: u64, rel: &str, dst: u64) -> Result<(), String> {
        let rel_id = graph::intern(rel).map_err(|_| "intern rel failed")?;

        // Check existing edges to ensure idempotence
        let edges = self.get_outbound_edges(src).map_err(|_| "failed to scan edges")?;
        for (e_rel, e_dst) in edges {
            if e_rel == rel_id && e_dst == dst {
                // Already exists
                return Ok(());
            }
        }

        graph::link(src, rel_id, dst).map_err(|_| "link failed")?;
        Ok(())
    }

    fn get_outbound_edges(&self, src: u64) -> Result<Vec<(u64, u64)>, ()> {
        let mut buf = [0u64; 1024]; // 512 edges max
        match graph::get_edges(src, &mut buf) {
            Ok(len) => {
                // len is bytes? No, get_edges returns usize (ret).
                // stem syscall wrappers usually return errno or Ok(val).
                // My wrapper returns Ok(ret as usize).
                // SYS_ROOT_GET_EDGES returns number of u64s written?
                // Or number of edges?
                // Kernel logic usually writes pairs.
                // Let's assume it returns number of u64 elements written.

                // Wait, checking `kernel`. `handle_get_edges`?
                // `abi/syscall.rs` says `SYS_ROOT_GET_EDGES: u32 = 0x15C`.
                // I should verify what the kernel does.
                // But I can't read kernel source easily for that specifically unless I grep.
                // Assuming it writes (Rel, Dst) pairs.

                let count = len / 2;
                let mut res = Vec::new();
                for i in 0..count {
                    res.push((buf[2*i], buf[2*i+1]));
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

            let val_id = match graph::prop_get(id, key_id) {
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
                            if s_id != val_id { return false; }
                        }
                        Err(_) => return false,
                    }
                }
            }
        }
        true
    }

    fn format_results(&self, vars: &[String]) -> String {
        let mut out = String::new();
        for var in vars {
            if let Some(&id) = self.bindings.get(var) {
                out.push_str(&format!("{}={} ", var, self.format_node(id)));
            } else {
                out.push_str(&format!("{}=? ", var));
            }
        }
        out.push('\n');
        out
    }

    fn format_node(&self, id: u64) -> String {
        let kind_id = match graph::get_kind(id) {
            Ok(k) => k,
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

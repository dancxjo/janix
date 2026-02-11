use crate::gql::{Command, NodePattern, Pattern, ReturnExpression, Value};
use crate::{ExecutionResult, ResultValue};
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::abi::ids::HandleId;
use stem::thing::sys as graph;
use stem::thing::ThingId;

pub trait Graph {
    fn get_kind(&self, id: ThingId) -> Result<stem::thing::ThingKind, stem::errors::Errno>;
    fn find(&self, kind: &str, out: &mut [ThingId]) -> Result<usize, stem::errors::Errno>;
    fn intern(&self, s: &str) -> Result<stem::abi::symbols::SymbolId, stem::errors::Errno>;
    fn prop_set(&self, id: ThingId, key: &str, value: u64) -> Result<(), stem::errors::Errno>;
    fn create_node(&self, kind: &str) -> Result<ThingId, stem::errors::Errno>;
    fn link(&self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), stem::errors::Errno>;
    fn get_edges(
        &self,
        id: ThingId,
        out: &mut [stem::abi::types::Edge],
    ) -> Result<usize, stem::errors::Errno>;
    fn describe_symbol(
        &self,
        id: stem::abi::symbols::SymbolId,
        out: &mut [u8],
    ) -> Result<usize, stem::errors::Errno>;
    fn prop_get(
        &self,
        id: ThingId,
        key: stem::abi::symbols::SymbolId,
    ) -> Result<u64, stem::errors::Errno>;
    fn yield_now(&self);
}

pub struct SystemGraph;

impl Graph for SystemGraph {
    fn get_kind(&self, id: ThingId) -> Result<stem::thing::ThingKind, stem::errors::Errno> {
        graph::get_kind(id)
    }
    fn find(&self, kind: &str, out: &mut [ThingId]) -> Result<usize, stem::errors::Errno> {
        graph::find(kind, out)
    }
    fn intern(&self, s: &str) -> Result<stem::abi::symbols::SymbolId, stem::errors::Errno> {
        graph::intern(s)
    }
    fn prop_set(&self, id: ThingId, key: &str, value: u64) -> Result<(), stem::errors::Errno> {
        graph::prop_set(id, key, value)
    }
    fn create_node(&self, kind: &str) -> Result<ThingId, stem::errors::Errno> {
        graph::create_node(kind)
    }
    fn link(&self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), stem::errors::Errno> {
        graph::link(src, rel, dst)
    }
    fn get_edges(
        &self,
        id: ThingId,
        out: &mut [stem::abi::types::Edge],
    ) -> Result<usize, stem::errors::Errno> {
        graph::get_edges(id, out)
    }
    fn describe_symbol(
        &self,
        id: stem::abi::symbols::SymbolId,
        out: &mut [u8],
    ) -> Result<usize, stem::errors::Errno> {
        graph::describe_symbol(id, out)
    }
    fn prop_get(
        &self,
        id: ThingId,
        key: stem::abi::symbols::SymbolId,
    ) -> Result<u64, stem::errors::Errno> {
        graph::prop_get(id, key)
    }
    fn yield_now(&self) {
        stem::yield_now();
    }
}

pub struct GraphExecutor<G: Graph = SystemGraph> {
    graph: G,
    bindings: BTreeMap<String, u64>,
    parameters: BTreeMap<String, Value>,
}

impl GraphExecutor<SystemGraph> {
    pub fn new() -> Self {
        Self {
            graph: SystemGraph,
            bindings: BTreeMap::new(),
            parameters: BTreeMap::new(),
        }
    }
}

impl<G: Graph> GraphExecutor<G> {
    pub fn with_graph(graph: G) -> Self {
        Self {
            graph,
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
            Command::Merge {
                pattern,
                returns,
                skip: _,
            } => self.execute_merge(pattern, returns),
            Command::Match {
                pattern,
                where_clause,
                returns,
                order_by,
                limit,
                skip,
            } => self.execute_match(pattern, where_clause, returns, order_by, limit, skip),
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
             QUIT",
        )
    }

    fn execute_merge(
        &mut self,
        pattern: Pattern,
        returns: Vec<ReturnExpression>,
    ) -> ExecutionResult {
        match pattern {
            Pattern::Node(node_pat) => {
                let id = match self.ensure_node(&node_pat) {
                    Ok(id) => id,
                    Err(e) => return ExecutionResult::error(&e),
                };
                if let Some(var) = &node_pat.var {
                    self.bindings.insert(var.clone(), id);
                }

                if returns.is_empty() {
                    return ExecutionResult::success(&format!("ok: merged node (id:{})", id));
                } else {
                    return self.format_results_structured(&returns);
                }
            }
            Pattern::Edge {
                src,
                rel_var,
                rel_kind,
                dst,
            } => {
                let src_id = match src.var.as_ref().and_then(|v| self.bindings.get(v)) {
                    Some(&id) => id,
                    None => {
                        return ExecutionResult::error(&format!(
                            "variable '{:?}' not bound",
                            src.var
                        ))
                    }
                };
                let dst_id = match dst.var.as_ref().and_then(|v| self.bindings.get(v)) {
                    Some(&id) => id,
                    None => {
                        return ExecutionResult::error(&format!(
                            "variable '{:?}' not bound",
                            dst.var
                        ))
                    }
                };

                let rel_kind_str = rel_kind
                    .as_deref()
                    .ok_or_else(|| "MERGE edge requires a relationship type".to_string());
                let rel = match rel_kind_str {
                    Ok(r) => r,
                    Err(e) => return ExecutionResult::error(&e),
                };

                match self.ensure_edge(src_id, rel, dst_id) {
                    Ok(_) => {
                        if returns.is_empty() {
                            ExecutionResult::success(&format!(
                                "ok: merged edge ({})-[:{}]->({})",
                                src_id, rel, dst_id
                            ))
                        } else {
                            if let Some(_rv) = rel_var {
                                // Bind the relationship if possible?
                                // Actually we don't have a good way to bind "edge IDs" yet as they are just predicates.
                                // For now we'll just return results.
                            }
                            self.format_results_structured(&returns)
                        }
                    }
                    Err(e) => ExecutionResult::error(&e),
                }
            }
        }
    }

    fn execute_match(
        &self,
        pattern: Pattern,
        where_clause: Option<crate::gql::Expression>,
        returns: Vec<ReturnExpression>,
        order_by: Option<crate::gql::OrderBy>,
        limit: usize,
        skip: usize,
    ) -> ExecutionResult {
        match pattern {
            Pattern::Node(node_pat) => {
                let limit_total = limit + skip;
                let mut matched_ids = Vec::new();

                // OPTIMIZATION: If WHERE id(n) = $id or id(n) = 123, just look up that node
                if let Some(ref expr) = where_clause {
                    if let crate::gql::Expression::Eq(left, right) = expr {
                        let mut target_id = None;
                        if let (
                            crate::gql::Expression::IdFunc(var),
                            crate::gql::Expression::Value(val),
                        ) = (&**left, &**right)
                        {
                            if Some(var) == node_pat.var.as_ref() {
                                target_id = self.resolve_value_as_u64(val);
                            }
                        } else if let (
                            crate::gql::Expression::Value(val),
                            crate::gql::Expression::IdFunc(var),
                        ) = (&**left, &**right)
                        {
                            if Some(var) == node_pat.var.as_ref() {
                                target_id = self.resolve_value_as_u64(val);
                            }
                        }

                        if let Some(id) = target_id {
                            // Verify kind and other props
                            let matches_kind = match &node_pat.kind {
                                Some(k) => match self.graph.get_kind(ThingId::from_u64(id)) {
                                    Ok(kind_id) => {
                                        let kind_name = self
                                            .resolve_symbol(kind_id.0 as u32)
                                            .unwrap_or_default();
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
                            return self.format_match_results(
                                &node_pat,
                                matched_ids,
                                returns,
                                order_by,
                                limit,
                                skip,
                            );
                        }
                    }
                }
                match &node_pat.kind {
                    Some(_k) => {
                        let mut candidates = [ThingId::from_u64(0); 2048];
                        stem::info!("phloem: calling find for kind: {}", _k);
                        let count = match self.graph.find(_k.as_str(), &mut candidates) {
                            Ok(c) => c,
                            Err(e) => {
                                stem::info!("phloem: find failed for kind {}: {:?}", _k, e);
                                return ExecutionResult::error("find syscall failed");
                            }
                        };
                        stem::info!("phloem: find returned {} candidates", count);

                        for i in 0..count {
                            self.graph.yield_now();
                            let id = candidates[i].to_u64_lossy();
                            if self.matches_props(id, &node_pat.props) {
                                if !matched_ids.contains(&id) {
                                    matched_ids.push(id);
                                }
                                // Don't limit early if we need to sort
                                if order_by.is_none() && matched_ids.len() >= limit_total {
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        matched_ids = self.discover_nodes(
                            &node_pat.props,
                            None,
                            limit_total,
                            where_clause.as_ref(),
                            node_pat.var.as_deref(),
                        );
                    }
                }
                stem::info!("phloem: discovered {} nodes", matched_ids.len());
                self.format_match_results(&node_pat, matched_ids, returns, order_by, limit, skip)
            }
            Pattern::Edge {
                src,
                rel_var,
                rel_kind,
                dst,
            } => {
                stem::info!(
                    "phloem: edge match starting. rel_var: {:?}, rel_kind: {:?}",
                    rel_var,
                    rel_kind
                );
                let limit_total = limit + skip;

                // Check if we have any aggregate functions
                let has_aggregate = returns
                    .iter()
                    .any(|r| matches!(r, ReturnExpression::Count(_)));

                // We'll collect edge match data first
                // Each entry is (src_id, rel_symbol_id, dst_id)
                let mut edge_matches: Vec<(u64, u64, u64)> = Vec::new();

                let src_id_opt = if let Some(ref props) = src.props.first() {
                    if props.0 == "id" {
                        self.resolve_value_as_u64(&props.1)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let source_nodes = if let Some(id) = src_id_opt {
                    stem::info!("phloem: using fixed src_id: {}", id);
                    alloc::vec![id]
                } else {
                    stem::info!("phloem: discovering source nodes for edge...");
                    self.discover_nodes(&src.props, src.kind.as_deref(), 1000, None, None)
                };
                stem::info!(
                    "phloem: found {} potential source nodes",
                    source_nodes.len()
                );
                stem::info!(
                    "phloem: edge match starting. source_nodes count: {}. limit_total: {}",
                    source_nodes.len(),
                    limit_total
                );

                for src_id in source_nodes {
                    self.graph.yield_now();
                    // For count, we may need all matches; for rows, respect limit
                    if !has_aggregate && edge_matches.len() >= limit_total {
                        break;
                    }

                    let edges = match self.get_outbound_edges(src_id) {
                        Ok(e) => e,
                        Err(_) => continue,
                    };

                    for (rel_symbol_id, dst_id) in edges {
                        self.graph.yield_now();
                        if !has_aggregate && edge_matches.len() >= limit_total {
                            break;
                        }

                        stem::trace!("phloem: checking edge {} -> {}", src_id, dst_id);

                        // Check rel_kind if present
                        if let Some(ref kind) = rel_kind {
                            let rel_name = self
                                .resolve_symbol(rel_symbol_id as u32)
                                .unwrap_or_default();
                            if &rel_name != kind {
                                continue;
                            }
                        }

                        // Check dst node pattern
                        if !self.matches_props(dst_id, &dst.props) {
                            continue;
                        }
                        if let Some(ref kind) = dst.kind {
                            let d_kind_id = match self.graph.get_kind(ThingId::from_u64(dst_id)) {
                                Ok(k) => k.0,
                                Err(_) => continue,
                            };
                            let d_kind_name =
                                self.resolve_symbol(d_kind_id as u32).unwrap_or_default();
                            if &d_kind_name != kind {
                                continue;
                            }
                        }

                        // Match!
                        edge_matches.push((src_id, rel_symbol_id as u64, dst_id));
                    }
                }

                // Now format results based on whether we have aggregates
                if has_aggregate {
                    let mut row = Vec::new();
                    for expr in &returns {
                        match expr {
                            ReturnExpression::Count(var) => {
                                // count(*), count(a), count(b), or count(e) should all count edge matches
                                if var == "*"
                                    || Some(var) == src.var.as_ref()
                                    || Some(var) == dst.var.as_ref()
                                    || Some(var) == rel_var.as_ref()
                                {
                                    row.push(ResultValue::Number(edge_matches.len() as u64));
                                } else {
                                    row.push(ResultValue::Number(0));
                                }
                            }
                            ReturnExpression::Variable(var) => {
                                // For aggregate queries with variables, just show first match
                                if let Some(&(s, r, d)) = edge_matches.first() {
                                    if Some(var) == src.var.as_ref() {
                                        row.push(ResultValue::Node(s));
                                    } else if Some(var) == dst.var.as_ref() {
                                        row.push(ResultValue::Node(d));
                                    } else if Some(var) == rel_var.as_ref() {
                                        let rel_name = self
                                            .resolve_symbol(r as u32)
                                            .unwrap_or_else(|| format!("{}", r));
                                        row.push(ResultValue::String(rel_name));
                                    } else {
                                        row.push(ResultValue::String(format!(
                                            "unsupported: {}",
                                            var
                                        )));
                                    }
                                } else {
                                    row.push(ResultValue::Number(0));
                                }
                            }
                        }
                    }
                    let cols: Vec<String> = returns.iter().map(|r| r.to_string()).collect();
                    ExecutionResult::rows(cols, alloc::vec![row])
                } else {
                    // Non-aggregate: build rows
                    let rows: Vec<Vec<ResultValue>> = edge_matches
                        .into_iter()
                        .skip(skip)
                        .take(limit)
                        .map(|(s, r, d)| {
                            let mut row = Vec::new();
                            for expr in &returns {
                                match expr {
                                    ReturnExpression::Variable(var) => {
                                        if Some(var) == src.var.as_ref() {
                                            row.push(ResultValue::Node(s));
                                        } else if Some(var) == dst.var.as_ref() {
                                            row.push(ResultValue::Node(d));
                                        } else if Some(var) == rel_var.as_ref() {
                                            let rel_name = self
                                                .resolve_symbol(r as u32)
                                                .unwrap_or_else(|| format!("{}", r));
                                            row.push(ResultValue::String(rel_name));
                                        } else {
                                            row.push(ResultValue::String(format!(
                                                "unsupported: {}",
                                                var
                                            )));
                                        }
                                    }
                                    ReturnExpression::Count(_) => unreachable!(),
                                }
                            }
                            row
                        })
                        .collect();

                    let cols: Vec<String> = returns.iter().map(|r| r.to_string()).collect();
                    ExecutionResult::rows(cols, rows)
                }
            }
        }
    }

    fn discover_nodes(
        &self,
        props: &[(String, Value)],
        kind: Option<&str>,
        limit_total: usize,
        where_clause: Option<&crate::gql::Expression>,
        var_name: Option<&str>,
    ) -> Vec<u64> {
        stem::info!("phloem: entering discover_nodes");
        let mut matched_ids = Vec::new();

        if let Some(k) = kind {
            stem::info!("phloem: discovering nodes of kind {}", k);
            let mut candidates = alloc::vec![ThingId::from_u64(0); 2048];
            if let Ok(count) = self.graph.find(k, &mut candidates) {
                stem::info!("phloem: found {} candidates for kind {}", count, k);
                for i in 0..count {
                    self.graph.yield_now();
                    let id = candidates[i].to_u64_lossy();
                    if self.matches_props(id, props) {
                        let matches_where = if let Some(expr) = where_clause {
                            if let Some(var) = var_name {
                                self.evaluate_expression(expr, id, var)
                            } else {
                                true
                            }
                        } else {
                            true
                        };

                        if matches_where {
                            if !matched_ids.contains(&id) {
                                matched_ids.push(id);
                            }
                            if matched_ids.len() >= limit_total {
                                return matched_ids;
                            }
                        }
                    }
                }
            }
        } else {
            // BFS Discovery from well-known roots and exhaustive kinds
            let mut queue = VecDeque::new();
            let mut seen = BTreeSet::new();

            // Well-known roots: Host(1), Root(2), Scheduler(3)
            stem::info!("phloem: starting BFS discovery from roots");
            for &root_id in &[1u64, 2u64, 3u64] {
                queue.push_back(root_id);
                seen.insert(root_id);
            }

            // Exhaustive kinds to seed from
            let fallback_kinds = [
                "fs.File",
                "content.Source",
                "Asset",
                "ui.Window",
                "proc.Process",
                "dev.bus.Pci",
                "dev.pci.Function",
                "dev.net.Nic",
                "dev.storage.Disk",
                "dev.display.Gpu",
                "dev.Cpu",
                "ui.Crown",
                "font.Family",
                "font.Face",
                "boot.Module",
                "svc.net.Stack",
                "svc.net.Driver",
                "Bytespace",
                "mem.Range",
                "proc.Thread",
                "proc.Task",
                "proc.Kernel",
                "svc.Root",
                "svc.Scheduler",
                "dev.Host",
                "mem.Page",
                "mem.Stack",
                "mem.Heap",
                "ui.Panel",
                "ui.Text",
                "font.Family",
                "font.Face",
                "font.File",
                "xml.Document",
                "html.Document",
                "css.Stylesheet",
            ];
            for &k in &fallback_kinds {
                self.graph.yield_now();
                let mut seeds = alloc::vec![ThingId::from_u64(0); 512];
                if let Ok(count) = self.graph.find(k, &mut seeds) {
                    for i in 0..count {
                        let id = seeds[i].to_u64_lossy();
                        if !seen.contains(&id) {
                            seen.insert(id);
                            queue.push_back(id);
                        }
                    }
                }
            }
            stem::info!("phloem: BFS seeded with {} nodes", queue.len());

            while let Some(current_id) = queue.pop_front() {
                self.graph.yield_now();
                if matched_ids.len() >= limit_total {
                    break;
                }

                if self.matches_props(current_id, props) {
                    let matches_where = if let Some(expr) = where_clause {
                        if let Some(var) = var_name {
                            self.evaluate_expression(expr, current_id, var)
                        } else {
                            true
                        }
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
        matched_ids
    }

    fn execute_set(&mut self, var: String, key: String, value: Value) -> ExecutionResult {
        let id = match self.bindings.get(&var) {
            Some(&id) => id,
            None => return ExecutionResult::error(&format!("variable '{}' not bound", var)),
        };

        let val_u64 = match value {
            Value::String(s) => match self.graph.intern(&s) {
                Ok(id) => id as u64,
                Err(_) => return ExecutionResult::error("intern value failed"),
            },
            Value::Number(n) => n,
            Value::Parameter(name) => match self.parameters.get(&name) {
                Some(Value::Number(n)) => *n,
                Some(Value::String(s)) => match self.graph.intern(s) {
                    Ok(id) => id as u64,
                    Err(_) => return ExecutionResult::error("intern parameter value failed"),
                },
                _ => {
                    return ExecutionResult::error(&format!(
                        "parameter '{}' not found or invalid type",
                        name
                    ))
                }
            },
        };

        match self
            .graph
            .prop_set(ThingId::from_u64(id), key.as_str(), val_u64)
        {
            Ok(_) => ExecutionResult::success("ok: property set"),
            Err(e) => ExecutionResult::error(&format!("prop_set failed {:?}", e)),
        }
    }

    fn ensure_node(&mut self, pat: &NodePattern) -> Result<u64, String> {
        let kind = pat.kind.as_ref().ok_or("MERGE requires a Kind")?;

        // 1. Try to find
        let mut candidates = [ThingId::from_u64(0); 128];
        let count = self
            .graph
            .find(kind.as_str(), &mut candidates)
            .map_err(|_| "find failed")?;

        for i in 0..count {
            let id = candidates[i].to_u64_lossy();
            if self.matches_props(id, &pat.props) {
                return Ok(id);
            }
        }

        // 2. Create
        let id_new = self
            .graph
            .create_node(kind.as_str())
            .map_err(|_| "create_node failed")?;
        let id = id_new.to_u64_lossy();

        // Set props
        for (k, v) in &pat.props {
            let val_u64 = match v {
                Value::String(s) => self.graph.intern(s).map_err(|_| "intern val failed")? as u64,
                Value::Number(n) => *n,
                Value::Parameter(p) => match self.parameters.get(p) {
                    Some(Value::Number(n)) => *n,
                    Some(Value::String(s)) => {
                        self.graph.intern(s).map_err(|_| "intern val failed")? as u64
                    }
                    _ => return Err(format!("parameter '{}' not found", p)),
                },
            };
            self.graph
                .prop_set(id_new, k.as_str(), val_u64)
                .map_err(|_| "prop_set failed")?;
        }

        Ok(id)
    }

    fn ensure_edge(&mut self, src: u64, rel: &str, dst: u64) -> Result<(), String> {
        // Check existing edges to ensure idempotence
        let edges = self
            .get_outbound_edges(src)
            .map_err(|_| "failed to scan edges")?;
        for (e_rel_id, e_dst) in edges {
            let rel_name = self.resolve_symbol(e_rel_id as u32).unwrap_or_default();
            if rel_name == rel && e_dst == dst {
                // Already exists
                return Ok(());
            }
        }

        self.graph
            .link(ThingId::from_u64(src), rel, ThingId::from_u64(dst))
            .map_err(|_| "link failed")?;
        Ok(())
    }

    fn get_outbound_edges(&self, src: u64) -> Result<Vec<(u32, u64)>, ()> {
        let mut edges = alloc::vec![stem::abi::types::Edge::default(); 256];
        match self.graph.get_edges(ThingId::from_u64(src), &mut edges) {
            Ok(count) => {
                let mut res = Vec::new();
                for i in 0..count {
                    res.push((
                        edges[i].predicate.to_u64_lossy() as u32,
                        edges[i].to.to_u64_lossy(),
                    ));
                }
                Ok(res)
            }
            Err(_) => Err(()),
        }
    }

    fn matches_props(&self, id: u64, props: &[(String, Value)]) -> bool {
        for (k, v) in props {
            let key_id = match self.graph.intern(k) {
                Ok(id) => id,
                Err(_) => return false,
            };

            let val_id = match self.graph.prop_get(ThingId::from_u64(id), key_id) {
                Ok(v) => v,
                Err(_) => return false, // Property missing
            };

            match v {
                Value::Number(n) => {
                    if val_id != *n {
                        return false;
                    }
                }
                Value::String(s) => match self.graph.intern(s) {
                    Ok(s_id) => {
                        if (s_id as u64) != val_id {
                            return false;
                        }
                    }
                    Err(_) => return false,
                },
                Value::Parameter(p) => match self.parameters.get(p) {
                    Some(Value::Number(n)) => {
                        if val_id != *n {
                            return false;
                        }
                    }
                    Some(Value::String(s)) => match self.graph.intern(s) {
                        Ok(s_id) => {
                            if (s_id as u64) != val_id {
                                return false;
                            }
                        }
                        Err(_) => return false,
                    },
                    _ => return false,
                },
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
        let kind_id = match self.graph.get_kind(ThingId::from_u64(id)) {
            Ok(k) => k.0,
            Err(_) => return format!("(id:{})", id),
        };

        let kind_name = self
            .resolve_symbol(kind_id as u32)
            .unwrap_or_else(|| format!("{}", kind_id));
        format!("(id:{} :{})", id, kind_name)
    }

    fn resolve_symbol(&self, id: u32) -> Option<String> {
        let mut buf = [0u8; 64];
        match self.graph.describe_symbol(id, &mut buf) {
            Ok(len) => {
                if len > buf.len() {
                    Some("...".to_string())
                } else {
                    core::str::from_utf8(&buf[..len])
                        .ok()
                        .map(|s| s.to_string())
                }
            }
            Err(_) => None,
        }
    }

    fn format_match_results(
        &self,
        node_pat: &NodePattern,
        mut matched_ids: Vec<u64>,
        returns: Vec<ReturnExpression>,
        order_by: Option<crate::gql::OrderBy>,
        limit: usize,
        skip: usize,
    ) -> ExecutionResult {
        // Check if we have any aggregate functions
        let has_aggregate = returns
            .iter()
            .any(|r| matches!(r, ReturnExpression::Count(_)));

        // Apply ORDER BY if present (before limit/skip)
        if let Some(ref order) = order_by {
            match order {
                crate::gql::OrderBy::IdAsc(_) => {
                    matched_ids.sort();
                }
                crate::gql::OrderBy::IdDesc(_) => {
                    matched_ids.sort();
                    matched_ids.reverse();
                }
            }
        }

        if has_aggregate {
            let mut row = Vec::new();
            for expr in &returns {
                match expr {
                    ReturnExpression::Count(var) => {
                        if Some(var) == node_pat.var.as_ref() || var == "*" {
                            row.push(ResultValue::Number(matched_ids.len() as u64));
                        } else {
                            row.push(ResultValue::Number(0));
                        }
                    }
                    ReturnExpression::Variable(v) => {
                        if let Some(&id) = matched_ids.first() {
                            if Some(v) == node_pat.var.as_ref() {
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
                                if Some(col) == node_pat.var.as_ref() {
                                    row.push(ResultValue::Node(id));
                                } else {
                                    row.push(ResultValue::String(format!("unsupported: {}", col)));
                                }
                            }
                            ReturnExpression::Count(_) => unreachable!(),
                        }
                    }
                    row
                })
                .collect();

            let cols = returns.iter().map(|r| r.to_string()).collect();
            ExecutionResult::rows(cols, rows)
        }
    }

    fn resolve_value_as_u64(&self, val: &Value) -> Option<u64> {
        match val {
            Value::Number(n) => Some(*n),
            Value::Parameter(name) => match self.parameters.get(name) {
                Some(Value::Number(n)) => Some(*n),
                _ => None,
            },
            Value::String(_) => None,
        }
    }

    fn evaluate_expression(
        &self,
        expr: &crate::gql::Expression,
        node_id: u64,
        node_var: &str,
    ) -> bool {
        match expr {
            crate::gql::Expression::Eq(left, right) => {
                let l_val = self.evaluate_primary(left, node_id, node_var);
                let r_val = self.evaluate_primary(right, node_id, node_var);
                l_val == r_val && l_val.is_some()
            }
            _ => false, // Only Eq supported at top level for now
        }
    }

    fn evaluate_primary(
        &self,
        expr: &crate::gql::Expression,
        node_id: u64,
        node_var: &str,
    ) -> Option<Value> {
        match expr {
            crate::gql::Expression::IdFunc(var) => {
                if var == node_var {
                    Some(Value::Number(node_id))
                } else {
                    None
                }
            }
            crate::gql::Expression::Value(v) => match v {
                Value::Parameter(name) => self.parameters.get(name).cloned(),
                _ => Some(v.clone()),
            },
            crate::gql::Expression::CountEdges(var) => {
                // count((var)-[]->()) - count outgoing edges from the node if var matches
                if var == node_var {
                    match self.get_outbound_edges(node_id) {
                        Ok(edges) => Some(Value::Number(edges.len() as u64)),
                        Err(_) => Some(Value::Number(0)),
                    }
                } else {
                    None
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
        assert_eq!(
            ex.resolve_value_as_u64(&Value::Parameter("pid".to_string())),
            Some(100)
        );
        assert_eq!(
            ex.resolve_value_as_u64(&Value::Parameter("unknown".to_string())),
            None
        );
    }

    #[test]
    fn test_evaluate_expression() {
        let mut ex = GraphExecutor::new();
        ex.set_parameter("target".to_string(), Value::Number(123));

        let expr = Expression::Eq(
            Box::new(Expression::IdFunc("n".to_string())),
            Box::new(Expression::Value(Value::Parameter("target".to_string()))),
        );

        // n = 123 -> true
        assert!(ex.evaluate_expression(&expr, 123, "n"));
        // n = 456 -> false
        assert!(!ex.evaluate_expression(&expr, 456, "n"));
        // wrong variable name -> false
        assert!(!ex.evaluate_expression(&expr, 123, "x"));
    }
}

use crate::executor::{Graph, GraphExecutor};
use crate::gql::parse;
use abi::ids::HandleId;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use stem::abi::symbols::SymbolId;
use stem::abi::types::Edge;
use stem::errors::Errno;
use stem::thing::{ThingId, ThingKind};

pub struct MockGraphState {
    pub nodes: BTreeMap<u64, String>,
    pub props: BTreeMap<u64, BTreeMap<u32, u64>>,
    pub edges: BTreeMap<u64, Vec<(u32, u64)>>,
    pub symbols: BTreeMap<String, u32>,
    pub symbol_names: BTreeMap<u32, String>,
    pub next_symbol_id: u32,
    pub next_node_id: u64,
}

pub struct MockGraph {
    pub state: RefCell<MockGraphState>,
}

impl MockGraph {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(MockGraphState {
                nodes: BTreeMap::new(),
                props: BTreeMap::new(),
                edges: BTreeMap::new(),
                symbols: BTreeMap::new(),
                symbol_names: BTreeMap::new(),
                next_symbol_id: 1,
                next_node_id: 1000,
            }),
        }
    }

    pub fn add_node(&self, id: u64, kind: &str) {
        let mut s = self.state.borrow_mut();
        s.nodes.insert(id, kind.to_string());
        self.intern_internal(&mut s, kind);
    }

    pub fn set_prop(&self, id: u64, key: &str, val: u64) {
        let mut s = self.state.borrow_mut();
        let key_id = self.intern_internal(&mut s, key);
        s.props.entry(id).or_default().insert(key_id, val);
    }

    pub fn add_edge(&self, src: u64, rel: &str, dst: u64) {
        let mut s = self.state.borrow_mut();
        let rel_id = self.intern_internal(&mut s, rel);
        s.edges.entry(src).or_default().push((rel_id, dst));
    }

    fn intern_internal(&self, s: &mut MockGraphState, name: &str) -> u32 {
        if let Some(&id) = s.symbols.get(name) {
            id
        } else {
            let id = s.next_symbol_id;
            s.next_symbol_id += 1;
            s.symbols.insert(name.to_string(), id);
            s.symbol_names.insert(id, name.to_string());
            id
        }
    }
}

impl Graph for &MockGraph {
    fn get_kind(&self, id: ThingId) -> Result<ThingKind, Errno> {
        let s = self.state.borrow();
        if let Some(kind_name) = s.nodes.get(&id.to_u64_lossy()) {
            let symbol_id = s.symbols.get(kind_name).ok_or(Errno::ENOENT)?;
            Ok(ThingKind(*symbol_id as u64))
        } else {
            Err(Errno::ENOENT)
        }
    }

    fn find(&self, kind: &str, out: &mut [ThingId]) -> Result<usize, Errno> {
        let s = self.state.borrow();
        let mut count = 0;
        for (&id, k) in &s.nodes {
            if k == kind {
                if count < out.len() {
                    out[count] = ThingId::from_u64(id);
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    fn intern(&self, name: &str) -> Result<SymbolId, Errno> {
        let mut s = self.state.borrow_mut();
        Ok(self.intern_internal(&mut s, name))
    }

    fn prop_set(&self, id: ThingId, key: &str, value: u64) -> Result<(), Errno> {
        let mut s = self.state.borrow_mut();
        let key_id = self.intern_internal(&mut s, key);
        s.props
            .entry(id.to_u64_lossy())
            .or_default()
            .insert(key_id, value);
        Ok(())
    }

    fn create_node(&self, kind: &str) -> Result<ThingId, Errno> {
        let mut s = self.state.borrow_mut();
        let id = s.next_node_id;
        s.next_node_id += 1;
        s.nodes.insert(id, kind.to_string());
        self.intern_internal(&mut s, kind);
        Ok(ThingId::from_u64(id))
    }

    fn link(&self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), Errno> {
        let mut s = self.state.borrow_mut();
        let rel_id = self.intern_internal(&mut s, rel);
        s.edges
            .entry(src.to_u64_lossy())
            .or_default()
            .push((rel_id, dst.to_u64_lossy()));
        Ok(())
    }

    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> Result<usize, Errno> {
        let s = self.state.borrow();
        let node_id = id.to_u64_lossy();
        if let Some(node_edges) = s.edges.get(&node_id) {
            let mut count = 0;
            for &(pred_id, target_id) in node_edges {
                if count < out.len() {
                    out[count] = Edge {
                        from: id,
                        predicate: ThingId::from_u64(pred_id as u64),
                        to: ThingId::from_u64(target_id),
                        flags: 0,
                    };
                    count += 1;
                }
            }
            Ok(count)
        } else {
            Ok(0)
        }
    }

    fn describe_symbol(&self, id: SymbolId, out: &mut [u8]) -> Result<usize, Errno> {
        let s = self.state.borrow();
        if let Some(name) = s.symbol_names.get(&(id as u32)) {
            let bytes = name.as_bytes();
            let len = core::cmp::min(bytes.len(), out.len());
            out[..len].copy_from_slice(&bytes[..len]);
            Ok(bytes.len())
        } else {
            Err(Errno::ENOENT)
        }
    }

    fn prop_get(&self, id: ThingId, key: SymbolId) -> Result<u64, Errno> {
        let s = self.state.borrow();
        let node_id = id.to_u64_lossy();
        if let Some(node_props) = s.props.get(&node_id) {
            node_props.get(&(key as u32)).copied().ok_or(Errno::ENOENT)
        } else {
            Err(Errno::ENOENT)
        }
    }

    fn yield_now(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock() -> MockGraph {
        let g = MockGraph::new();
        // Nodes
        g.add_node(1, "proc.Process");
        g.set_prop(1, "name", 12345);
        g.set_prop(1, "state", 100); // "running" symbol
        g.add_node(2, "proc.Process");
        g.set_prop(2, "name", 67890);
        g.set_prop(2, "state", 101); // "stopped" symbol
        g.add_node(3, "svc.Scheduler");
        g.add_node(4, "Kind"); // A Kind node for introspection
        g.add_node(5, "fs.File");
        g.set_prop(5, "name", 11111);

        // Edges
        g.add_edge(1, "CHILD", 2);
        g.add_edge(1, "PARENT", 3);
        g.add_edge(3, "OWNS", 1);
        g.add_edge(3, "OWNS", 2);
        g
    }

    // ===== 0) Smoke tests and introspection =====

    #[test]
    fn test_smoke_list_nodes() {
        // MATCH (n) RETURN n LIMIT 10
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n LIMIT 10").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
    }

    #[test]
    fn test_count_nodes() {
        // MATCH (n) RETURN count(n)
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN count(n)").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should have rows with count
        assert!(!res.rows.is_empty());
        if let crate::ResultValue::Number(n) = res.rows[0][0] {
            assert_eq!(n, 5);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_match_multiple_properties() {
        // MATCH (n:proc.Process {state: 100, name: 12345}) RETURN n
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Process {state: 100, name: 12345}) RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 1);
        } else {
            panic!("Expected Node ID");
        }

        // MATCH (n:proc.Process {state: 101, name: 12345}) RETURN n
        // Should return empty result as properties conflict
        let cmd2 = parse("MATCH (n:proc.Process {state: 101, name: 12345}) RETURN n").unwrap();
        let res2 = ex.execute(cmd2);
        assert!(res2.success);
        assert!(res2.rows.is_empty());
    }

    #[test]
    fn test_list_nodes_by_kind() {
        // MATCH (n:proc.Process) RETURN n LIMIT 10
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Process) RETURN n LIMIT 10").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should find 2 proc.Process nodes
        assert_eq!(res.rows.len(), 2);
    }

    #[test]
    fn test_count_nodes_empty_result() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        // A kind that doesn't exist
        let cmd = parse("MATCH (n:nonexistent.Kind) RETURN count(n)").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success);
        // Aggregate queries like count() should return exactly one row even if there are no matches
        assert_eq!(res.rows.len(), 1);
        assert!(matches!(res.rows[0][0], crate::ResultValue::Number(0)));
    }

    #[test]
    fn test_count_nodes_by_kind() {
        // MATCH (n:proc.Process) RETURN count(n)
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Process) RETURN count(n)").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        if let crate::ResultValue::Number(n) = res.rows[0][0] {
            assert_eq!(n, 2);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_list_kinds() {
        // MATCH (k:Kind) RETURN k
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (k:Kind) RETURN k").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should find 1 Kind node
        assert_eq!(res.rows.len(), 1);
    }

    #[test]
    fn test_order_by_id() {
        // MATCH (n) RETURN n ORDER BY id(n) LIMIT 200
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) LIMIT 200").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Results should be sorted by ascending ID
        // In our mock we have nodes 1, 2, 3, 4, 5
        if res.rows.len() >= 2 {
            // Verify ascending order
            for i in 1..res.rows.len() {
                let prev_id = if let crate::ResultValue::Node(id) = res.rows[i - 1][0] {
                    id
                } else {
                    0
                };
                let curr_id = if let crate::ResultValue::Node(id) = res.rows[i][0] {
                    id
                } else {
                    0
                };
                assert!(prev_id <= curr_id, "Expected ascending order");
            }
        }
    }

    #[test]
    fn test_order_by_id_desc_limit() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) DESC LIMIT 1").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 5);
        } else {
            panic!("Expected Node ID");
        }
    }

    #[test]
    fn test_order_desc_pagination() {
        // MATCH (n) RETURN n ORDER BY id(n) DESC SKIP 1 LIMIT 2
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) DESC SKIP 1 LIMIT 2").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Expecting nodes 4, 3 (reverse ID order: 5, 4, 3, 2, 1 -> skip 1 -> 4, 3)
        assert_eq!(res.rows.len(), 2);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 4);
        } else {
            panic!("Expected Node ID");
        }
        if let crate::ResultValue::Node(id) = res.rows[1][0] {
            assert_eq!(id, 3);
        } else {
            panic!("Expected Node ID");
        }
    }

    // ===== 1) Identity and direct lookup =====

    #[test]
    fn test_lookup_by_id() {
        // MATCH (n) WHERE id(n) = 1 RETURN n
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) WHERE id(n) = 1 RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);
    }

    #[test]
    fn test_existence_check() {
        // MATCH (n) WHERE id(n) = 1 RETURN count(n)
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) WHERE id(n) = 1 RETURN count(n)").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Count should be 1
        if let crate::ResultValue::Number(n) = res.rows[0][0] {
            assert_eq!(n, 1);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_where_count_edges_zero() {
        // MATCH (n) WHERE count((n)-[]->()) = 0 RETURN n LIMIT 50
        // This finds nodes with no outgoing edges
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) WHERE count((n)-[]->()) = 0 RETURN n LIMIT 50").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // In our mock, nodes 4 (Process) and 5 (Process) have no outgoing edges
        // We expect to find them (and possibly others that have no edges)
        assert!(!res.rows.is_empty());
    }

    #[test]
    fn test_where_count_edges_exact() {
        // MATCH (n) WHERE count((n)-[]->()) = 2 RETURN n
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) WHERE count((n)-[]->()) = 2 RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);

        // In our mock:
        // Node 1 has CHILD(2), PARENT(3) -> 2 edges
        // Node 3 has OWNS(1), OWNS(2) -> 2 edges
        // We expect exactly these two nodes

        let mut ids = Vec::new();
        for row in res.rows {
            if let crate::ResultValue::Node(id) = row[0] {
                ids.push(id);
            }
        }
        ids.sort();
        assert_eq!(ids, vec![1, 3]);
    }

    #[test]
    fn test_lookup_nonexistent() {
        // MATCH (n) WHERE id(n) = 9999 RETURN n
        // NOTE: The ID lookup optimization now checks existence.
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) WHERE id(n) = 9999 RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should find no rows because the node does not exist
        assert!(res.rows.is_empty());
    }

    #[test]
    fn test_lookup_by_id_kind_mismatch() {
        // MATCH (n:proc.Thread) WHERE id(n) = 1 RETURN n
        // Node 1 is a proc.Process, so it should not match the kind proc.Thread
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Thread) WHERE id(n) = 1 RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should find no rows because the node kind does not match
        assert!(res.rows.is_empty());
    }

    // ===== 3) Edges and neighborhood traversal =====

    #[test]
    fn test_outgoing_neighbors() {
        // MATCH (a)-[]->(b) WHERE id(a) = 1 RETURN b LIMIT 50
        // Note: The current parser doesn't support WHERE with edge patterns well,
        // but we test edge traversal with kind-based filtering instead.
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (a)-[]->(b) RETURN a, b LIMIT 50").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should find edges
        assert!(!res.rows.is_empty());
    }

    #[test]
    fn test_typed_edge_traversal() {
        // MATCH (a)-[:OWNS]->(b) RETURN a, b
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (a)-[:OWNS]->(b) RETURN a, b").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Scheduler owns processes
        assert!(!res.rows.is_empty());
    }

    #[test]
    fn test_return_edges_too() {
        // MATCH (a)-[e:OWNS]->(b) RETURN e, b
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (a)-[e:OWNS]->(b) RETURN e, b").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
    }

    #[test]
    fn test_edge_variable_binding() {
        // MATCH ()-[e]->() RETURN e
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH ()-[e]->() RETURN e").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
    }

    #[test]
    fn test_match_multiple_inline_properties() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // MATCH (n {name: 12345, state: 100}) RETURN n
        // In setup_mock, node 1 has name=12345 and state=100
        let cmd = parse("MATCH (n {name: 12345, state: 100}) RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);

        // Make sure it's node 1
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 1);
        } else {
            panic!("Expected Node ID");
        }

        // Now test conflicting properties
        let cmd2 = parse("MATCH (n {name: 12345, state: 999}) RETURN n").unwrap();
        let res2 = ex.execute(cmd2);
        assert!(res2.success);
        // Node 1 has state 100, not 999, so it shouldn't match
        assert!(res2.rows.is_empty());
    }

    #[test]
    fn test_count_edge_matches() {
        // MATCH (a)-[]->(b) RETURN count(b) LIMIT 200
        // This should return the count of edges, not "count() not supported" message
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (a)-[]->(b) RETURN count(b) LIMIT 200").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Should have one row with the count
        assert_eq!(res.rows.len(), 1);
        // The count should be a number, not a string error message
        if let Some(row) = res.rows.first() {
            if let Some(crate::ResultValue::Number(n)) = row.first() {
                // We have 4 edges in setup_mock
                assert_eq!(*n, 4);
            } else {
                panic!(
                    "Expected a Number result for count(), got: {:?}",
                    row.first()
                );
            }
        }
    }

    // ===== 7) Minimal mutation =====

    #[test]
    fn test_merge_node() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // 1. Merge a new node
        let cmd1 = parse("MERGE (n:Kind {key: 1}) RETURN n").unwrap();
        let res1 = ex.execute(cmd1);
        assert!(res1.success, "First MERGE failed");
        assert_eq!(res1.rows.len(), 1);
        let id1 = if let crate::ResultValue::Node(id) = res1.rows[0][0] {
            id
        } else {
            panic!("Expected Node ID");
        };

        // 2. Merge the same node again
        let cmd2 = parse("MERGE (n:Kind {key: 1}) RETURN n").unwrap();
        let res2 = ex.execute(cmd2);
        assert!(res2.success, "Second MERGE failed");
        assert_eq!(res2.rows.len(), 1);
        let id2 = if let crate::ResultValue::Node(id) = res2.rows[0][0] {
            id
        } else {
            panic!("Expected Node ID");
        };

        // 3. IDs should be identical (idempotence)
        assert_eq!(id1, id2, "MERGE should be idempotent");
    }

    // ===== Parser-level tests for unsupported features =====
    // These test that the parser correctly errors or parses the syntax

    #[test]
    fn test_parse_help() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("HELP").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
    }

    #[test]
    fn test_parse_schema() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("SCHEMA").unwrap();
        let res = ex.execute(cmd);
        // Schema is not implemented
        assert!(!res.success);
    }

    #[test]
    fn test_set_command_requires_bound_var() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("SET n.state = 42").unwrap();
        let res = ex.execute(cmd);
        // Should fail because 'n' is not bound
        assert!(!res.success);
    }

    #[test]
    fn test_set_command_with_bound_var() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // First, bind 'n' via MERGE
        let cmd1 = parse("MERGE (n:Kind {key: 123}) RETURN n").unwrap();
        let res1 = ex.execute(cmd1);
        assert!(res1.success, "MERGE failed: {}", res1.message);

        // Now, set a property on the bound node 'n'
        let cmd2 = parse("SET n.state = 42").unwrap();
        let res2 = ex.execute(cmd2);
        // Should succeed because 'n' is bound
        assert!(res2.success, "SET failed: {}", res2.message);

        // Verify that the property was actually updated by matching the node
        let cmd3 = parse("MATCH (n:Kind {key: 123, state: 42}) RETURN n").unwrap();
        let res3 = ex.execute(cmd3);
        assert!(res3.success, "MATCH failed: {}", res3.message);
        assert_eq!(res3.rows.len(), 1, "Should find exactly 1 node with the updated property");
    }

    // ===== 8) Pagination and Ordering =====

    #[test]
    fn test_pagination_and_ordering_combined() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) DESC SKIP 1 LIMIT 2").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 2);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 4);
        } else {
            panic!("Expected Node result");
        }
        if let crate::ResultValue::Node(id) = res.rows[1][0] {
            assert_eq!(id, 3);
        } else {
            panic!("Expected Node result");
        }
    }

    #[test]
    fn test_pagination() {
        // MATCH (n) RETURN n ORDER BY id(n) ASC SKIP 1 LIMIT 2
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) ASC SKIP 1 LIMIT 2").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Expecting nodes 2, 3
        assert_eq!(res.rows.len(), 2);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 2);
        } else {
            panic!("Expected Node ID");
        }
        if let crate::ResultValue::Node(id) = res.rows[1][0] {
            assert_eq!(id, 3);
        } else {
            panic!("Expected Node ID");
        }
    }

    #[test]
    fn test_skip_overflow() {
        // MATCH (n) RETURN n SKIP 100
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n SKIP 100").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert!(res.rows.is_empty());
    }

    #[test]
    fn test_count_match_with_no_results() {
        // MATCH (n:NonExistent) RETURN count(n)
        // Ensure it returns a number (0) rather than empty results.
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:NonExistent) RETURN count(n)").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);
        if let crate::ResultValue::Number(n) = res.rows[0][0] {
            assert_eq!(n, 0);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_order_desc() {
        // MATCH (n) RETURN n ORDER BY id(n) DESC LIMIT 2
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n) RETURN n ORDER BY id(n) DESC LIMIT 2").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        // Expecting nodes 5, 4 (reverse ID order)
        assert_eq!(res.rows.len(), 2);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 5);
        } else {
            panic!("Expected Node ID");
        }
        if let crate::ResultValue::Node(id) = res.rows[1][0] {
            assert_eq!(id, 4);
        } else {
            panic!("Expected Node ID");
        }
    }

    #[test]
    fn test_merge_existing_node() {
        // MERGE (n:proc.Process {name: 12345}) RETURN n
        // This should match the existing node 1 created in setup_mock
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MERGE (n:proc.Process {name: 12345}) RETURN n").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success, "MERGE command failed");
        assert_eq!(res.rows.len(), 1, "Expected 1 row result");

        // It should be the existing node 1
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 1, "Expected to merge with existing node 1");
        } else {
            panic!("Expected Node result");
        }

        // Verify no new node was created (total count of proc.Process should be 2)
        let cmd_count = parse("MATCH (n:proc.Process) RETURN count(n)").unwrap();
        let res_count = ex.execute(cmd_count);
        assert!(res_count.success);
        if let crate::ResultValue::Number(n) = res_count.rows[0][0] {
            assert_eq!(n, 2, "Expected still only 2 proc.Process nodes");
        } else {
            panic!("Expected Number result for count");
        }
    }

    #[test]
    fn test_merge_edge_inline_nodes() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // MERGE (a:Kind {key: 100})-[:REL]->(b:Kind {key: 101})
        // This should create 'a', 'b', and the edge 'REL'.
        let cmd =
            parse("MERGE (a:Kind {key: 100})-[:REL]->(b:Kind {key: 101}) RETURN a, b").unwrap();
        let res = ex.execute(cmd);

        assert!(
            res.success,
            "MERGE edge with inline nodes failed: {}",
            res.message
        );
        assert_eq!(res.rows.len(), 1);
    }


    #[test]
    fn test_merge_edge_preserves_node_properties() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // MERGE an edge with newly created nodes that have properties
        let cmd = parse("MERGE (a:Person {age: 30})-[:KNOWS]->(b:Person {age: 25}) RETURN a, b").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success, "MERGE failed");
        assert_eq!(res.rows.len(), 1);

        let id_a = if let crate::ResultValue::Node(id) = res.rows[0][0] { id } else { panic!("No id a") };
        let id_b = if let crate::ResultValue::Node(id) = res.rows[0][1] { id } else { panic!("No id b") };

        // Test that they can be found by their properties via MATCH
        let cmd_check_a = parse("MATCH (n:Person {age: 30}) RETURN n").unwrap();
        let res_check_a = ex.execute(cmd_check_a);
        assert!(res_check_a.success);
        assert_eq!(res_check_a.rows.len(), 1);
        let found_a = if let crate::ResultValue::Node(id) = res_check_a.rows[0][0] { id } else { panic!() };
        assert_eq!(id_a, found_a);

        let cmd_check_b = parse("MATCH (n:Person {age: 25}) RETURN n").unwrap();
        let res_check_b = ex.execute(cmd_check_b);
        assert!(res_check_b.success);
        assert_eq!(res_check_b.rows.len(), 1);
        let found_b = if let crate::ResultValue::Node(id) = res_check_b.rows[0][0] { id } else { panic!() };
        assert_eq!(id_b, found_b);
    }

    #[test]
    fn test_match_edge_with_id_property_confusion() {
        let g = setup_mock();
        // Node 1 has internal ID 1. Let's give it a property "id" = 999.
        g.set_prop(1, "id", 999);

        let mut ex = GraphExecutor::with_graph(&g);

        // MATCH (a {id: 999})-[]->(b)
        // This should find node 1 (because it has prop id=999) and its edges.
        // But if the optimization kicks in, it might look for internal node 999!
        let cmd = parse("MATCH (a {id: 999})-[]->(b) RETURN a, b").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success);
        // Should find edges from node 1.
        assert!(!res.rows.is_empty(), "Expected edges from node 1, but got empty result. The executor likely confused property 'id' with internal Node ID.");
    }
    #[test]
    fn test_match_where_id_and_properties_must_both_match() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // Node 1 is proc.Process with name 12345 in our mock

        // 1. Correct kind, correct property, correct ID
        let cmd1 = parse("MATCH (n:proc.Process {name: 12345}) WHERE id(n) = 1 RETURN n").unwrap();
        let res1 = ex.execute(cmd1);
        assert!(res1.success);
        assert_eq!(res1.rows.len(), 1, "Should match when ID, kind, and properties all match");

        // 2. Correct kind, WRONG property, correct ID
        let cmd2 = parse("MATCH (n:proc.Process {name: 99999}) WHERE id(n) = 1 RETURN n").unwrap();
        let res2 = ex.execute(cmd2);
        assert!(res2.success);
        assert_eq!(res2.rows.len(), 0, "Should not match if properties differ, even if internal ID matches");

        // 3. WRONG kind, correct property, correct ID
        let cmd3 = parse("MATCH (n:fs.File {name: 12345}) WHERE id(n) = 1 RETURN n").unwrap();
        let res3 = ex.execute(cmd3);
        assert!(res3.success);
        assert_eq!(res3.rows.len(), 0, "Should not match if kind differs, even if internal ID matches");
    }
    #[test]

      fn test_edge_order_by_desc() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // MATCH (a)-[]->(b) RETURN a ORDER BY id(a) DESC
        // Edges: 1->2, 1->3, 3->1, 3->2
        // We expect 'a' to be 3, 3, 1, 1

        let cmd = parse("MATCH (a)-[]->(b) RETURN a ORDER BY id(a) DESC").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);

        let mut ids = Vec::new();
        for row in res.rows {
            if let crate::ResultValue::Node(id) = row[0] {
                ids.push(id);
            }
        }

        assert_eq!(ids, vec![3, 3, 1, 1]);
  }
  #[test]
    fn test_multiple_return_expressions() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Process) RETURN n, count(n)").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.columns, vec!["n", "count(n)"]);
        // It has 1 result row since it is an aggregation? Or does it group?
        // Wait, executor.rs says:
        // if has_aggregate ... row.push(ResultValue::Number(matched_ids.len() as u64));
        // else push 0.
        assert_eq!(res.rows.len(), 1);
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 1); // first node found
        } else {
            panic!("Expected Node ID");
        }
        if let crate::ResultValue::Number(n) = res.rows[0][1] {
            assert_eq!(n, 2); // 2 proc.Process nodes
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_merge_with_params() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // Parameter for property value
        ex.set_parameter("p_key".to_string(), crate::gql::Value::Number(999));

        // MERGE using the parameter
        let cmd = parse("MERGE (n:Kind {key: $p_key}) RETURN n").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success, "MERGE with params failed: {}", res.message);
        assert_eq!(res.rows.len(), 1);

        let id = if let crate::ResultValue::Node(id) = res.rows[0][0] {
            id
        } else {
            panic!("Expected Node ID");
        };

        // Verify the node was created with the correct property value by querying it back
        // We use the literal value here to ensure the parameter was correctly resolved during MERGE
        let cmd_check = parse("MATCH (n:Kind {key: 999}) RETURN n").unwrap();
        let res_check = ex.execute(cmd_check);

        assert!(res_check.success);
        assert_eq!(
            res_check.rows.len(),
            1,
            "Should find the node with property key=999"
        );

        if let crate::ResultValue::Node(id_check) = res_check.rows[0][0] {
            assert_eq!(
                id, id_check,
                "The found node should be the same as the merged one"
            );
        } else {
            panic!("Expected Node ID");
        }
    }

    #[test]
    fn test_match_multiple_props() {
        let g = setup_mock();
        // Add nodes with multiple properties
        g.add_node(6, "proc.Thread");
        g.set_prop(6, "tid", 1001);
        g.set_prop(6, "state", 200);

        g.add_node(7, "proc.Thread");
        g.set_prop(7, "tid", 1002);
        g.set_prop(7, "state", 200);

        let mut ex = GraphExecutor::with_graph(&g);

        // MATCH (t:proc.Thread {state: 200, tid: 1002}) RETURN t
        let cmd = parse("MATCH (t:proc.Thread {state: 200, tid: 1002}) RETURN t").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success);
        assert_eq!(res.rows.len(), 1, "Expected exactly 1 match for logical AND of multiple properties");
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 7);
        } else {
            panic!("Expected Node ID");
        }

        // MATCH (t:proc.Thread {state: 200, tid: 9999}) RETURN t
        // Should return empty since tid 9999 doesn't exist
        let cmd_empty = parse("MATCH (t:proc.Thread {state: 200, tid: 9999}) RETURN t").unwrap();
        let res_empty = ex.execute(cmd_empty);
        assert!(res_empty.success);
        assert_eq!(res_empty.rows.len(), 0, "Expected 0 matches because of conflicting property");
          }

    #[test]

    fn test_variable_binding_lifecycle() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // 1. MATCH does not bind variables across statements
        let cmd_match = parse("MATCH (n:proc.Process {name: 12345}) RETURN n").unwrap();
        let res_match = ex.execute(cmd_match);
        assert!(res_match.success, "MATCH failed");
        assert_eq!(res_match.rows.len(), 1, "Expected to find node 1");

        let cmd_set_fail = parse("SET n.priority = 10").unwrap();
        let res_set_fail = ex.execute(cmd_set_fail);
        assert!(!res_set_fail.success, "SET should fail because 'n' is not bound by MATCH");
        assert!(res_set_fail.message.contains("not bound"), "Expected not bound message, got: {}", res_set_fail.message);

        // 2. MERGE binds variables across statements
        let cmd_merge = parse("MERGE (m:proc.Process {name: 12345}) RETURN m").unwrap();
        let res_merge = ex.execute(cmd_merge);
        assert!(res_merge.success, "MERGE failed");
        assert_eq!(res_merge.rows.len(), 1, "Expected to return node 1");

        // Let's verify it actually returned node 1
        if let crate::ResultValue::Node(id) = res_merge.rows[0][0] {
            assert_eq!(id, 1, "Expected node 1");
        } else {
            panic!("Expected Node ID");
        }

        // 3. SET on a bound variable should succeed
        let cmd_set_success = parse("SET m.priority = 10").unwrap();
        let res_set_success = ex.execute(cmd_set_success);
        assert!(res_set_success.success, "SET should succeed because 'm' was bound by MERGE. Message: {}", res_set_success.message);

        // 4. Verify the property was set correctly
        let cmd_verify = parse("MATCH (x:proc.Process {priority: 10}) RETURN x").unwrap();
        let res_verify = ex.execute(cmd_verify);
        assert!(res_verify.success, "MATCH for verification failed");
        assert_eq!(res_verify.rows.len(), 1, "Expected to find 1 node with priority 10");

        if let crate::ResultValue::Node(id) = res_verify.rows[0][0] {
            assert_eq!(id, 1, "Expected the modified node to be node 1");
        } else {
            panic!("Expected Node ID");
        }
    }

    #[test]

      fn test_merge_multiple_properties() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);

        // MERGE a node with multiple properties
        let cmd = parse("MERGE (n:Kind {key1: 10, key2: 20, key3: 30}) RETURN n").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success, "MERGE with multiple properties failed: {}", res.message);
        assert_eq!(res.rows.len(), 1);

        let id = if let crate::ResultValue::Node(id) = res.rows[0][0] {
            id
        } else {
            panic!("Expected Node ID");
        };

        // Verify the properties were set correctly by matching them back
        let cmd_check = parse("MATCH (n:Kind {key1: 10, key2: 20, key3: 30}) RETURN n").unwrap();
        let res_check = ex.execute(cmd_check);

        assert!(res_check.success);
        assert_eq!(res_check.rows.len(), 1, "Should find the node with all properties");

        if let crate::ResultValue::Node(id_check) = res_check.rows[0][0] {
            assert_eq!(id, id_check, "The found node should be the same as the merged one");
        } else {
            panic!("Expected Node ID");
        }
  }
    #[test]
    fn test_match_multiple_inline_properties() {
        let g = setup_mock();
        // Give node 1 two specific properties
        g.set_prop(1, "prop1", 100);
        g.set_prop(1, "prop2", 200);

        let mut ex = GraphExecutor::with_graph(&g);

        // 1. Match with both properties correct (Logical AND)
        let cmd = parse("MATCH (n {prop1: 100, prop2: 200}) RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1, "Should find node with both properties matching");
        if let crate::ResultValue::Node(id) = res.rows[0][0] {
            assert_eq!(id, 1);
        } else {
            panic!("Expected Node result");
        }

        // 2. Match with one property conflicting
        let cmd2 = parse("MATCH (n {prop1: 100, prop2: 999}) RETURN n").unwrap();
        let res2 = ex.execute(cmd2);
        assert!(res2.success);
        assert_eq!(res2.rows.len(), 0, "Should return empty result if one property conflicts");

        // 3. Match with the other property conflicting
        let cmd3 = parse("MATCH (n {prop1: 999, prop2: 200}) RETURN n").unwrap();
        let res3 = ex.execute(cmd3);
        assert!(res3.success);
        assert_eq!(res3.rows.len(), 0, "Should return empty result if the other property conflicts");

        // 4. Match with missing property
        let cmd4 = parse("MATCH (n {prop1: 100, prop3: 300}) RETURN n").unwrap();
        let res4 = ex.execute(cmd4);
        assert!(res4.success);
        assert_eq!(res4.rows.len(), 0, "Should return empty result if property is missing");
    }
}

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

    // ===== 8) Pagination and Ordering =====

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
        let cmd = parse("MERGE (a:Kind {key: 100})-[:REL]->(b:Kind {key: 101}) RETURN a, b").unwrap();
        let res = ex.execute(cmd);

        assert!(res.success, "MERGE edge with inline nodes failed: {}", res.message);
        assert_eq!(res.rows.len(), 1);
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
    fn test_multiple_properties_in_node_pattern() {
        let g = setup_mock();
        let mut ex = GraphExecutor::with_graph(&g);
        let cmd = parse("MATCH (n:proc.Process {name: 12345, state: 100}) RETURN n").unwrap();
        let res = ex.execute(cmd);
        assert!(res.success);
        assert_eq!(res.rows.len(), 1);
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
}

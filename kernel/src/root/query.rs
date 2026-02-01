use crate::root::graph::Graph;
use abi::query::QueryRow;
use abi::symbols::SymbolId;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub struct PreparedStep {
    pub op: u64,
    pub arg1: u64,
    pub symbol: SymbolId,
}

pub fn execute(graph: &Graph, plan: &[PreparedStep], out: &mut [QueryRow]) -> Result<usize, ()> {
    if plan.is_empty() {
        return Ok(0);
    }

    let mut current_rows: Vec<QueryRow> = Vec::new();

    // Step 0: Source (Scan or Start)
    let step0 = &plan[0];
    
    if step0.op == 1 {
        // Scan: initialize with all nodes of a given kind
        let kind_sym = step0.symbol;
        let limit = if step0.arg1 == 0 {
            64
        } else {
            step0.arg1 as usize
        };

        if let Some(ids) = graph.kind_index.get(&kind_sym) {
            for id in ids.iter().take(limit) {
                current_rows.push(QueryRow {
                    id: *id,
                    kind_rel: kind_sym as u64,
                    val_dst: 0,
                    extra: 0,
                });
            }
        }
    } else if step0.op == 4 {
        // Start: initialize with a specific node ID
        let node_id = step0.arg1;
        if graph.nodes.contains_key(&node_id) {
            let kind = graph.nodes.get(&node_id).map(|n| n.kind).unwrap_or(0);
            current_rows.push(QueryRow {
                id: node_id,
                kind_rel: kind as u64,
                val_dst: 0,
                extra: 0,
            });
        }
    } else {
        return Err(()); // Must be Scan or Start
    }

    // Pipeline
    for step in &plan[1..] {
        let mut next_rows = Vec::new();
        match step.op {
            2 => {
                // FilterEq
                let key = step.symbol;
                let val = step.arg1;
                for row in current_rows {
                    if let Some(node) = graph.nodes.get(&row.id) {
                        if let Some(&prop_val) = node.props.get(&key) {
                            if prop_val == val {
                                next_rows.push(row);
                            }
                        }
                    }
                }
                current_rows = next_rows;
            }
            3 => {
                // Expand
                let dir = step.arg1;
                let rel = step.symbol;
                // Increase arbitrary expansion limit to prevent explosions but allow more data
                let limit = 16384;
                let mut count = 0;

                for row in current_rows {
                    if count >= limit {
                        break;
                    }
                    if let Some(node) = graph.nodes.get(&row.id) {
                        if dir == 0 {
                            // Out
                            for (r, dst) in &node.edges {
                                if rel == 0 || *r == rel {
                                    next_rows.push(QueryRow {
                                        id: row.id,
                                        kind_rel: *r as u64,
                                        val_dst: *dst,
                                        extra: 0,
                                    });
                                    count += 1;
                                }
                            }
                        } else {
                            // In
                            // Fast lookup using reverse index
                            if let Some(incoming) = graph.reverse_index.get(&row.id) {
                                for (r, src) in incoming {
                                    if rel == 0 || *r == rel {
                                        next_rows.push(QueryRow {
                                            id: *src,
                                            kind_rel: *r as u64,
                                            val_dst: row.id,
                                            extra: 0,
                                        });
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                current_rows = next_rows;
            }
            _ => return Err(()),
        }
    }

    let count = core::cmp::min(current_rows.len(), out.len());
    for i in 0..count {
        out[i] = current_rows[i];
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::root::graph::Graph;

    #[test]
    fn test_query_execution_scenarios() {
        let mut graph = Graph::new();

        // Define symbols
        let kind_person = 100;
        let kind_post = 101;
        let prop_age = 200;
        let rel_authored = 300;
        let rel_liked = 301;

        // Create Data
        // Person 1: Age 25
        let p1 = graph.alloc(kind_person);
        if let Some(node) = graph.get_node_mut(p1) {
            node.props.insert(prop_age, 25);
        }

        // Person 2: Age 30
        let p2 = graph.alloc(kind_person);
        if let Some(node) = graph.get_node_mut(p2) {
            node.props.insert(prop_age, 30);
        }

        // Post 1
        let post1 = graph.alloc(kind_post);

        // Post 2
        let post2 = graph.alloc(kind_post);

        // Links
        // P2 authored Post1
        graph.link(p2, rel_authored, post1);
        // P1 liked Post1
        graph.link(p1, rel_liked, post1);
        // P2 liked Post2
        graph.link(p2, rel_liked, post2);

        // Scenario 1: Scan(Person) -> Filter(Age=30) -> Expand(Authored, Out)
        // Should find P2 -> Post1
        let plan1 = vec![
            PreparedStep { op: 1, symbol: kind_person, arg1: 0 },
            PreparedStep { op: 2, symbol: prop_age, arg1: 30 },
            PreparedStep { op: 3, symbol: rel_authored, arg1: 0 },
        ];

        let mut out = [QueryRow::default(); 10];
        let count = execute(&graph, &plan1, &mut out).expect("Plan 1 failed");

        assert_eq!(count, 1);
        assert_eq!(out[0].id, p2);
        assert_eq!(out[0].val_dst, post1);

        // Scenario 2: Scan(Post) -> Expand(Authored, In)
        // Should find Post1 -> P2
        let plan2 = vec![
            PreparedStep { op: 1, symbol: kind_post, arg1: 0 },
            PreparedStep { op: 3, symbol: rel_authored, arg1: 1 }, // In
        ];

        let count = execute(&graph, &plan2, &mut out).expect("Plan 2 failed");

        // Post1 is authored by P2. Post2 is authored by nobody.
        // Scan returns Post1, Post2.
        // Expand In for Post1 finds P2.
        // Expand In for Post2 finds nothing.
        // Result: 1 row.

        assert_eq!(count, 1);
        assert_eq!(out[0].id, p2); // Because Expand In sets id to the source node found
        assert_eq!(out[0].val_dst, post1); // And val_dst to the matched node (the one we expanded from)

        // Scenario 3: Scan(Person) -> Expand(Liked, Out)
        // P1 -> Post1, P2 -> Post2.
        // Should return 2 rows.
        let plan3 = vec![
            PreparedStep { op: 1, symbol: kind_person, arg1: 0 },
            PreparedStep { op: 3, symbol: rel_liked, arg1: 0 },
        ];

        let count = execute(&graph, &plan3, &mut out).expect("Plan 3 failed");
        assert_eq!(count, 2);

        // Order depends on Scan order (kind_index order) and Expand order.
        let r1 = &out[0];
        let r2 = &out[1];

        // Verify we found both relations
        let found_p1 = (r1.id == p1 && r1.val_dst == post1) || (r2.id == p1 && r2.val_dst == post1);
        let found_p2 = (r1.id == p2 && r1.val_dst == post2) || (r2.id == p2 && r2.val_dst == post2);

        assert!(found_p1, "Did not find P1 liked Post1");
        assert!(found_p2, "Did not find P2 liked Post2");

        // Scenario 4: Scan(Person) -> Expand(Wildcard, Out)
        // P2 has Authored(Post1) and Liked(Post2).
        // P1 has Liked(Post1).
        // Should find 3 edges.
        let plan4 = vec![
            PreparedStep {
                op: 1,
                symbol: kind_person,
                arg1: 0,
            },
            PreparedStep {
                op: 3,
                symbol: 0,
                arg1: 0,
            },
        ];
        let count = execute(&graph, &plan4, &mut out).expect("Plan 4 failed");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_query_reverse_index_optimization() {
        let mut graph = Graph::new();
        let kind_a = 1;
        let kind_b = 2;
        let rel_x = 10;

        let a1 = graph.alloc(kind_a);
        let a2 = graph.alloc(kind_a);
        let b1 = graph.alloc(kind_b);

        // a1 -> x -> b1
        // a2 -> x -> b1
        graph.link(a1, rel_x, b1);
        graph.link(a2, rel_x, b1);

        // Plan: Scan(B) -> Expand(X, In)
        // Should find a1, a2
        let plan = vec![
            PreparedStep {
                op: 1,
                symbol: kind_b,
                arg1: 0,
            },
            PreparedStep {
                op: 3,
                symbol: rel_x,
                arg1: 1,
            }, // In
        ];

        let mut out = [QueryRow::default(); 10];
        let count = execute(&graph, &plan, &mut out).expect("Plan failed");

        assert_eq!(count, 2);

        let r0 = &out[0];
        let r1 = &out[1];

        let ids = vec![r0.id, r1.id];
        assert!(ids.contains(&a1));
        assert!(ids.contains(&a2));
        assert_eq!(r0.val_dst, b1);
        assert_eq!(r1.val_dst, b1);
    }

    #[test]
    fn test_query_start_op_and_edge_cases() {
        let mut graph = Graph::new();
        let kind_a = 1;
        let kind_empty = 2;
        let prop_x = 10;

        let n1 = graph.alloc(kind_a);
        if let Some(node) = graph.get_node_mut(n1) {
            node.props.insert(prop_x, 100);
        }

        let _n2 = graph.alloc(kind_a);
        // n2 has no props

        let mut out = [QueryRow::default(); 10];

        // 1. Test Start(Op 4) with valid node
        let plan_start_valid = vec![PreparedStep {
            op: 4,
            symbol: 0,
            arg1: n1,
        }];
        let count = execute(&graph, &plan_start_valid, &mut out).expect("Start valid failed");
        assert_eq!(count, 1);
        assert_eq!(out[0].id, n1);
        assert_eq!(out[0].kind_rel, kind_a as u64);

        // 2. Test Start(Op 4) with invalid node
        let plan_start_invalid = vec![PreparedStep {
            op: 4,
            symbol: 0,
            arg1: 999999,
        }];
        let count = execute(&graph, &plan_start_invalid, &mut out).expect("Start invalid failed");
        assert_eq!(count, 0);

        // 3. Test Scan(Op 1) with empty kind
        let plan_scan_empty = vec![PreparedStep {
            op: 1,
            symbol: kind_empty,
            arg1: 0,
        }];
        let count = execute(&graph, &plan_scan_empty, &mut out).expect("Scan empty failed");
        assert_eq!(count, 0);

        // 4. Test Scan + FilterEq where property is missing on some nodes
        // n1 has prop_x=100, n2 has no prop_x
        // Filter for prop_x=100 -> should find n1
        let plan_filter_hit = vec![
            PreparedStep {
                op: 1,
                symbol: kind_a,
                arg1: 0,
            },
            PreparedStep {
                op: 2,
                symbol: prop_x,
                arg1: 100,
            },
        ];
        let count = execute(&graph, &plan_filter_hit, &mut out).expect("Filter hit failed");
        assert_eq!(count, 1);
        assert_eq!(out[0].id, n1);

        // Filter for prop_x=200 -> should find nothing
        let plan_filter_miss_val = vec![
            PreparedStep {
                op: 1,
                symbol: kind_a,
                arg1: 0,
            },
            PreparedStep {
                op: 2,
                symbol: prop_x,
                arg1: 200,
            },
        ];
        let count = execute(&graph, &plan_filter_miss_val, &mut out).expect("Filter miss val failed");
        assert_eq!(count, 0);
    }
}

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

    // Step 0: Source (Scan)
    let step0 = &plan[0];
    if step0.op != 1 {
        return Err(());
    } // Must be Scan

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
                // Arbitrary expansion limit to prevent explosions
                let limit = 256;
                let mut count = 0;

                for row in current_rows {
                    if count >= limit {
                        break;
                    }
                    if let Some(node) = graph.nodes.get(&row.id) {
                        if dir == 0 {
                            // Out
                            for (r, dst) in &node.edges {
                                if *r == rel {
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
                            // Slow scan for incoming
                            for (nid, n) in &graph.nodes {
                                for (r, dst) in &n.edges {
                                    if *dst == row.id && *r == rel {
                                        next_rows.push(QueryRow {
                                            id: *nid,
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
    }
}

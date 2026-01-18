use crate::root::graph::Graph;
use abi::query::QueryRow;
use abi::symbols::SymbolId;
use abi::wire::ThingId;
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
                kind_rel: kind_sym,
                val_dst: ThingId::default(), // 0?
                extra: 0,
            });
        }
    }

    // Pipeline
    for step in &plan[1..] {
        let mut next_rows = Vec::new();
        match step.op {
            2 => {
                // FilterEq (Property Equality)
                // val is u64 in PreparedStep::arg1.
                // But props are [u8; 16].
                // We need to match. If we assume u64 props are stored as zero-padded bytes...
                let key = step.symbol;
                let val_u64 = step.arg1;
                let mut val_bytes = [0u8; 16];
                val_bytes[0..8].copy_from_slice(&val_u64.to_le_bytes());

                for row in current_rows {
                    if let Some(node) = graph.nodes.get(&row.id) {
                        if let Some(prop_val) = node.props.get(&key) {
                            if *prop_val == val_bytes {
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
                                        kind_rel: *r,
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
                                            kind_rel: *r,
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

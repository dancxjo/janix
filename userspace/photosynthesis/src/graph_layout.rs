use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::cmp::Ordering;
use stem::thing::ThingId;

#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub id: ThingId,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub fixed: bool,
    pub rank: i32,
}

#[derive(Clone, Debug)]
pub struct LayoutEdge {
    pub from: ThingId,
    pub to: ThingId,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct LayoutSettings {
    pub rank_spacing: f32,
    pub node_spacing: f32,
    pub rank_padding: f32,
    pub damping: f32,
    pub max_iterations: usize,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            rank_spacing: 80.0,
            node_spacing: 100.0,
            rank_padding: 20.0,
            damping: 0.5,
            max_iterations: 2,
        }
    }
}

// Internal structures for graph traversal
struct InternalNode {
    neighbors_in: Vec<usize>,
    neighbors_out: Vec<usize>,
    connection_count: usize,
}

pub fn compute_layout(nodes: &mut [LayoutNode], edges: &[LayoutEdge], settings: &LayoutSettings) {
    if nodes.is_empty() {
        return;
    }

    // 1. Build index map
    let mut id_map = BTreeMap::new();
    let mut internal_nodes = Vec::with_capacity(nodes.len());

    for (i, node) in nodes.iter().enumerate() {
        id_map.insert(node.id, i);
        internal_nodes.push(InternalNode {
            neighbors_in: Vec::new(),
            neighbors_out: Vec::new(),
            connection_count: 0,
        });
    }

    // 2. Build graph topology
    for edge in edges {
        if let (Some(&src_idx), Some(&tgt_idx)) = (id_map.get(&edge.from), id_map.get(&edge.to)) {
            internal_nodes[src_idx].neighbors_out.push(tgt_idx);
            internal_nodes[src_idx].connection_count += 1;

            internal_nodes[tgt_idx].neighbors_in.push(src_idx);
            internal_nodes[tgt_idx].connection_count += 1;
        }
    }

    // 3. Assign Ranks (Phase 1)
    assign_ranks(nodes, &internal_nodes);

    // 4. Initial Placement
    initial_placement(nodes, &internal_nodes, settings);

    // 5. Collision Resolution (Phase 2)
    resolve_collisions(nodes, settings);

    // 6. Refinement (Phase 3)
    for _ in 0..settings.max_iterations {
        refine_edges(nodes, &internal_nodes, settings);
        resolve_collisions(nodes, settings);
    }
}

fn assign_ranks(nodes: &mut [LayoutNode], internal: &[InternalNode]) {
    // Collect roots
    let mut queue = VecDeque::new();

    // Reset ranks for non-fixed nodes
    for node in nodes.iter_mut() {
        if !node.fixed {
            node.rank = -1;
        }
    }

    // Initialize queue with sources (in-degree 0) or explicit roots
    for (i, internal_node) in internal.iter().enumerate() {
        if internal_node.neighbors_in.is_empty() {
            if nodes[i].rank == -1 {
                nodes[i].rank = 0;
            }
            queue.push_back(i);
        } else if nodes[i].rank == 0 {
            // Forced root
            queue.push_back(i);
        }
    }

    // If queue is empty (cyclic graph with no roots), pick the first node
    if queue.is_empty() && !nodes.is_empty() {
        if nodes[0].rank == -1 {
            nodes[0].rank = 0;
        }
        queue.push_back(0);
    }

    // Longest Path Layering
    let max_passes = nodes.len() * 2;
    let mut passes = 0;

    let mut changed = true;
    while changed && passes < max_passes {
        changed = false;
        passes += 1;

        for i in 0..nodes.len() {
            let u_rank = nodes[i].rank;
            if u_rank == -1 {
                continue;
            }

            for &v_idx in &internal[i].neighbors_out {
                if nodes[v_idx].fixed {
                    continue;
                }

                if nodes[v_idx].rank < u_rank + 1 {
                    nodes[v_idx].rank = u_rank + 1;
                    changed = true;
                }
            }
        }
    }

    // Ensure everyone has a rank
    for node in nodes.iter_mut() {
        if node.rank == -1 {
            node.rank = 0;
        }
    }
}

fn initial_placement(
    nodes: &mut [LayoutNode],
    internal: &[InternalNode],
    settings: &LayoutSettings,
) {
    // Group by rank
    let mut layers: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        layers.entry(node.rank).or_default().push(i);
    }

    for (rank, indices) in &mut layers {
        let rank_y = (*rank as f32) * settings.rank_spacing + 50.0;

        // Sort indices based on connection count / stability
        indices.sort_by(|&a, &b| {
            let nc_a = internal[a].connection_count;
            let nc_b = internal[b].connection_count;
            let res = nc_b.cmp(&nc_a); // descending
            if res != Ordering::Equal {
                return res;
            }
            nodes[a].id.cmp(&nodes[b].id) // stable tie-breaker
        });

        let mut current_x = 50.0;
        for &idx in indices.iter() {
            let node = &mut nodes[idx];
            if !node.fixed {
                node.y = rank_y;
                node.x = current_x;
                current_x += node.w + settings.node_spacing;
            } else {
                if node.x + node.w > current_x {
                    current_x = node.x + node.w + settings.node_spacing;
                }
            }
        }
    }
}

fn resolve_collisions(nodes: &mut [LayoutNode], settings: &LayoutSettings) {
    let mut layers: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        layers.entry(node.rank).or_default().push(i);
    }

    for (_, indices) in &mut layers {
        // Sort by X
        indices.sort_by(|&a, &b| {
            nodes[a]
                .x
                .partial_cmp(&nodes[b].x)
                .unwrap_or(Ordering::Equal)
        });

        // Sweep left-to-right
        for i in 1..indices.len() {
            let left_idx = indices[i - 1];
            let curr_idx = indices[i];

            let left_right_edge = nodes[left_idx].x + nodes[left_idx].w + settings.rank_padding;

            if nodes[curr_idx].x < left_right_edge {
                if !nodes[curr_idx].fixed {
                    nodes[curr_idx].x = left_right_edge;
                }
            }
        }
    }
}

fn refine_edges(nodes: &mut [LayoutNode], internal: &[InternalNode], settings: &LayoutSettings) {
    let max_move = 50.0;

    let mut moves = Vec::with_capacity(nodes.len());

    for (i, node) in nodes.iter().enumerate() {
        if node.fixed {
            moves.push(0.0);
            continue;
        }

        let mut sum_x = 0.0;
        let mut count = 0;

        // Incoming neighbors
        for &n_in in &internal[i].neighbors_in {
            sum_x += nodes[n_in].x + (nodes[n_in].w / 2.0);
            count += 1;
        }

        // Outgoing neighbors
        for &n_out in &internal[i].neighbors_out {
            sum_x += nodes[n_out].x + (nodes[n_out].w / 2.0);
            count += 1;
        }

        if count > 0 {
            let target_center = sum_x / (count as f32);
            let current_center = node.x + (node.w / 2.0);
            let delta = target_center - current_center;

            let move_amt = delta * settings.damping;
            moves.push(move_amt.clamp(-max_move, max_move));
        } else {
            moves.push(0.0);
        }
    }

    // Apply moves
    for (i, move_amt) in moves.iter().enumerate() {
        if *move_amt != 0.0 {
            nodes[i].x += *move_amt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::types::HandleId;

    #[test]
    fn test_collision_resolution() {
        let mut nodes = vec![
            LayoutNode {
                id: ThingId::from_u64(1),
                x: 100.0,
                y: 100.0,
                w: 100.0,
                h: 30.0,
                fixed: false,
                rank: 0,
            },
            LayoutNode {
                id: ThingId::from_u64(2),
                x: 110.0,
                y: 100.0,
                w: 100.0,
                h: 30.0,
                fixed: false,
                rank: 0,
            },
        ];
        let settings = LayoutSettings::default();
        resolve_collisions(&mut nodes, &settings);

        // Expected behavior: node 2 pushed to right of node 1 + padding
        let n1_right = nodes[0].x + nodes[0].w;
        // With x=100, w=100, right=200.
        // n2 starts at 110.
        // Should be pushed to 200 + 20 = 220.

        assert!(
            nodes[1].x >= 220.0,
            "Node 2 was not pushed enough, x={}",
            nodes[1].x
        );
    }
}

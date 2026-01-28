use alloc::{vec, vec::Vec};
use alloc::collections::BTreeMap;
use stem::thing::ThingId;

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutNode {
    pub id: ThingId,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub fixed: bool,
    pub rank: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutEdge {
    pub from: ThingId,
    pub to: ThingId,
    pub weight: f32,
}

pub struct LayoutSettings {
    pub rank_spacing: f32,
    pub node_spacing: f32,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            rank_spacing: 120.0,
            node_spacing: 150.0,
        }
    }
}

pub fn compute_layout(
    nodes: &mut [LayoutNode],
    edges: &[LayoutEdge],
    settings: &LayoutSettings,
) {
    if nodes.is_empty() {
        return;
    }

    // Phase 1: Ranking (if not already set)
    assign_ranks(nodes, edges);

    // Phase 2: Initial Placement
    initial_placement(nodes, settings);

    // Phase 3: Collision Resolution
    resolve_collisions(nodes, settings);

    // Phase 4: Relaxation (Nudging)
    relax_edges(nodes, edges, 2);
    
    // Final Collision Check after relaxation
    resolve_collisions(nodes, settings);
}

fn assign_ranks(nodes: &mut [LayoutNode], edges: &[LayoutEdge]) {
    // Basic BFS-based ranking
    let mut adjacency = BTreeMap::new();
    for edge in edges {
        adjacency.entry(edge.from).or_insert_with(Vec::new).push(edge.to);
    }

    let mut ranks = BTreeMap::new();
    let mut queue = Vec::new();

    // Find roots (nodes with no incoming edges)
    let mut incoming_counts = BTreeMap::new();
    for node in nodes.iter() {
        incoming_counts.insert(node.id, 0);
    }
    for edge in edges {
        *incoming_counts.entry(edge.to).or_insert(0) += 1;
    }

    for node in nodes.iter() {
        if incoming_counts.get(&node.id).cloned().unwrap_or(0) == 0 {
            queue.push((node.id, 0));
        }
    }

    // If no roots (cycle?), pick the first node
    if queue.is_empty() && !nodes.is_empty() {
        queue.push((nodes[0].id, 0));
    }

    let mut head = 0;
    while head < queue.len() {
        let (id, rank) = queue[head];
        head += 1;

        let current_rank = ranks.entry(id).or_insert(rank);
        if rank > *current_rank {
            *current_rank = rank;
        }

        if let Some(neighbors) = adjacency.get(&id) {
            for &neighbor in neighbors {
                // To prevent infinite loops in cycles, we limit depth or check seen
                if rank < 20 {
                    queue.push((neighbor, rank + 1));
                }
            }
        }
    }

    for node in nodes.iter_mut() {
        if let Some(&rank) = ranks.get(&node.id) {
            node.rank = rank;
        }
    }
}

fn initial_placement(nodes: &mut [LayoutNode], settings: &LayoutSettings) {
    // Group by rank
    let mut rank_groups: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        rank_groups.entry(node.rank).or_insert_with(Vec::new).push(i);
    }

    for (&rank, indices) in rank_groups.iter() {
        // Sort indices by stable ID hash for determinism
        let mut sorted_indices = indices.clone();
        sorted_indices.sort_by_key(|&i| nodes[i].id.to_u64_lossy());

        for (idx_in_rank, &node_idx) in sorted_indices.iter().enumerate() {
            let node = &mut nodes[node_idx];
            if !node.fixed {
                node.y = (rank as f32) * settings.rank_spacing + 60.0;
                node.x = (idx_in_rank as f32) * settings.node_spacing + 60.0;
            }
        }
    }
}

fn resolve_collisions(nodes: &mut [LayoutNode], settings: &LayoutSettings) {
    // Group by rank for horizontal sweep
    let mut rank_groups: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        rank_groups.entry(node.rank).or_insert_with(Vec::new).push(i);
    }

    for indices in rank_groups.values() {
        let mut sorted_indices = indices.clone();
        sorted_indices.sort_by(|&a, &b| nodes[a].x.partial_cmp(&nodes[b].x).unwrap());

        for i in 1..sorted_indices.len() {
            let prev_idx = sorted_indices[i-1];
            let curr_idx = sorted_indices[i];
            
            let prev_right = nodes[prev_idx].x + nodes[prev_idx].w / 2.0;
            let curr_left = nodes[curr_idx].x - nodes[curr_idx].w / 2.0;
            
            let min_gap = 20.0;
            if curr_left < prev_right + min_gap {
                if !nodes[curr_idx].fixed {
                    nodes[curr_idx].x = prev_right + min_gap + nodes[curr_idx].w / 2.0;
                }
            }
        }
    }
}

fn relax_edges(nodes: &mut [LayoutNode], edges: &[LayoutEdge], passes: usize) {
    for _ in 0..passes {
        let mut shifts = BTreeMap::new();
        
        for edge in edges {
            let from_pos = nodes.iter().find(|n| n.id == edge.from).map(|n| n.x);
            let to_pos = nodes.iter().find(|n| n.id == edge.to).map(|n| n.x);
            
            if let (Some(fx), Some(tx)) = (from_pos, to_pos) {
                // Pull toward each other
                let target = (fx + tx) / 2.0;
                let pull_strength = 0.1;
                
                let from_shift = (target - fx) * pull_strength;
                let to_shift = (target - tx) * pull_strength;
                
                *shifts.entry(edge.from).or_insert(0.0) += from_shift;
                *shifts.entry(edge.to).or_insert(0.0) += to_shift;
            }
        }
        
        for node in nodes.iter_mut() {
            if !node.fixed {
                if let Some(&shift) = shifts.get(&node.id) {
                    // Clamp shift to preserve stability
                    let max_delta = 20.0;
                    node.x += shift.clamp(-max_delta, max_delta);
                }
            }
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
            LayoutNode { id: ThingId::from_u64(1), x: 100.0, y: 100.0, w: 100.0, h: 30.0, fixed: false, rank: 0 },
            LayoutNode { id: ThingId::from_u64(2), x: 110.0, y: 100.0, w: 100.0, h: 30.0, fixed: false, rank: 0 },
        ];
        let settings = LayoutSettings::default();
        resolve_collisions(&mut nodes, &settings);
        
        let n1_right = nodes[0].x + nodes[0].w / 2.0;
        let n2_left = nodes[1].x - nodes[1].w / 2.0;
        assert!(n2_left >= n1_right + 19.9);
    }
}

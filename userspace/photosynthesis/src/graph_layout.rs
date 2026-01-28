use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
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
    pub grid_w: f32,
    pub grid_h: f32,
    pub rank_separation: f32, // Vertical distance between ranks (grid units usually)
    pub node_separation: f32, // Horizontal distance between nodes
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            grid_w: 160.0, // Cell width
            grid_h: 120.0, // Cell height
            rank_separation: 1.0, 
            node_separation: 1.0,
        }
    }
}



// Helper to establish a stable ordering for the spiral
fn bfs_ordering(nodes: &[LayoutNode], edges: &[LayoutEdge]) -> Vec<usize> {
    let mut adj: BTreeMap<ThingId, Vec<ThingId>> = BTreeMap::new();
    let mut id_to_idx = BTreeMap::new();
    
    for (i, node) in nodes.iter().enumerate() {
        id_to_idx.insert(node.id, i);
        adj.entry(node.id).or_default();
    }

    for edge in edges {
        adj.entry(edge.from).or_default().push(edge.to);
        adj.entry(edge.to).or_default().push(edge.from); // Undirected for proximity
    }

    // Find root: standard approach is 0-indegree, but for a general "center" of a cluster,
    // we might just pick the node with the most connections?
    // Or just picking index 0 as fallback. 
    // The prompt says "center the root element".
    // Let's look for a node with no incoming edges (true root).
    let mut in_degrees = BTreeMap::new();
    for edge in edges {
        *in_degrees.entry(edge.to).or_insert(0) += 1;
    }
    
    // Candidates with 0 in-degree
    let mut root_idx = 0;
    for (i, node) in nodes.iter().enumerate() {
        if *in_degrees.get(&node.id).unwrap_or(&0) == 0 {
            root_idx = i;
            break;
        }
    }

    let mut visited = BTreeMap::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited.insert(nodes[root_idx].id, true);
    queue.push_back(nodes[root_idx].id);
    order.push(root_idx);

    while let Some(u_id) = queue.pop_front() {
        if let Some(neighbors) = adj.get(&u_id) {
            // Sort neighbors for deterministic behavior
            let mut neighbors = neighbors.clone();
            neighbors.sort();
            
            for &v_id in &neighbors {
                if !visited.contains_key(&v_id) {
                    visited.insert(v_id, true);
                    if let Some(&idx) = id_to_idx.get(&v_id) {
                        order.push(idx);
                        queue.push_back(v_id);
                    }
                }
            }
        }
    }

    // Add any disconnected components
    for i in 0..nodes.len() {
        if !visited.contains_key(&nodes[i].id) {
            order.push(i);
        }
    }

    order
}

pub fn compute_layout(nodes: &mut [LayoutNode], edges: &[LayoutEdge], settings: &LayoutSettings) {
    if nodes.is_empty() {
        return;
    }

    // 1. Determine Order (BFS from Root)
    let order = bfs_ordering(nodes, edges);

    // 2. Spiral Placement
    // Formula:
    // r = c * sqrt(n)
    // theta = n * divergence_angle (Golden Angle is approx 2.39996 radians)
    
    // Constants
    // Use grid_w as base stride.
    let c = settings.grid_w * 1.2; 
    
    // We want the root at the visual center of the likely 800x600 window.
    let center_x = 400.0;
    let center_y = 300.0;
    
    for (i, &node_idx) in order.iter().enumerate() {
        if i == 0 {
            nodes[node_idx].x = center_x;
            nodes[node_idx].y = center_y;
            continue;
        }

        // Using Vogel's model for phyllotaxis
        let n = i as f32;
        let theta = n * 2.3999632; // Golden angle in radians
        let r = c * libm::sqrtf(n);
        
        nodes[node_idx].x = center_x + r * libm::cosf(theta);
        nodes[node_idx].y = center_y + r * libm::sinf(theta);
    }
}

pub fn route_edges(
    nodes: &[LayoutNode],
    edges: &[LayoutEdge],
    _settings: &LayoutSettings,
) -> BTreeMap<(ThingId, ThingId), Vec<(f32, f32)>> {
    let mut routes = BTreeMap::new();
    let mut id_map = BTreeMap::new();
    for node in nodes {
        id_map.insert(node.id, node);
    }

    for edge in edges {
        if let (Some(src), Some(tgt)) = (id_map.get(&edge.from), id_map.get(&edge.to)) {
            // Direct line from Center to Center (or port to port)
            // Existing painter uses center for nodes.
            // Let's just give center points. 
            // The drawing code might want to clip to the rect? 
            // The current main.rs routing_algo_orthogonal calculated ports.
            // Let's calculate simple ports: center-to-center intersection?
            // Or just strict Center Center and let the painter handle occlusion (or just draw under).
            // But main.rs draws arrowheads.
            // Let's assume Center -> Center is fine for now, main.rs might just draw it.
            // HOWEVER, main.rs orthogonal routing calculated specific ports (bottom center -> top center).
            // For spiral, any angle is possible.
            // Let's return Center -> Center.
            
            let p1 = (src.x, src.y);
            let p2 = (tgt.x, tgt.y);
            
            routes.insert((edge.from, edge.to), alloc::vec![p1, p2]);
        }
    }
    routes
}

// Unused legacy functions removed (assign_ranks, routing_algo_orthogonal, grid_placement)


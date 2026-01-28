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

    // 3. Assign Ranks
    assign_ranks(nodes, &internal_nodes);

    // 4. Grid Placement
    grid_placement(nodes, &internal_nodes, settings);
}

// Calculate orthogonal routes for edges
pub fn route_edges(
    nodes: &[LayoutNode],
    edges: &[LayoutEdge],
    settings: &LayoutSettings,
) -> BTreeMap<(ThingId, ThingId), Vec<(f32, f32)>> {
    let mut routes = BTreeMap::new();
    let mut id_map = BTreeMap::new();
    for node in nodes {
        id_map.insert(node.id, node);
    }

    for edge in edges {
        if let (Some(src), Some(tgt)) = (id_map.get(&edge.from), id_map.get(&edge.to)) {
            let path = routing_algo_orthogonal(src, tgt, settings);
            routes.insert((edge.from, edge.to), path);
        }
    }
    routes
}

fn routing_algo_orthogonal(
    src: &LayoutNode,
    tgt: &LayoutNode,
    settings: &LayoutSettings,
) -> Vec<(f32, f32)> {
    let start_x = src.x;
    let start_y = src.y; // Center
    let end_x = tgt.x;
    let end_y = tgt.y;

    let half_w = src.w / 2.0;
    let half_h = src.h / 2.0;

    // Output port: Bottom center of Source
    let p0 = (start_x, start_y + half_h);
    // Input port: Top center of Target
    let p_last = (end_x, end_y - half_h);

    let mut path = Vec::new();
    path.push(p0);

    // Simple 3-segment routing (Manhattan)
    // 1. Down to mid-rank
    // 2. Over to target X
    // 3. Down to target Y

    // Determine "Mid Y".
    // If target is strictly below source (higher Rank Y), we can go halfway.
    if end_y > start_y + src.h {
         let mid_y = start_y + half_h + (settings.grid_h * 0.5);
         // Point A: Down
         path.push((start_x, mid_y));
         // Point B: Over
         path.push((end_x, mid_y));
         // Point C: Down (to target)
         path.push(p_last);
    } else {
        // Back-edge or same-rank edge. Needs to loop around.
        // For simple flowcharts, we can route around the side?
        // Let's do a simple C-shape out to the right.
        let avoidance_x = (start_x + half_w).max(end_x + half_w) + 20.0;
        
        path.push((start_x, start_y + half_h + 20.0)); // Down a bit
        path.push((avoidance_x, start_y + half_h + 20.0)); // Out
        path.push((avoidance_x, end_y - half_h - 20.0)); // Vertical traversal
        path.push((end_x, end_y - half_h - 20.0)); // In
        path.push(p_last);
    }

    path
}

fn assign_ranks(nodes: &mut [LayoutNode], internal: &[InternalNode]) {
    let mut queue = VecDeque::new();

    // Reset ranks for non-fixed nodes
    for node in nodes.iter_mut() {
        if !node.fixed {
            node.rank = -1;
        }
    }

    // Initialize with forced roots or 0-indegree
    for (i, internal_node) in internal.iter().enumerate() {
        if internal_node.neighbors_in.is_empty() {
             if nodes[i].rank == -1 {
                nodes[i].rank = 0;
            }
            queue.push_back(i);
        } else if nodes[i].rank == 0 {
            queue.push_back(i);
        }
    }

    // Default root if empty
    if queue.is_empty() && !nodes.is_empty() {
        if nodes[0].rank == -1 {
            nodes[0].rank = 0;
        }
        queue.push_back(0);
    }

    let max_passes = nodes.len() * 2;
    let mut passes = 0;
    
    // BFS / Longest Path
    let mut changed = true;
    while changed && passes < max_passes {
        changed = false;
        passes += 1;

        for i in 0..nodes.len() {
            let u_rank = nodes[i].rank;
            if u_rank == -1 { continue; }

            for &v_idx in &internal[i].neighbors_out {
                if !nodes[v_idx].fixed {
                     if nodes[v_idx].rank < u_rank + 1 {
                        nodes[v_idx].rank = u_rank + 1;
                        changed = true;
                    }
                }
            }
        }
    }
    
    // Fallback
    for node in nodes.iter_mut() {
        if node.rank == -1 {
            node.rank = 0;
        }
    }
}

fn grid_placement(
    nodes: &mut [LayoutNode], 
    internal: &[InternalNode],
    settings: &LayoutSettings
) {
    // Group by rank
    let mut layers: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        layers.entry(node.rank).or_default().push(i);
    }

    for (rank, indices) in &mut layers {
        // Sort
        indices.sort_by(|&a, &b| {
            // Heuristic: Median X of parents?
             // For now, stable sort by connection count then ID
            let nc_a = internal[a].connection_count;
            let nc_b = internal[b].connection_count;
            match nc_b.cmp(&nc_a) {
                Ordering::Equal => nodes[a].id.cmp(&nodes[b].id),
                o => o,
            }
        });

        // Determine Y based on Rank (Grid Coordinates)
        // Rank 0 = Y 0
        // Rank 1 = Y 1 (which maps to grid_h pixels)
        let grid_y = *rank; 
        
        let mut grid_x = 0;
        
        for &idx in indices.iter() {
            let node = &mut nodes[idx];
            
            if !node.fixed {
                // Snap center to grid
                // To keep it simple, let's map grid coordinates to pixels here
                // We'll treat x as "column index"
                
                // Center-based coordinates
                // node.x is center X
                // column 0 center = grid_w / 2
                // column 1 center = grid_w + grid_w/2
                
                // Let's just step by whole grid units
                // If grid_w is the stride.
                let center_x = (grid_x as f32 * settings.grid_w) + (settings.grid_w / 2.0);
                let center_y = (grid_y as f32 * settings.grid_h) + (settings.grid_h / 2.0);
                
                node.x = center_x;
                node.y = center_y;
                
                grid_x += 1;
            } else {
                // It's fixed, but maybe snap it to the nearest grid cell?
                // Or leave it alone. The prompt asks for "lay the things out in a simple grid"
                // Let's snap fixed nodes too if they are "close enough", 
                // but usually fixed means "user moved it", so we should respect pixel coords.
                // But for the grid system to work well, we might want to Quantize.
                
                // Let's Quantize fixed nodes to nearest grid slot to allow routing to work better?
                // For now, trust the fixed position but maybe align the rank?
                
                // Update grid_x to avoid overlap if we swept past it?
                // Calculate which column this fixed node occupies
                let col = libm::roundf(node.x / settings.grid_w) as i32;
                if col >= grid_x {
                    grid_x = col + 1;
                }
            }
        }
    }
}


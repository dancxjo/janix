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
    pub vx: f32, // velocity X
    pub vy: f32, // velocity Y
    pub fixed: bool,
    pub pinned: bool, // user-pinned (cannot move)
    pub rank: i32,
    pub gen: u64, // layout generation
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
    pub rank_separation: f32,
    pub node_separation: f32,
    
    // Force-directed physics parameters
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub damping: f32,
    pub min_distance: f32, // minimum separation between nodes
    pub time_budget_ms: u64, // max time per tick for layout
    pub max_iterations: usize, // max iterations per tick
    pub stability_threshold: f32, // movement threshold to consider stable
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            grid_w: 160.0,
            grid_h: 120.0,
            rank_separation: 1.0,
            node_separation: 1.0,
            
            repulsion_strength: 5000.0,
            attraction_strength: 0.3,
            damping: 0.85,
            min_distance: 20.0,
            time_budget_ms: 5,
            max_iterations: 20,
            stability_threshold: 0.5,
        }
    }
}

// Helper to establish a stable ordering for the initial placement
fn bfs_ordering(nodes: &[LayoutNode], edges: &[LayoutEdge]) -> Vec<usize> {
    let mut adj: BTreeMap<ThingId, Vec<ThingId>> = BTreeMap::new();
    let mut id_to_idx = BTreeMap::new();

    for (i, node) in nodes.iter().enumerate() {
        id_to_idx.insert(node.id, i);
        adj.entry(node.id).or_default();
    }

    for edge in edges {
        adj.entry(edge.from).or_default().push(edge.to);
        adj.entry(edge.to).or_default().push(edge.from);
    }

    // Find root: node with most connections or no incoming edges
    let mut in_degrees = BTreeMap::new();
    for edge in edges {
        *in_degrees.entry(edge.to).or_insert(0) += 1;
    }

    let mut root_idx = 0;
    let mut max_connections = 0;
    
    for (i, node) in nodes.iter().enumerate() {
        let in_deg = *in_degrees.get(&node.id).unwrap_or(&0);
        let connections = adj.get(&node.id).map(|v| v.len()).unwrap_or(0);
        
        if in_deg == 0 && connections > max_connections {
            root_idx = i;
            max_connections = connections;
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

/// Initial placement using spiral or previous positions
fn initial_placement(nodes: &mut [LayoutNode], edges: &[LayoutEdge], settings: &LayoutSettings) {
    let center_x = 400.0;
    let center_y = 300.0;
    
    // Build neighbor map for new node placement
    let mut neighbors: BTreeMap<ThingId, Vec<ThingId>> = BTreeMap::new();
    for edge in edges {
        neighbors.entry(edge.from).or_default().push(edge.to);
        neighbors.entry(edge.to).or_default().push(edge.from);
    }
    
    // Build id-to-index map for neighbor lookups
    let mut id_to_idx: BTreeMap<ThingId, usize> = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        id_to_idx.insert(node.id, i);
    }
    
    // Compute BFS ordering once upfront
    let order = bfs_ordering(nodes, edges);
    
    for i in 0..nodes.len() {
        // Skip nodes that already have positions (preserve existing layout)
        if nodes[i].x != 0.0 || nodes[i].y != 0.0 {
            continue;
        }
        
        // Skip pinned nodes (they stay where they are)
        if nodes[i].pinned {
            continue;
        }
        
        let node_id = nodes[i].id;
        
        // For new nodes, place near neighbors if they exist
        if let Some(nbrs) = neighbors.get(&node_id) {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut count = 0;
            
            for nbr_id in nbrs {
                if let Some(&nbr_idx) = id_to_idx.get(nbr_id) {
                    if nodes[nbr_idx].x != 0.0 || nodes[nbr_idx].y != 0.0 {
                        sum_x += nodes[nbr_idx].x;
                        sum_y += nodes[nbr_idx].y;
                        count += 1;
                    }
                }
            }
            
            if count > 0 {
                // Place near average of neighbors with small random offset
                nodes[i].x = sum_x / count as f32 + (node_id.to_u64_lossy() as f32 % 50.0 - 25.0);
                nodes[i].y = sum_y / count as f32 + ((node_id.to_u64_lossy() >> 8) as f32 % 50.0 - 25.0);
                continue;
            }
        }
        
        // Fallback: spiral placement for isolated nodes
        let idx = order.iter().position(|&ord_idx| nodes[ord_idx].id == node_id).unwrap_or(0);
        
        if idx == 0 {
            nodes[i].x = center_x;
            nodes[i].y = center_y;
        } else {
            let n = idx as f32;
            let theta = n * 2.3999632; // Golden angle
            let r = settings.grid_w * 1.2 * libm::sqrtf(n);
            nodes[i].x = center_x + r * libm::cosf(theta);
            nodes[i].y = center_y + r * libm::sinf(theta);
        }
    }
}

/// Apply repulsion forces between nodes (avoid overlap)
fn apply_repulsion(nodes: &mut [LayoutNode], settings: &LayoutSettings) {
    let n = nodes.len();
    
    for i in 0..n {
        if nodes[i].pinned {
            continue;
        }
        
        let mut fx = 0.0;
        let mut fy = 0.0;
        
        for j in 0..n {
            if i == j {
                continue;
            }
            
            let dx = nodes[i].x - nodes[j].x;
            let dy = nodes[i].y - nodes[j].y;
            let dist_sq = dx * dx + dy * dy;
            
            if dist_sq < 0.01 {
                // Nodes too close, apply random displacement
                fx += (nodes[i].id.to_u64_lossy() as f32 % 10.0 - 5.0);
                fy += ((nodes[i].id.to_u64_lossy() >> 8) as f32 % 10.0 - 5.0);
                continue;
            }
            
            let dist = libm::sqrtf(dist_sq);
            
            // Rectangle collision check
            let overlap_x = (nodes[i].w + nodes[j].w) / 2.0 + settings.min_distance - dist.abs();
            let overlap_y = (nodes[i].h + nodes[j].h) / 2.0 + settings.min_distance - dist.abs();
            
            if overlap_x > 0.0 || overlap_y > 0.0 {
                // Strong repulsion for overlapping nodes
                let force = settings.repulsion_strength / (dist_sq + 1.0);
                fx += (dx / dist) * force;
                fy += (dy / dist) * force;
            } else {
                // Weaker repulsion for distant nodes
                let force = settings.repulsion_strength / (dist_sq + 100.0);
                fx += (dx / dist) * force;
                fy += (dy / dist) * force;
            }
        }
        
        nodes[i].vx += fx;
        nodes[i].vy += fy;
    }
}

/// Apply attraction forces along edges
fn apply_attraction(nodes: &mut [LayoutNode], edges: &[LayoutEdge], settings: &LayoutSettings) {
    let mut id_to_idx = BTreeMap::new();
    for (i, node) in nodes.iter().enumerate() {
        id_to_idx.insert(node.id, i);
    }
    
    for edge in edges {
        let i = match id_to_idx.get(&edge.from) {
            Some(&idx) => idx,
            None => continue,
        };
        let j = match id_to_idx.get(&edge.to) {
            Some(&idx) => idx,
            None => continue,
        };
        
        let dx = nodes[j].x - nodes[i].x;
        let dy = nodes[j].y - nodes[i].y;
        let dist = libm::sqrtf(dx * dx + dy * dy);
        
        if dist < 0.01 {
            continue;
        }
        
        let force = settings.attraction_strength * edge.weight;
        let fx = (dx / dist) * force * dist;
        let fy = (dy / dist) * force * dist;
        
        if !nodes[i].pinned {
            nodes[i].vx += fx;
            nodes[i].vy += fy;
        }
        
        if !nodes[j].pinned {
            nodes[j].vx -= fx;
            nodes[j].vy -= fy;
        }
    }
}

/// Resolve rectangle overlaps by pushing nodes apart
fn resolve_collisions(nodes: &mut [LayoutNode], settings: &LayoutSettings) {
    let n = nodes.len();
    
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = nodes[i].x - nodes[j].x;
            let dy = nodes[i].y - nodes[j].y;
            
            let half_w = (nodes[i].w + nodes[j].w) / 2.0 + settings.min_distance;
            let half_h = (nodes[i].h + nodes[j].h) / 2.0 + settings.min_distance;
            
            // Check for rectangle overlap
            if dx.abs() < half_w && dy.abs() < half_h {
                // Calculate minimal separating vector
                let overlap_x = half_w - dx.abs();
                let overlap_y = half_h - dy.abs();
                
                let (push_x, push_y) = if overlap_x < overlap_y {
                    // Separate horizontally
                    if dx > 0.0 {
                        (overlap_x / 2.0, 0.0)
                    } else {
                        (-overlap_x / 2.0, 0.0)
                    }
                } else {
                    // Separate vertically
                    if dy > 0.0 {
                        (0.0, overlap_y / 2.0)
                    } else {
                        (0.0, -overlap_y / 2.0)
                    }
                };
                
                if !nodes[i].pinned {
                    nodes[i].x += push_x;
                    nodes[i].y += push_y;
                }
                
                if !nodes[j].pinned {
                    nodes[j].x -= push_x;
                    nodes[j].y -= push_y;
                }
            }
        }
    }
}

/// Update positions based on velocities and apply damping
fn update_positions(nodes: &mut [LayoutNode], settings: &LayoutSettings) -> f32 {
    let mut max_movement = 0.0;
    
    for node in nodes.iter_mut() {
        if node.pinned {
            node.vx = 0.0;
            node.vy = 0.0;
            continue;
        }
        
        // Apply damping
        node.vx *= settings.damping;
        node.vy *= settings.damping;
        
        // Update position
        node.x += node.vx;
        node.y += node.vy;
        
        // Track maximum movement
        let movement = libm::sqrtf(node.vx * node.vx + node.vy * node.vy);
        if movement > max_movement {
            max_movement = movement;
        }
    }
    
    max_movement
}

pub fn compute_layout(nodes: &mut [LayoutNode], edges: &[LayoutEdge], settings: &LayoutSettings) {
    if nodes.is_empty() {
        return;
    }

    // Stage A: Coarse placement (only for new nodes)
    initial_placement(nodes, edges, settings);

    // Stage B: Physics refinement (incremental force layout)
    for _iteration in 0..settings.max_iterations {
        // Reset velocities from forces
        for node in nodes.iter_mut() {
            if !node.pinned {
                node.vx *= 0.5; // Decay previous frame's velocity
                node.vy *= 0.5;
            }
        }
        
        // Apply forces
        apply_repulsion(nodes, settings);
        apply_attraction(nodes, edges, settings);
        
        // Update positions and check for stability
        let max_movement = update_positions(nodes, settings);
        
        // Resolve any remaining collisions
        resolve_collisions(nodes, settings);
        
        // Early exit if stable
        if max_movement < settings.stability_threshold {
            break;
        }
    }
    
    // Increment generation for all modified nodes
    for node in nodes.iter_mut() {
        node.gen += 1;
    }
}

/// Calculate intersection point of line from center to edge of rectangle
fn rect_edge_intersection(cx: f32, cy: f32, w: f32, h: f32, tx: f32, ty: f32) -> (f32, f32) {
    let dx = tx - cx;
    let dy = ty - cy;
    
    if dx.abs() < 0.01 && dy.abs() < 0.01 {
        return (cx, cy);
    }
    
    let half_w = w / 2.0;
    let half_h = h / 2.0;
    
    // Find intersection with rectangle edges
    let t_vertical = if dx.abs() > 0.01 {
        half_w / dx.abs()
    } else {
        f32::MAX
    };
    
    let t_horizontal = if dy.abs() > 0.01 {
        half_h / dy.abs()
    } else {
        f32::MAX
    };
    
    let t = t_vertical.min(t_horizontal);
    
    (cx + dx * t, cy + dy * t)
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
            // Calculate edge attachment points on node boundaries
            let src_point = rect_edge_intersection(src.x, src.y, src.w, src.h, tgt.x, tgt.y);
            let tgt_point = rect_edge_intersection(tgt.x, tgt.y, tgt.w, tgt.h, src.x, src.y);

            routes.insert((edge.from, edge.to), alloc::vec![src_point, tgt_point]);
        }
    }
    routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_overlapping_nodes() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Create a simple graph with 5 nodes
        for i in 0..5 {
            nodes.push(LayoutNode {
                id: ThingId::from_u64(i as u64 + 1),
                x: 0.0,
                y: 0.0,
                w: 120.0,
                h: 160.0,
                vx: 0.0,
                vy: 0.0,
                fixed: false,
                pinned: false,
                rank: 0,
                gen: 0,
            });
        }

        // Create edges in a chain
        for i in 0..4 {
            edges.push(LayoutEdge {
                from: ThingId::from_u64(i as u64 + 1),
                to: ThingId::from_u64(i as u64 + 2),
                weight: 1.0,
            });
        }

        let settings = LayoutSettings::default();
        compute_layout(&mut nodes, &edges, &settings);

        // Check that no nodes overlap
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let n1 = &nodes[i];
                let n2 = &nodes[j];

                let dx = (n1.x - n2.x).abs();
                let dy = (n1.y - n2.y).abs();
                let min_dx = (n1.w + n2.w) / 2.0 + settings.min_distance;
                let min_dy = (n1.h + n2.h) / 2.0 + settings.min_distance;

                // Assert no overlap
                assert!(
                    dx >= min_dx || dy >= min_dy,
                    "Nodes {} and {} overlap: dx={}, dy={}, min_dx={}, min_dy={}",
                    n1.id.to_u64_lossy(),
                    n2.id.to_u64_lossy(),
                    dx,
                    dy,
                    min_dx,
                    min_dy
                );
            }
        }
    }

    #[test]
    fn test_pinned_nodes_stay_fixed() {
        let mut nodes = vec![
            LayoutNode {
                id: ThingId::from_u64(1),
                x: 100.0,
                y: 100.0,
                w: 120.0,
                h: 160.0,
                vx: 0.0,
                vy: 0.0,
                fixed: false,
                pinned: true, // This node is pinned
                rank: 0,
                gen: 0,
            },
            LayoutNode {
                id: ThingId::from_u64(2),
                x: 0.0,
                y: 0.0,
                w: 120.0,
                h: 160.0,
                vx: 0.0,
                vy: 0.0,
                fixed: false,
                pinned: false,
                rank: 0,
                gen: 0,
            },
        ];

        let edges = vec![LayoutEdge {
            from: ThingId::from_u64(1),
            to: ThingId::from_u64(2),
            weight: 1.0,
        }];

        let settings = LayoutSettings::default();
        let orig_x = nodes[0].x;
        let orig_y = nodes[0].y;

        compute_layout(&mut nodes, &edges, &settings);

        // Pinned node should not have moved
        assert_eq!(nodes[0].x, orig_x, "Pinned node X position changed");
        assert_eq!(nodes[0].y, orig_y, "Pinned node Y position changed");
        
        // Pinned node should have zero velocity
        assert_eq!(nodes[0].vx, 0.0, "Pinned node has non-zero X velocity");
        assert_eq!(nodes[0].vy, 0.0, "Pinned node has non-zero Y velocity");
    }

    #[test]
    fn test_edge_routing() {
        let nodes = vec![
            LayoutNode {
                id: ThingId::from_u64(1),
                x: 0.0,
                y: 0.0,
                w: 120.0,
                h: 160.0,
                vx: 0.0,
                vy: 0.0,
                fixed: false,
                pinned: false,
                rank: 0,
                gen: 0,
            },
            LayoutNode {
                id: ThingId::from_u64(2),
                x: 200.0,
                y: 0.0,
                w: 120.0,
                h: 160.0,
                vx: 0.0,
                vy: 0.0,
                fixed: false,
                pinned: false,
                rank: 0,
                gen: 0,
            },
        ];

        let edges = vec![LayoutEdge {
            from: ThingId::from_u64(1),
            to: ThingId::from_u64(2),
            weight: 1.0,
        }];

        let settings = LayoutSettings::default();
        let routes = route_edges(&nodes, &edges, &settings);

        // Should have one route
        assert_eq!(routes.len(), 1);

        let route = routes
            .get(&(ThingId::from_u64(1), ThingId::from_u64(2)))
            .unwrap();

        // Route should have exactly 2 points (start and end)
        assert_eq!(route.len(), 2);

        // Start point should be on the edge of node 1, not at center
        assert!(route[0].0 != 0.0 || route[0].1 != 0.0);
        
        // End point should be on the edge of node 2, not at center
        assert!(route[1].0 != 200.0 || route[1].1 != 0.0);
    }
}


// Unused legacy functions removed (assign_ranks, routing_algo_orthogonal, grid_placement)

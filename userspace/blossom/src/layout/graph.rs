extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use core::cmp::Ordering;

use stem::thing::ThingId;
use crate::layout::LayoutRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutEdgeKind {
    Structural,
    Data,
    Control,
    UI,
}

#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub id: ThingId,
    pub rect: LayoutRect,
    pub rank: i32,
    pub fixed: bool,
    // Indices in the nodes vector
    pub neighbors_in: Vec<usize>,
    pub neighbors_out: Vec<usize>,
    // For sorting/stability
    pub connection_count: usize,
}

#[derive(Clone, Debug)]
pub struct LayoutEdge {
    pub source: usize,
    pub target: usize,
    pub weight: f32,
    pub kind: LayoutEdgeKind,
}

pub struct GraphLayout {
    pub nodes: Vec<LayoutNode>,
    pub edges: Vec<LayoutEdge>,
    pub node_indices: BTreeMap<ThingId, usize>,
    
    // Configuration
    pub rank_spacing: i32,
    pub node_spacing: i32,
    pub rank_padding: i32,
}

impl Default for GraphLayout {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_indices: BTreeMap::new(),
            rank_spacing: 120,
            node_spacing: 150,
            rank_padding: 20,
        }
    }
}

impl GraphLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: ThingId, rect: LayoutRect, fixed: bool, rank_hint: Option<i32>) {
        if self.node_indices.contains_key(&id) {
            return;
        }
        let index = self.nodes.len();
        self.nodes.push(LayoutNode {
            id,
            rect,
            rank: rank_hint.unwrap_or(-1), // -1 means unassigned
            fixed,
            neighbors_in: Vec::new(),
            neighbors_out: Vec::new(),
            connection_count: 0,
        });
        self.node_indices.insert(id, index);
    }

    pub fn add_edge(&mut self, source: ThingId, target: ThingId, kind: LayoutEdgeKind, weight: f32) {
        let src_idx = *self.node_indices.get(&source).unwrap_or(&usize::MAX);
        let tgt_idx = *self.node_indices.get(&target).unwrap_or(&usize::MAX);

        if src_idx < self.nodes.len() && tgt_idx < self.nodes.len() {
            self.edges.push(LayoutEdge {
                source: src_idx,
                target: tgt_idx,
                weight,
                kind,
            });
            self.nodes[src_idx].neighbors_out.push(tgt_idx);
            self.nodes[src_idx].connection_count += 1;
            
            self.nodes[tgt_idx].neighbors_in.push(src_idx);
            self.nodes[tgt_idx].connection_count += 1;
        }
    }

    pub fn run(&mut self) {
        self.assign_ranks();
        self.initial_placement();
        self.resolve_collisions();
        // Run a couple refinement passes
        for _ in 0..2 {
            self.refine_edges();
            self.resolve_collisions(); // Re-resolve after nudging
        }
    }

    pub fn get_rect(&self, id: ThingId) -> Option<LayoutRect> {
        self.node_indices.get(&id).map(|&idx| self.nodes[idx].rect)
    }

    // Phase 1: Assign ranks using BFS/Longest path approach
    fn assign_ranks(&mut self) {
        // Collect roots: nodes with no incoming edges (or manually ranked 0)
        // Note: For existing fixed nodes, we might want to respect their implicit rank? 
        // For now, we recalculate ranks based on topology + hints.
        
        let mut queue = VecDeque::new();
        let mut in_degree = vec![0; self.nodes.len()];
        
        for i in 0..self.nodes.len() {
            // Count logical in-degree
            in_degree[i] = self.nodes[i].neighbors_in.len();
            if in_degree[i] == 0 {
                if self.nodes[i].rank == -1 {
                    self.nodes[i].rank = 0;
                }
                queue.push_back(i);
            }
        }
        
        // Handle nodes that might not be roots but have rank hint 0?
        for (i, node) in self.nodes.iter().enumerate() {
            if node.rank == 0 && in_degree[i] > 0 {
                // It's a forced root
                 queue.push_back(i);
            }
        }

        // BFS
        // Use a visited set to avoid cycles
        let mut visited = vec![false; self.nodes.len()];
        
        while let Some(u) = queue.pop_front() {
            if visited[u] { continue; }
            visited[u] = true;
            
            let u_rank = self.nodes[u].rank;
            let neighbors = self.nodes[u].neighbors_out.clone();
            
            for &v in &neighbors {
                if self.nodes[v].fixed {
                     // If v is fixed, we don't update its rank based on u.
                } else {
                    // Update rank of v (Longest Path)
                    if self.nodes[v].rank < u_rank + 1 {
                        self.nodes[v].rank = u_rank + 1;
                    }
                }
                
                if !visited[v] {
                     queue.push_back(v);
                }
            }
        }
        
        // Fallback for unranked nodes (cycles/disconnected)
        for node in &mut self.nodes {
            if node.rank == -1 {
                node.rank = 0;
            }
        }
    }

    fn initial_placement(&mut self) {
        // Group by rank
        let mut layers: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (idx, node) in self.nodes.iter().enumerate() {
            layers.entry(node.rank).or_default().push(idx);
        }

        for (rank, indices) in &mut layers {
            let rank_y = *rank * self.rank_spacing;
            
            // Sort: Stable hash tie-breaker + connection count
            indices.sort_by(|&a, &b| {
                let node_a = &self.nodes[a];
                let node_b = &self.nodes[b];
                
                // Primary sort: try to minimize crossings (heuristic).
                // Sort by average position of parents (barycenter) if available?
                // For initial placement, we might lack parent X positions.
                // Heuristic: number of connections (descending)
                let conn_ord = node_b.connection_count.cmp(&node_a.connection_count);
                if conn_ord != Ordering::Equal {
                    return conn_ord;
                }
                // Tie breaker: ID (stable)
                node_a.id.0.cmp(&node_b.id.0)
            });

            // Allow fixed nodes to stay where they are?
            // "Initial placement: y = rank * SPACING, x = index * SPACING"
            // If a node is fixed, we should probably respect its position, 
            // but maybe we move it to the correct Rank Y?
            // "Neighbors may adjust, the node does not" implies fixed nodes are FIXED.
            
            let mut current_x = 0;
            for &idx in indices.iter() {
                let node = &mut self.nodes[idx];
                
                if node.fixed {
                    // It stays where it is.
                    // But we might want to update current_x to skip past it?
                    current_x = node.rect.x + node.rect.w + self.node_spacing;
                } else {
                    node.rect.y = rank_y;
                    node.rect.x = current_x;
                    current_x += node.rect.w + self.node_spacing;
                }
            }
            
            // Center row?
            if !indices.is_empty() {
                // Optional: center entire rank. 
                // Let's implement Phase 2 collision resolution first, it handles flow.
            }
        }
    }

    // Phase 2: Collision Resolution (Rectangle Packing)
    fn resolve_collisions(&mut self) {
       // Re-group by rank in case ranks changed (they didn't) or Y changed (they shouldn't have)
       // Actually, we process rank by rank.
       
        let mut layers: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (idx, node) in self.nodes.iter().enumerate() {
            layers.entry(node.rank).or_default().push(idx);
        }

        for (_, indices) in &mut layers {
             // sort by current X
             indices.sort_by_key(|&i| self.nodes[i].rect.x);
             
             // Sweep left -> right
             for i in 1..indices.len() {
                 let left_idx = indices[i-1];
                 let curr_idx = indices[i];
                 
                 let left_node = &self.nodes[left_idx];
                 let curr_rect_w = self.nodes[curr_idx].rect.w;
                 let curr_fixed = self.nodes[curr_idx].fixed;
                 let curr_x = self.nodes[curr_idx].rect.x;
                 
                 let min_x = left_node.rect.x + left_node.rect.w + self.rank_padding; // node_spacing?
                 
                 if curr_x < min_x {
                     // Overlap!
                     if curr_fixed {
                         // Can't move current. Push/Pull previous left?
                         // "If rect[i] overlaps rect[i-1], push it right"
                         // If curr is fixed, maybe we should have pushed left_node left earlier?
                         // For now, strict left-to-right push. 
                         // If curr is fixed, we leave it. Overlap might persist if previous was blocked.
                     } else {
                         self.nodes[curr_idx].rect.x = min_x;
                     }
                 }
             }

             // Optional: Center the rank
             if let (Some(&first), Some(&last)) = (indices.first(), indices.last()) {
                 let total_w = self.nodes[last].rect.x + self.nodes[last].rect.w - self.nodes[first].rect.x;
                 // Assuming we want to center around x=0 or some canvas center?
                 // Let's just keep them positive or near their parents.
             }
        }
    }

    // Phase 3: Edge-Aware Refinement (Cheap Physics)
    fn refine_edges(&mut self) {
        // "For each node... Pull slightly toward the average X of its neighbors"
        
        // We need a snapshot of X positions to avoid order-dependency bias (jacobi vs gauss-seidel).
        // Gauss-Seidel (using new positions immediately) converges faster usually.
        // Let's iterate ranks? Or just all nodes?
        
        let damping = 0.5;
        let max_move = 50;

        for i in 0..self.nodes.len() {
            if self.nodes[i].fixed { continue; }
            
            // Calculate barycenter of neighbors
            let mut sum_x = 0;
            let mut count = 0;
            
            // Incoming
            for &n_in in &self.nodes[i].neighbors_in {
                // Center-to-center pull? Or just X pull.
                sum_x += self.nodes[n_in].rect.x + (self.nodes[n_in].rect.w / 2);
                count += 1;
            }
            
            // Outgoing
            for &n_out in &self.nodes[i].neighbors_out {
                sum_x += self.nodes[n_out].rect.x + (self.nodes[n_out].rect.w / 2);
                count += 1;
            }
            
            if count > 0 {
                let target_center_x = sum_x / count;
                let current_center_x = self.nodes[i].rect.x + (self.nodes[i].rect.w / 2);
                
                let delta = target_center_x - current_center_x;
                let move_amt = (delta as f32 * damping) as i32;
                
                // Clamp
                let move_amt = move_amt.clamp(-max_move, max_move);
                
                self.nodes[i].rect.x += move_amt;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ids::HandleId;

    #[test]
    fn test_graph_layout_no_overlap() {
        let mut layout = GraphLayout::new();
        
        let rect_size = LayoutRect { x: 0, y: 0, w: 100, h: 40 };
        
        // Define IDs
        let kernel = ThingId::from_u64(1);
        let platform = ThingId::from_u64(2);
        let host = ThingId::from_u64(3);
        let root = ThingId::from_u64(4);
        let scheduler = ThingId::from_u64(5);
        let window1 = ThingId::from_u64(6);
        let window2 = ThingId::from_u64(7);
        
        // Roots: Kernel, Platform
        layout.add_node(kernel, rect_size, false, Some(0));
        layout.add_node(platform, rect_size, false, Some(0));
        
        // Others
        layout.add_node(host, rect_size, false, None);
        layout.add_node(root, rect_size, false, None);
        layout.add_node(scheduler, rect_size, false, None);
        layout.add_node(window1, rect_size, false, None);
        layout.add_node(window2, rect_size, false, None);
        
        // Edges (Kernel -> Host -> Root -> Scheduler -> Windows)
        // Note: For Top-Down layout (Rank Y increasing), Parent -> Child
        layout.add_edge(kernel, host, LayoutEdgeKind::Structural, 1.0);
        layout.add_edge(platform, host, LayoutEdgeKind::Structural, 1.0);
        layout.add_edge(host, root, LayoutEdgeKind::Structural, 1.0);
        layout.add_edge(root, scheduler, LayoutEdgeKind::Structural, 1.0);
        layout.add_edge(scheduler, window1, LayoutEdgeKind::Control, 1.0);
        layout.add_edge(scheduler, window2, LayoutEdgeKind::Control, 1.0);
        
        layout.run();
        
        // Check overlaps
        for i in 0..layout.nodes.len() {
            for j in (i+1)..layout.nodes.len() {
                let r1 = layout.nodes[i].rect;
                let r2 = layout.nodes[j].rect;
               
                let overlap_x = r1.x < r2.x + r2.w && r1.x + r1.w > r2.x;
                let overlap_y = r1.y < r2.y + r2.h && r1.y + r1.h > r2.y;
                
                if overlap_x && overlap_y {
                     panic!("Overlap between {:?} ({:?}) and {:?} ({:?})", 
                        layout.nodes[i].id, r1, layout.nodes[j].id, r2);
                }
            }
        }

        // Check Ranking order
        let r_kernel = layout.get_rect(kernel).unwrap();
        let r_host = layout.get_rect(host).unwrap();
        let r_root = layout.get_rect(root).unwrap();
        let r_scheduler = layout.get_rect(scheduler).unwrap();
        
        assert!(r_kernel.y <= r_host.y, "Kernel Y {} should be <= Host Y {}", r_kernel.y, r_host.y);
        assert!(r_host.y < r_root.y, "Host Y {} should be < Root Y {}", r_host.y, r_root.y);
        assert!(r_root.y < r_scheduler.y, "Root Y {} should be < Scheduler Y {}", r_root.y, r_scheduler.y);
    }
}

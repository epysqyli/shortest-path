mod edge;
pub mod node;
mod path;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::path::Path;

type NodeID = usize;
type EdgeID = usize;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(self: &mut Self, x: f32, y: f32) -> Result<NodeID, ()> {
        if self.get_node_by_coords(x, y).is_some() {
            Err(())
        } else {
            let new_node_id = if self.nodes.is_empty() {
                0
            } else {
                self.nodes.last().unwrap().id + 1
            };

            self.nodes.push(Node {
                x,
                y,
                id: new_node_id,
                edges: vec![],
            });

            Ok(new_node_id)
        }
    }

    pub fn create_edge(self: &mut Self, from_id: NodeID, to_id: NodeID) -> Result<EdgeID, ()> {
        let from_node = self.nodes.get(from_id);
        let to_node = self.nodes.get(to_id);

        if from_node.is_some() && to_node.is_some() {
            let new_edge_id = if self.edges.is_empty() {
                0
            } else {
                self.edges.last().unwrap().id + 1
            };

            let from_node = from_node.unwrap();
            let to_node = to_node.unwrap();

            self.edges.push(Edge {
                a: from_id,
                b: to_id,
                id: new_edge_id,
                distance: f32::sqrt((to_node.x - from_node.x).powf(2.0) + (to_node.y - from_node.y).powf(2.0)),
            });

            let from_node = self.nodes.get_mut(from_id).unwrap();
            from_node.edges.push(new_edge_id);

            let to_node = self.nodes.get_mut(to_id).unwrap();
            to_node.edges.push(new_edge_id);

            Ok(new_edge_id)
        } else {
            Err(())
        }
    }

    pub fn find_shortest_path(self: &Self, from_id: NodeID, to_id: NodeID) -> Option<(Path, f32)> {
        let mut unique_paths: Vec<Path> = vec![];

        {
            let mut paths: Vec<Path> = vec![];
            self.create_or_extend_paths(&mut paths, from_id, from_id, from_id, to_id, None);

            // remove duplicate paths
            let paths_len = paths.len();
            for (i, p) in paths.iter().enumerate() {
                if !&paths[i + 1..paths_len].contains(p) {
                    unique_paths.push(p.clone());
                }
            }
        }

        // filter out paths that do not end on the desired to_id node
        let eligible_paths: Vec<Path> = unique_paths
            .into_iter()
            .filter(|p| p.0.first().unwrap() == &from_id && p.0.last().unwrap() == &to_id)
            .collect();

        if eligible_paths.is_empty() {
            return None;
        }

        // find the shortest path based on distance
        let mut distances: Vec<f32> = vec![];
        eligible_paths.iter().for_each(|path| {
            let mut distance: f32 = 0.0;
            path.0
                .iter()
                .zip(path.0.iter().skip(1))
                .for_each(|(node_a_id, node_b_id)| {
                    distance += self.get_edge_between_nodes(*node_a_id, *node_b_id).unwrap().distance;
                });

            distances.push(distance);
        });

        let mut shortest_path_index: usize = 0;
        distances.iter().enumerate().for_each(|(i, dist)| {
            if *dist < distances[shortest_path_index] {
                shortest_path_index = i;
            }
        });

        Some((
            eligible_paths[shortest_path_index].clone(),
            distances[shortest_path_index],
        ))
    }

    fn create_or_extend_paths(
        self: &Self,
        paths: &mut Vec<Path>,
        prev_id: NodeID,
        cur_id: NodeID,
        origin_id: NodeID,
        dest_id: NodeID,
        path: Option<Path>,
    ) {
        let cur_node = self.get_node_by_id(cur_id).unwrap();

        for edge_id in &cur_node.edges {
            let edge = self.get_edge_by_id(*edge_id).unwrap();
            let next_node_id = edge.get_other_node_id(cur_id);

            if path.is_some() {
                let new_path = path.clone().unwrap();
                if new_path.0.contains(&next_node_id) {
                    paths.push(new_path);
                    continue;
                }
            }

            if path.is_some() {
                if &prev_id == path.clone().unwrap().0.last().unwrap() {
                    let mut new_path = path.clone().unwrap();
                    new_path.0.push(cur_id);
                    new_path.0.push(next_node_id);
                    paths.push(new_path);
                }
            }

            if cur_id == dest_id {
                if let Some(ref cur_path) = path {
                    let mut extended_path = cur_path.clone();
                    extended_path.0.push(cur_id);
                    paths.push(extended_path);
                }

                break;
            }

            if next_node_id == prev_id {
                if cur_node.edges.len() == 1 {
                    if let Some(ref cur_path) = path {
                        let mut extended_path = cur_path.clone();
                        extended_path.0.push(cur_id);
                        paths.push(extended_path);
                    }
                }

                continue;
            }

            if next_node_id == origin_id {
                if let Some(ref cur_path) = path {
                    let mut extended_path = cur_path.clone();
                    extended_path.0.push(cur_id);
                    paths.push(extended_path);
                }

                break;
            }

            match path {
                Some(ref cur_path) => {
                    let mut ext_path = cur_path.clone();
                    ext_path.0.push(cur_id);
                    self.create_or_extend_paths(paths, cur_node.id, next_node_id, origin_id, dest_id, Some(ext_path));
                }
                None => {
                    let new_path = Path(vec![cur_id]);
                    self.create_or_extend_paths(paths, cur_node.id, next_node_id, origin_id, dest_id, Some(new_path));
                }
            }
        }
    }

    fn get_node_by_id(self: &Self, node_id: NodeID) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    fn get_edge_by_id(self: &Self, edge_id: EdgeID) -> Option<&Edge> {
        self.edges.get(edge_id)
    }

    fn get_node_by_coords(self: &Self, x: f32, y: f32) -> Option<NodeID> {
        match self.nodes.iter().find(|n| n.x == x && n.y == y) {
            Some(n) => Some(n.id),
            None => None,
        }
    }

    fn get_edge_between_nodes(self: &Self, from_id: NodeID, to_id: NodeID) -> Option<&Edge> {
        let a_to_b = self.edges.iter().find(|e| e.a == from_id && e.b == to_id);
        if a_to_b.is_some() {
            return a_to_b;
        }

        let b_to_a = self.edges.iter().find(|e| e.b == from_id && e.a == to_id);
        if b_to_a.is_some() {
            return b_to_a;
        }

        None
    }
}

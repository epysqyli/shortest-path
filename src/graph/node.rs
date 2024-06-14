use super::EdgeID;
use super::NodeID;

#[derive(Debug)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub id: NodeID,
    pub edges: Vec<EdgeID>,
}

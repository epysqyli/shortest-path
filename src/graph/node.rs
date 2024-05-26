use super::Edge;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub connections: Vec<Rc<Edge>>,
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            connections: vec![],
        }
    }

    pub fn add_connection(self: &mut Self, edge: Rc<Edge>) {
        self.connections.push(edge)
    }
}

impl PartialEq for Node {
    fn eq(self: &Self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

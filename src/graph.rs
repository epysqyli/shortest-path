mod edge;
mod node;

use edge::Edge;
use node::Node;
use std::rc::Rc;

/// Raw collection of all existing points and edges
#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Rc<Node>>,
    edges: Vec<Rc<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: vec![],
            nodes: vec![],
        }
    }

    pub fn get_node(self: &Self, x: f32, y: f32) -> Option<Rc<Node>> {
        match self.nodes.iter().find(|p| p.x == x && p.y == y) {
            Some(p) => Some(Rc::clone(p)),
            None => None,
        }
    }

    pub fn add_node(self: &mut Self, x: f32, y: f32) {
        match self.get_node(x, y) {
            None => self.nodes.push(Rc::new(Node::new(x, y))),
            _ => {}
        }
    }

    pub fn get_edge(self: &Self, a: Rc<Node>, b: Rc<Node>) -> Option<Rc<Edge>> {
        match self.edges.iter().find(|edge| edge.a == a && edge.b == b) {
            Some(e) => Some(Rc::clone(e)),
            None => None,
        }
    }

    pub fn add_edge(self: &mut Self, a: Rc<Node>, b: Rc<Node>) {
        match self.get_edge(Rc::clone(&a), Rc::clone(&b)) {
            Some(_) => {}
            None => self.edges.push(Rc::new(Edge::new(Rc::clone(&a), Rc::clone(&b)))),
        }
    }
}

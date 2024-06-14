use super::EdgeID;
use super::NodeID;

#[derive(Debug)]
pub struct Edge {
    pub id: EdgeID,
    pub a: NodeID,
    pub b: NodeID,
    pub distance: f32,
}

impl Edge {
    pub fn get_other_node_id(self: &Self, node_id: NodeID) -> NodeID {
        if node_id == self.a {
            self.b
        } else {
            self.a
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Edge;
    use crate::graph::Node;

    #[test]
    fn other_node_id_is_correct() {
        let node_1 = Node {
            id: 0,
            x: 1.0,
            y: 1.0,
            edges: vec![],
        };

        let node_2 = Node {
            id: 1,
            x: 2.0,
            y: 2.0,
            edges: vec![],
        };

        let edge = Edge {
            id: 0,
            a: node_1.id,
            b: node_2.id,
            distance: 0.0,
        };

        assert_eq!(edge.get_other_node_id(node_1.id), 1);
        assert_eq!(edge.get_other_node_id(node_2.id), 0);
    }
}

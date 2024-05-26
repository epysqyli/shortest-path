use super::Node;
use std::rc::Rc;

#[derive(Debug)]
pub struct Edge {
    pub a: Rc<Node>,
    pub b: Rc<Node>,
}

impl Edge {
    pub fn new(a: Rc<Node>, b: Rc<Node>) -> Self {
        Self { a, b }
    }

    pub fn distance(self: &Self) -> f32 {
        f32::sqrt((self.b.x - self.a.x).powf(2.0) + (self.b.y - self.a.y).powf(2.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_distance() {
        let p1 = Node::new(3.0, 3.0);
        let p2 = Node::new(4.0, 4.0);
        let e12 = Edge::new(Rc::new(p1), Rc::new(p2));

        assert_eq!(1.4142135, e12.distance());
    }

    #[test]
    fn calculate_distance_regardless_of_order() {
        let p1 = Rc::new(Node::new(3.0, 3.0));
        let p2 = Rc::new(Node::new(4.0, 4.0));
        let e12 = Edge::new(Rc::clone(&p1), Rc::clone(&p2));
        let e21 = Edge::new(p2, p1);

        assert_eq!(e12.distance(), e21.distance());
    }
}

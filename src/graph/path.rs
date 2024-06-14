use super::NodeID;

#[derive(Debug, Clone)]
pub struct Path(pub Vec<NodeID>);

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        let len = self.0.len();
        if len != other.0.len() {
            return false;
        }

        for (i, _) in self.0.iter().enumerate() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl IntoIterator for Path {
    type Item = NodeID;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

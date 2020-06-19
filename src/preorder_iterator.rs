use crate::tree::Tree;
use crate::tree_node::{TreeIndex};

pub struct PreorderIterator {
    stack: Vec<TreeIndex>,
}

#[allow(dead_code)]
impl PreorderIterator {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreorderIterator { stack: vec![index] }
        } else {
            PreorderIterator { stack: vec![] }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop() {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(right) = node.right {
                    self.stack.push(right)
                }

                if let Some(left) = node.left {
                    self.stack.push(left)
                }

                return Some(node_index);
            }
        }
        None
    } // immutable borrow &Tree ends here
}

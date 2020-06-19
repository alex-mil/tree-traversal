use crate::preorder_iterator::PreorderIterator;
use crate::tree_node::TreeNode;

#[allow(dead_code)]
pub struct Tree {
    root: Option<TreeNode>,
}

#[allow(dead_code)]
impl Tree {
    pub fn new(root: Option<TreeNode>) -> Self {
        Tree { root }
    }

    pub fn iter(&self) -> PreorderIterator {
        PreorderIterator::new(self.root.as_ref())
    }
}

use crate::preorder_iterator::PreorderIterator;
use crate::tree_node::{TreeIndex, TreeNode};

#[allow(dead_code)]
pub struct Tree {
    arena: Vec<Option<TreeNode>>,
    root: Option<TreeIndex>,
}

#[allow(dead_code)]
impl Tree {
    pub fn new() -> Self {
        Tree {
            arena: vec![],
            root: None,
        }
    }

    pub fn iter(&self) -> PreorderIterator {
        PreorderIterator::new(self.root)
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root
    }

    pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        index
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        };
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }
}

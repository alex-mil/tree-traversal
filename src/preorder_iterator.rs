use crate::tree_node::TreeNode;

pub struct PreorderIterator<'a> {
    stack: Vec<&'a TreeNode>,
}

#[allow(dead_code)]
impl<'a> PreorderIterator<'a> {
    pub fn new(root: Option<&'a TreeNode>) -> Self {
        if let Some(node) = root {
            PreorderIterator { stack: vec![node] }
        } else {
            PreorderIterator { stack: vec![] }
        }
    }
}

#[allow(dead_code)]
impl<'a> Iterator for PreorderIterator<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &node.right {
                self.stack.push(&right)
            }

            if let Some(left) = &node.left {
                self.stack.push(&left)
            }

            return Some(node);
        }
        None
    }
}

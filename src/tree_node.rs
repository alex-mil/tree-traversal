pub struct TreeNode {
    pub value: usize,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new(value: usize, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { value, left, right }
    }
}

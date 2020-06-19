mod preorder_iterator;
mod tree;
mod tree_node;

#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::tree_node::TreeNode;

    #[test]
    fn graph_traversal() {
        let a = TreeNode::new(4, None, None);
        let b = TreeNode::new(5, None, None);
        let c = TreeNode::new(2, Some(Box::from(a)), Some(Box::from(b)));
        let d = TreeNode::new(3, None, None);
        let e = TreeNode::new(1, Some(Box::from(c)), Some(Box::from(d)));

        let tree = Tree::new(Some(e));

        for _node in tree.iter() {
            // _node.value *= 10;
        }

        let mut iterator = tree.iter();
        while let Some(node) = iterator.next() {
            println!("{}", node.value);
        }
    }
}

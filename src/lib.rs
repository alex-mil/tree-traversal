mod preorder_iterator;
mod tree;
mod tree_node;

#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::tree_node::TreeNode;

    #[test]
    fn graph_traversal() {
        let mut tree = Tree::new();

        let four = tree.add_node(TreeNode::new(4, None, None));
        let five = tree.add_node(TreeNode::new(5, None, None));
        let two = tree.add_node(TreeNode::new(2, Some(four), Some(five)));
        let three = tree.add_node(TreeNode::new(3, None, None));
        let one = tree.add_node(TreeNode::new(1, Some(two), Some(three)));

        tree.set_root(Some(one));

        let mut iterator = tree.iter();
        while let Some(i) = iterator.next(&tree) {
            let node = tree.node_at_mut(i).expect("node to exist at given index");
            node.value *= 10;
        } // mutable borrow ends here

        iterator = tree.iter();
        while let Some(i) = iterator.next(&tree) {
            let node = tree.node_at(i).expect("node to exist at given index");
            println!("{}", node.value)
        }
    }
}

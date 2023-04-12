// Return the maximum depth of a binary tree:
use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Node,
    right: Node,
}

fn max_depth(root: Node) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            1 + i32::max(max_depth(node_ref.left.clone()), max_depth(node_ref.right.clone()))
        }
        None => 0,
    }
}

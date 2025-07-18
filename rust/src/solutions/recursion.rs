use std::{cell::RefCell, rc::Rc};

use crate::data_structures::tree_node::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  fn is_valid_subtree(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    if root.is_none() {
      return true;
    }

    let root = root.unwrap().as_ref().as_ptr();
    let left = (unsafe { root.as_ref() }).unwrap().left.clone();
    let right = (unsafe { root.as_ref() }).unwrap().right.clone();
    let val = (unsafe { root.as_ref() }).unwrap().val;

    if max.is_some() && val >= max.unwrap() {
      return false;
    }

    if min.is_some() && val <= min.unwrap() {
      return false;
    }

    is_valid_subtree(left, min, Some(val)) && is_valid_subtree(right, Some(val), max)
  }

  is_valid_subtree(root, None, None)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_is_valid_bst() {
    use crate::data_structures::tree_node::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Helper to easily create a TreeNode wrapped in Rc<RefCell>
    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
      Rc::new(RefCell::new(TreeNode::new(val)))
    }

    // Valid BST:    2
    //              / \
    //             1   3
    let root = node(2);
    root.borrow_mut().left = Some(node(1));
    root.borrow_mut().right = Some(node(3));
    assert!(super::is_valid_bst(Some(root)));

    // Invalid BST:    5
    //                / \
    //               1   4
    //                  / \
    //                 3   6
    let root = node(5);
    root.borrow_mut().left = Some(node(1));
    let right = node(4);
    right.borrow_mut().left = Some(node(3));
    right.borrow_mut().right = Some(node(6));
    root.borrow_mut().right = Some(right);
    assert!(!super::is_valid_bst(Some(root)));

    // Single node (valid BST)
    let root = node(1);
    assert!(super::is_valid_bst(Some(root)));

    // Empty tree (valid BST)
    assert!(super::is_valid_bst(None));
  }

  #[test]
  fn test_invalid_bst_case() {
    use crate::data_structures::tree_node::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
      Rc::new(RefCell::new(TreeNode::new(val)))
    }

    // Build the tree: [5,4,6,null,null,3,7]
    let root = node(5);
    let left = node(4);
    let right = node(6);
    right.borrow_mut().left = Some(node(3));
    right.borrow_mut().right = Some(node(7));
    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    assert!(!super::is_valid_bst(Some(root)));
  }
}

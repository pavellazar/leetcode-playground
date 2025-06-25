use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::data_structures::tree_node::TreeNode;

// LeetCode #105 - Construct Binary Tree from Preorder and Inorder Traversal
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  fn helper(
    pre_start: usize,
    in_start: usize,
    in_end: usize,
    preorder: &Vec<i32>,
    inorder_map: &HashMap<i32, usize>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if pre_start >= preorder.len() || in_start > in_end {
      return None;
    }
    let root_val = preorder[pre_start];
    let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
    let in_root_idx = *inorder_map.get(&root_val).unwrap();
    let left_size = in_root_idx - in_start;

    root.borrow_mut().left = if in_root_idx > 0 {
      helper(
        pre_start + 1,
        in_start,
        in_root_idx - 1,
        preorder,
        inorder_map,
      )
    } else {
      None
    };
    root.borrow_mut().right = helper(
      pre_start + left_size + 1,
      in_root_idx + 1,
      in_end,
      preorder,
      inorder_map,
    );
    Some(root)
  }

  let inorder_map: HashMap<i32, usize> = inorder
    .iter()
    .enumerate()
    .map(|(i, &val)| (val, i))
    .collect();
  helper(0, 0, inorder.len() - 1, &preorder, &inorder_map)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_tree() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let tree = build_tree(preorder, inorder);

    assert!(tree.is_some());
    let root = tree.unwrap();
    assert_eq!(root.borrow().val, 3);
    assert!(root.borrow().left.is_some());
    assert!(root.borrow().right.is_some());
    assert_eq!(root.borrow().left.as_ref().unwrap().borrow().val, 9);
    assert_eq!(root.borrow().right.as_ref().unwrap().borrow().val, 20);
  }
}

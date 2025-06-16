use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

pub fn lowest_common_ancestor(
  root: Option<Rc<RefCell<TreeNode>>>,
  p: Option<Rc<RefCell<TreeNode>>>,
  q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
  fn helper(
    node: &Option<Rc<RefCell<TreeNode>>>,
    p: &Option<Rc<RefCell<TreeNode>>>,
    q: &Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if node.is_none() {
      return None;
    }
    let node_rc = node.as_ref().unwrap();
    if Rc::ptr_eq(node_rc, p.as_ref().unwrap()) || Rc::ptr_eq(node_rc, q.as_ref().unwrap()) {
      return Some(node_rc.clone());
    }
    let left = helper(&node_rc.borrow().left, p, q);
    let right = helper(&node_rc.borrow().right, p, q);

    match (left, right) {
      (Some(_), Some(_)) => Some(node_rc.clone()),
      (Some(l), None) => Some(l),
      (None, Some(r)) => Some(r),
      (None, None) => None,
    }
  }
  helper(&root, &p, &q)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::rc::Rc;

  fn build_tree() -> Option<Rc<RefCell<TreeNode>>> {
    // Manually build the tree [3,5,1,6,2,0,8,null,null,7,4]
    let n7 = Rc::new(RefCell::new(TreeNode {
      val: 7,
      left: None,
      right: None,
    }));
    let n4 = Rc::new(RefCell::new(TreeNode {
      val: 4,
      left: None,
      right: None,
    }));
    let n2 = Rc::new(RefCell::new(TreeNode {
      val: 2,
      left: Some(n7.clone()),
      right: Some(n4.clone()),
    }));
    let n6 = Rc::new(RefCell::new(TreeNode {
      val: 6,
      left: None,
      right: None,
    }));
    let n5 = Rc::new(RefCell::new(TreeNode {
      val: 5,
      left: Some(n6.clone()),
      right: Some(n2.clone()),
    }));
    let n0 = Rc::new(RefCell::new(TreeNode {
      val: 0,
      left: None,
      right: None,
    }));
    let n8 = Rc::new(RefCell::new(TreeNode {
      val: 8,
      left: None,
      right: None,
    }));
    let n1 = Rc::new(RefCell::new(TreeNode {
      val: 1,
      left: Some(n0.clone()),
      right: Some(n8.clone()),
    }));
    let root = Rc::new(RefCell::new(TreeNode {
      val: 3,
      left: Some(n5.clone()),
      right: Some(n1.clone()),
    }));
    Some(root)
  }

  #[test]
  fn test_lca_5_and_4() {
    let root = build_tree();
    // Find nodes 5 and 4
    let n5 = root
      .as_ref()
      .unwrap()
      .borrow()
      .left
      .as_ref()
      .unwrap()
      .clone();
    let n2 = n5.borrow().right.as_ref().unwrap().clone();
    let n4 = n2.borrow().right.as_ref().unwrap().clone();

    let lca = lowest_common_ancestor(root, Some(n5.clone()), Some(n4.clone()));
    assert_eq!(lca.unwrap().borrow().val, 5);
  }
}

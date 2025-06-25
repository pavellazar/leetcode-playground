use std::cell::RefCell;
use std::rc::Rc;

pub fn climb_stairs(n: i32) -> i32 {
  fn step(n: i32) -> i32 {
    if n == 0 || n == 1 {
      1
    } else {
      step(n - 1) + step(n - 2)
    }
  }
  step(n)
}

pub fn climb_stairs_math(n: i32) -> i32 {
  if n == 0 || n == 1 {
    return 1;
  }

  let mut a = 1; // f(0)
  let mut b = 1; // f(1)

  for _ in 2..=n {
    let temp = a + b;
    a = b;
    b = temp;
  }

  b
}

// LeetCode #198 - House Robber
pub fn rob(nums: Vec<i32>) -> i32 {
  if nums.is_empty() {
    return 0;
  }
  let mut amounts = vec![0; nums.len()];

  for i in 0..nums.len() {
    let prev2 = if i >= 2 { amounts[i - 2] } else { 0 };
    let prev1 = if i >= 1 { amounts[i - 1] } else { 0 };
    amounts[i] = (prev2 + nums[i]).max(prev1);
  }

  amounts[amounts.len() - 1]
}

// LeetCode #152 - Maximum Product Subarray
pub fn max_product(nums: Vec<i32>) -> i32 {
  if nums.is_empty() {
    return 0;
  }

  let mut max_prod = nums[0];
  let mut cur_max = nums[0];
  let mut cur_min = nums[0];

  for &n in nums.iter().skip(1) {
    let (tmp_max, tmp_min) = (cur_max, cur_min);
    cur_max = n.max(tmp_max * n).max(tmp_min * n);
    cur_min = n.min(tmp_max * n).min(tmp_min * n);
    max_prod = max_prod.max(cur_max);
  }
  max_prod
}

// LeetCode #300 - Longest Increasing Subsequence
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
  if nums.is_empty() {
    return 0;
  }
  let n = nums.len();
  let mut dp = vec![1; n];
  let mut max_len = 1;

  for i in 1..n {
    for j in 0..i {
      if nums[i] > nums[j] {
        dp[i] = dp[i].max(dp[j] + 1);
      }
    }
    max_len = max_len.max(dp[i]);
  }
  max_len
}

// LeetCode #416 - Partition Equal Subset Sum
pub fn can_partition(nums: Vec<i32>) -> bool {
  let total: i32 = nums.iter().sum();
  if total % 2 != 0 {
    return false;
  }
  let target = total / 2;
  let mut sums = std::collections::HashSet::new();
  sums.insert(0);

  for num in nums {
    let mut next = sums.clone();
    for &s in &sums {
      if s + num == target {
        return true;
      }
      next.insert(s + num);
    }
    sums = next;
  }
  sums.contains(&target)
}

// LeetCode #560 - Subarray Sum Equals K
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
  let mut count = 0;
  let mut prefix_sum = 0;
  let mut map = std::collections::HashMap::new();
  map.insert(0, 1);

  for num in nums {
    prefix_sum += num;
    if let Some(&c) = map.get(&(prefix_sum - k)) {
      count += c;
    }
    *map.entry(prefix_sum).or_insert(0) += 1;
  }

  count
}

// LeetCode #647 - Palindromic Substrings
pub fn count_substrings(s: String) -> i32 {
  let n = s.len();
  let mut count = 0;

  // Helper function to expand around center
  let mut expand_around_center = |left: usize, right: usize| {
    let mut l = left as isize;
    let mut r = right as isize;
    while l >= 0 && r < n as isize && s.as_bytes()[l as usize] == s.as_bytes()[r as usize] {
      count += 1;
      l -= 1;
      r += 1;
    }
  };

  for i in 0..n {
    // Odd length palindromes
    expand_around_center(i, i);
    // Even length palindromes
    if i + 1 < n {
      expand_around_center(i, i + 1);
    }
  }

  count
}

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

// LeetCode #337 - House Robber III
pub fn rob_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  fn rob_helper(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(n) = node {
      let n = n.borrow();
      let left = rob_helper(n.left.clone());
      let right = rob_helper(n.right.clone());

      // rob_this: rob current node, so skip children
      let rob_this = n.val + left.1 + right.1;
      // skip_this: take the best of robbing or not robbing each child
      let skip_this = left.0.max(left.1) + right.0.max(right.1);

      (rob_this, skip_this)
    } else {
      (0, 0)
    }
  }

  let (rob, skip) = rob_helper(root);
  rob.max(skip)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_subarray_sum() {
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    assert_eq!(subarray_sum(vec![1, 2, 3], 6), 1);
    assert_eq!(subarray_sum(vec![1, -1, 1], 0), 2);
    assert_eq!(subarray_sum(vec![0, 0, 0], 0), 6);
    assert_eq!(subarray_sum(vec![], 0), 0); // Edge case
    assert_eq!(subarray_sum(vec![5], 5), 1); // Edge case
    assert_eq!(subarray_sum(vec![5], 10), 0); // Edge case
    assert_eq!(subarray_sum(vec![10; 100], 10), 100); // Large input
  }

  #[test]
  fn test_can_partition() {
    assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    assert_eq!(can_partition(vec![1, 2, 3]), true);
    assert_eq!(can_partition(vec![]), true); // Edge case
    assert_eq!(can_partition(vec![1]), false); // Edge case
    assert_eq!(can_partition(vec![1, 1]), true); // Edge case
    assert_eq!(can_partition(vec![1, 2]), false); // Edge case
    assert_eq!(can_partition(vec![10; 100]), true); // Large input
  }

  #[test]
  fn test_rob() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    assert_eq!(rob(vec![0]), 0);
    assert_eq!(rob(vec![]), 0);
    assert_eq!(rob(vec![5]), 5);
    assert_eq!(rob(vec![1, 2]), 2);
    assert_eq!(rob(vec![1, 2, 3]), 4);
    assert_eq!(rob(vec![10, 9, 8]), 18);
    assert_eq!(rob(vec![10, 1, 10]), 20);
    assert_eq!(rob(vec![10, 9, 10]), 20);
    assert_eq!(rob(vec![10, 9, 8, 7]), 18);
    assert_eq!(rob(vec![10; 100]), (10 * (100 / 2)) as i32); // Large input
  }

  #[test]
  fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);
    assert_eq!(climb_stairs(6), 13);
    assert_eq!(climb_stairs_math(44), 1134903170);
  }

  #[test]
  fn test_max_product() {
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(max_product(vec![-2, 0, -1]), 0);
    assert_eq!(max_product(vec![-2, -3, -4]), 12);
    assert_eq!(max_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(max_product(vec![-1, -2, -3]), 6);
    assert_eq!(max_product(vec![0, 2]), 2);
    assert_eq!(max_product(vec![0, -1]), 0);
    assert_eq!(max_product(vec![1]), 1);
    assert_eq!(max_product(vec![]), 0); // Edge case
  }

  #[test]
  fn test_length_of_lis() {
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 3);
    assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 2);
    assert_eq!(length_of_lis(vec![7, 7, 7, 7]), 1);
    assert_eq!(length_of_lis(vec![]), 0); // Edge case
    assert_eq!(length_of_lis(vec![1]), 1);
    assert_eq!(length_of_lis(vec![10, 9]), 1);
    assert_eq!(length_of_lis(vec![1, 2, 3]), 3);
    assert_eq!(length_of_lis(vec![3, 2, 1]), 1);
    assert_eq!(length_of_lis(vec![1, 3, 2]), 2);
    assert_eq!(length_of_lis(vec![10; 100]), 1); // Large input
  }

  #[test]
  fn test_climb_stairs_math() {
    assert_eq!(climb_stairs_math(2), 2);
    assert_eq!(climb_stairs_math(3), 3);
    assert_eq!(climb_stairs_math(4), 5);
    assert_eq!(climb_stairs_math(5), 8);
    assert_eq!(climb_stairs_math(6), 13);
    assert_eq!(climb_stairs_math(44), 1134903170);
  }

  #[test]
  fn test_count_substrings() {
    // Placeholder for count_substrings tests
    // Implement the function and add tests as needed
    assert_eq!(count_substrings("abc".to_string()), 3); // "a", "b", "c"
    assert_eq!(count_substrings("aaa".to_string()), 6); // "a", "a", "a", "aa", "aa", "aaa"
    assert_eq!(count_substrings("".to_string()), 0); // Edge case
    assert_eq!(count_substrings("ab".to_string()), 2); // "a", "b"
  }

  #[test]
  fn test_rob_binary_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let right_left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let right_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    root.as_ref().unwrap().borrow_mut().left = left;
    root.as_ref().unwrap().borrow_mut().right = right;
    root
      .as_ref()
      .unwrap()
      .borrow_mut()
      .right
      .as_ref()
      .unwrap()
      .borrow_mut()
      .left = right_left;
    root
      .as_ref()
      .unwrap()
      .borrow_mut()
      .right
      .as_ref()
      .unwrap()
      .borrow_mut()
      .right = right_right;

    assert_eq!(rob_binary_tree(root), 7);

    // [4,1,null,2,null,3]
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let left2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left_left2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let left_left_left2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root2.as_ref().unwrap().borrow_mut().left = left2.clone();
    root2.as_ref().unwrap().borrow_mut().right = right2.clone();
    left2.as_ref().unwrap().borrow_mut().left = left_left2.clone();
    left_left2.as_ref().unwrap().borrow_mut().left = left_left_left2.clone();
    assert_eq!(rob_binary_tree(root2), 7);
  }
}

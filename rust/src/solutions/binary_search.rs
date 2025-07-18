// LeetCode #34 - Find First and Last Position of Element in Sorted Array
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
  fn binary_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> Option<usize> {
    if start > end || end >= nums.len() {
      return None;
    }

    let middle = (start + end) / 2;

    if nums[middle] == target {
      return Some(middle);
    }

    if target < nums[middle] {
      if middle == 0 {
        // Prevent underflow
        return None;
      }
      return binary_search(nums, target, start, middle - 1);
    }

    binary_search(nums, target, middle + 1, end)
  }

  let position = binary_search(&nums, target, 0, nums.len().saturating_sub(1));
  if position.is_none() {
    return vec![-1, -1];
  }

  let mut start = position.unwrap();
  let mut end = position.unwrap();

  while start > 0 && nums[start - 1] == target {
    start -= 1;
  }

  while end + 1 < nums.len() && nums[end + 1] == target {
    end += 1;
  }

  vec![start as i32, end as i32]
}

// LeetCode #153 - Find Minimum in Rotated Sorted Array
pub fn find_min(nums: Vec<i32>) -> i32 {
  // O(n) approach - iterate all elements, keep track of global min and return

  let mut start = 0;
  let mut end = nums.len() - 1;
  while start < end {
    let mid = (start + end) / 2;
    if nums[mid] > nums[end] {
      start = mid + 1;
    } else {
      end = mid;
    }
  }
  nums[start]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search_range() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(search_range(vec![1], 1), vec![0, 0]);
  }

  #[test]
  fn test_find_min() {
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
  }
}

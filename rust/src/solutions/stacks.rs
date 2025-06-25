// LeetCode #42 - Trapping Rain Water
pub fn trap(height: Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = height.len() - 1;
  let mut left_max = 0;
  let mut right_max = 0;
  let mut trapped = 0;

  while left < right {
    if height[left] < height[right] {
      if height[left] >= left_max {
        left_max = height[left];
      } else {
        trapped += left_max - height[left];
      }

      left += 1;
    } else {
      if height[right] >= right_max {
        right_max = height[right];
      } else {
        trapped += right_max - height[right];
      }

      right -= 1;
    }
  }

  trapped
}

// LeetCode #496 - Next Greater Element I
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  use std::collections::HashMap;
  let mut stack = Vec::new();
  let mut next_greater = HashMap::new();

  // Build the next greater map for nums2 using a monotonic decreasing stack
  for &num in nums2.iter() {
    while let Some(&top) = stack.last() {
      if num > top {
        next_greater.insert(stack.pop().unwrap(), num);
      } else {
        break;
      }
    }
    stack.push(num);
  }
  // For elements left in the stack, there is no next greater element
  for num in stack {
    next_greater.insert(num, -1);
  }

  // Build the result for nums1
  nums1
    .iter()
    .map(|&num| *next_greater.get(&num).unwrap_or(&-1))
    .collect()
}

// LeetCode #739 - Daily Temperatures
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
  let mut stack = Vec::new();
  let mut days = std::collections::HashMap::new();

  for i in 0..temperatures.len() {
    while let Some(&(value, idx)) = stack.last() {
      if temperatures[i] > value {
        stack.pop();
        days.insert(idx, i - idx);
      } else {
        break;
      }
    }

    stack.push((temperatures[i], i));
  }

  for pair in stack {
    days.insert(pair.1, 0);
  }

  let mut ans = vec![0; temperatures.len()];
  for i in 0..temperatures.len() {
    ans[i] = *days.get(&i).unwrap() as i32;
  }

  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_trap() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    assert_eq!(trap(vec![4, 0, 2, 1, 4]), 9);
  }

  #[test]
  fn test_next_greater_element() {
    assert_eq!(
      next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
      vec![-1, 3, -1]
    );
    assert_eq!(
      next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
      vec![3, -1]
    );
  }

  #[test]
  fn test_daily_temperatures() {
    assert_eq!(
      daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
      vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
  }
}

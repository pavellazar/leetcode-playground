use std::collections::HashMap;

pub fn two_sum(array: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::new();

  for (i, &num) in array.iter().enumerate() {
    match map.get(&(target - num)) {
      Some(&index) => return vec![index as i32, i as i32],
      None => map.insert(num, i),
    };
  }

  vec![]
}

pub fn container(height: Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = height.len() - 1;

  let mut max_area = 0;

  while left < right {
    let width = right - left;
    let area = width * height[right].min(height[left]) as usize;
    if area > max_area {
      max_area = area;
    }

    if height[right] > height[left] {
      left += 1;
    } else {
      right -= 1;
    }
  }

  max_area as i32
}

pub fn three_zero_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
  return three_sum(nums, 0);
}

fn three_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut nums = nums;
  nums.sort_unstable();
  let mut result = Vec::new();

  for i in 0..nums.len() {
    if i > 0 && nums[i] == nums[i - 1] {
      continue;
    }

    let mut left = i + 1;
    let mut right = nums.len() - 1;

    while left < right {
      let sum = nums[i] + nums[left] + nums[right];
      if sum == target {
        result.push(vec![nums[i], nums[left], nums[right]]);
        while left < right && nums[left] == nums[left + 1] {
          left += 1;
        }
        while left < right && nums[right] == nums[right - 1] {
          right -= 1;
        }
        left += 1;
        right -= 1;
      } else if sum < target {
        left += 1;
      } else {
        right -= 1;
      }
    }
  }

  result
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  if nums.is_empty() {
    return 0;
  }

  let mut unique_count = 1;

  for i in 1..nums.len() {
    if nums[i] != nums[unique_count - 1] {
      nums[unique_count] = nums[i];
      unique_count += 1;
    }
  }

  nums.truncate(unique_count);
  unique_count as i32
}

pub fn rotated_array_search(nums: Vec<i32>, target: i32) -> i32 {
  let mut left: usize = 0;
  let mut right: usize = nums.len() - 1;

  while left <= right {
    let mid = left + (right - left) / 2;

    if nums[mid] == target {
      return mid as i32;
    }

    if nums[left] <= nums[mid] {
      if nums[left] <= target && target < nums[mid] {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    } else {
      if nums[mid] < target && target <= nums[right] {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
  }

  -1
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
  fn backtrack(
    nums: &Vec<i32>,
    used: &mut Vec<bool>,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
  ) {
    if current.len() == nums.len() {
      result.push(current.clone());
      return;
    }
    for i in 0..nums.len() {
      if used[i] {
        continue;
      }
      used[i] = true;
      current.push(nums[i]);
      backtrack(nums, used, current, result);
      current.pop();
      used[i] = false;
    }
  }
  let mut result = Vec::new();
  let mut current = Vec::new();
  let mut used = vec![false; nums.len()];
  backtrack(&nums, &mut used, &mut current, &mut result);
  result
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
  let mut global = nums[0];
  let mut current = nums[0];

  for i in 1..nums.len() {
    current = nums[i].max(current + nums[i]);
    if current > global {
      global = current
    }
  }

  global
}

pub fn find_missing_number(nums: Vec<i32>) -> usize {
  let mut left = 0;
  let mut right = nums.len();

  while left < right {
    let mid = left + (right - left) / 2;
    if nums[mid] as usize > mid {
      right = mid;
    } else {
      left = mid + 1;
    }
  }
  left
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
  let mut dp = vec![amount + 1; (amount + 1) as usize];
  dp[0] = 0;
  for a in 1..=amount {
    for &coin in &coins {
      if coin <= a {
        dp[a as usize] = dp[a as usize].min(1 + dp[(a - coin) as usize]);
      }
    }
  }
  if dp[amount as usize] > amount {
    -1
  } else {
    dp[amount as usize]
  }
}

pub fn coin_change_combinations(coins: Vec<i32>, amount: i32) -> i32 {
  let mut dp = vec![0; (amount + 1) as usize];
  dp[0] = 1;
  for &coin in &coins {
    for a in coin..=amount {
      dp[a as usize] += dp[(a - coin) as usize];
    }
  }
  dp[amount as usize]
}

pub fn can_jump(nums: Vec<i32>) -> bool {
  fn helper(nums: &Vec<i32>, pos: usize, memo: &mut Vec<Option<bool>>) -> bool {
    if pos >= nums.len() {
      return false;
    }

    if pos == nums.len() - 1 {
      return true;
    }

    if let Some(res) = memo[pos] {
      return res;
    }

    let jumps = nums[pos] as usize;
    for i in 1..=jumps {
      if helper(nums, pos + i, memo) {
        memo[pos] = Some(true);
        return true;
      }
    }
    memo[pos] = Some(false);
    return false;
  }

  let mut memo = vec![None; nums.len()];
  helper(&nums, 0, &mut memo)
}

pub fn optimized_can_jump(nums: Vec<i32>) -> bool {
  let mut last_pos = nums.len() - 1;
  for i in (0..nums.len()).rev() {
    if i + nums[i] as usize >= last_pos {
      last_pos = i;
    }
  }
  last_pos == 0
}

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut sorted = intervals.clone();
  sorted.sort();

  let mut merged = vec![sorted[0].clone()];
  for i in 1..sorted.len() {
    let last = merged.last_mut().unwrap();
    if last[1] >= sorted[i][0] {
      last[1] = last[1].max(sorted[i][1]);
    } else {
      merged.push(sorted[i].clone());
    }
  }

  merged
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_intervals() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let merged = merge_intervals(intervals);
    assert_eq!(merged, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
  }

  #[test]
  fn test_can_jump() {
    assert!(!can_jump(vec![
      2, 0, 6, 9, 8, 4, 5, 0, 8, 9, 1, 2, 9, 6, 8, 8, 0, 6, 3, 1, 2, 2, 1, 2, 6, 5, 3, 1, 2, 2, 6,
      4, 2, 4, 3, 0, 0, 0, 3, 8, 2, 4, 0, 1, 2, 0, 1, 4, 6, 5, 8, 0, 7, 9, 3, 4, 6, 6, 5, 8, 9, 3,
      4, 3, 7, 0, 4, 9, 0, 9, 8, 4, 3, 0, 7, 7, 1, 9, 1, 9, 4, 9, 0, 1, 9, 5, 7, 7, 1, 5, 8, 2, 8,
      2, 6, 8, 2, 2, 7, 5, 1, 7, 9, 6
    ]));
    assert!(can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    assert!(can_jump(vec![0]));
    assert!(can_jump(vec![2, 0]));
    assert!(can_jump(vec![1, 2, 3]));
    assert!(!can_jump(vec![1, 0, 0, 0]));
  }

  #[test]
  fn test_coin_change() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
    assert_eq!(coin_change(vec![1], 0), 0);
    assert_eq!(coin_change(vec![1], 2), 2);
    assert_eq!(coin_change(vec![1, 2, 5], 100), 20);
  }

  #[test]
  fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
  }

  #[test]
  fn test_container() {
    assert_eq!(49, container(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
  }

  #[test]
  fn test_three_zero_sum() {
    assert_eq!(
      three_zero_sum(vec![-1, 0, 1, 2, -1, -4]),
      vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(three_zero_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(three_zero_sum(vec![]), Vec::<Vec<i32>>::new());
  }

  #[test]
  fn test_remove_duplicates() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums), 5);
    assert_eq!(nums[..5], [0, 1, 2, 3, 4]);
  }

  #[test]
  fn test_rotated_array_search() {
    assert_eq!(rotated_array_search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(rotated_array_search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(rotated_array_search(vec![1], 0), -1);
  }

  #[test]
  fn test_permute() {
    let result = permute(vec![1, 2, 3]);
    assert_eq!(result.len(), 6);
    assert!(result.contains(&vec![1, 2, 3]));
    assert!(result.contains(&vec![1, 3, 2]));
    assert!(result.contains(&vec![2, 1, 3]));
    assert!(result.contains(&vec![2, 3, 1]));
    assert!(result.contains(&vec![3, 1, 2]));
    assert!(result.contains(&vec![3, 2, 1]));
  }

  #[test]
  fn test_max_sub_array() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(vec![1]), 1);
    assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(max_sub_array(vec![-2, -3, -1]), -1);
  }

  #[test]
  fn test_find_missing_number() {
    assert_eq!(find_missing_number(vec![0, 1, 2, 3, 5]), 4);
    assert_eq!(find_missing_number(vec![0, 1, 3]), 2);
    assert_eq!(find_missing_number(vec![1, 2, 3]), 0);
    assert_eq!(find_missing_number(vec![0]), 1);
    assert_eq!(find_missing_number(vec![]), 0);
  }
}

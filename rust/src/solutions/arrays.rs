// HashSet and HashMap provide average-case O(1) insert and lookup
use std::collections::HashMap;
use std::collections::HashSet;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let count = nums1.len() + nums2.len();
  let mut median_indexes = vec![];

  if count % 2 == 1 {
    median_indexes.push(count / 2);
  } else {
    median_indexes.push(count / 2 - 1);
    median_indexes.push(count / 2);
  }

  let (mut left, mut right) = (0, 0);
  let mut merged = vec![];

  while left < nums1.len() && right < nums2.len() {
    if nums1[left] < nums2[right] {
      merged.push(nums1[left]);
      left += 1;
    } else {
      merged.push(nums2[right]);
      right += 1;
    }
  }

  if left < nums1.len() {
    merged.extend_from_slice(&nums1[left..]);
  }

  if right < nums2.len() {
    merged.extend_from_slice(&nums2[right..]);
  }

  let mut median = 0;
  for i in 0..median_indexes.len() {
    median += merged[median_indexes[i]];
  }

  median as f64 / median_indexes.len() as f64
}

// LeetCode #238 - Product of Array Except Self
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
  let mut result = nums.clone();
  let mut product = 1;
  let mut zeros = 0;

  for n in nums {
    if n == 0 {
      zeros += 1;
    } else {
      product *= n;
    }
  }

  for i in 0..result.len() {
    if zeros > 1 {
      // it's always 0 in this case
      result[i] = 0;
    } else if zeros == 1 {
      if result[i] == 0 {
        // this is the only 0
        result[i] = product;
      } else {
        result[i] = 0;
      }
    } else {
      result[i] = product / result[i];
    }
  }

  result
}

// LeetCode #238 - Product of Array Except Self without division
pub fn product_except_self_no_division(nums: Vec<i32>) -> Vec<i32> {
  let n = nums.len();
  let mut result = vec![1; n];

  // Prefix products
  let mut prefix = 1;
  for i in 0..n {
    result[i] = prefix;
    prefix *= nums[i];
  }

  // Suffix products
  let mut suffix = 1;
  for i in (0..n).rev() {
    result[i] *= suffix;
    suffix *= nums[i];
  }

  result
}

// LeetCode #283 - Move Zeroes
pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut current = 0;
  let mut fill = 0;

  while current < nums.len() {
    if nums[current] != 0 {
      if fill != current {
        nums[fill] = nums[current];
      }
      current += 1;
      fill += 1;
    } else {
      current += 1;
    }
  }

  for i in fill..nums.len() {
    nums[i] = 0;
  }
}

// LeetCode #128 - Longest Consecutive Sequence
pub fn longest_consecutive_sequence(array: Vec<i32>) -> i32 {
  let set: HashSet<i32> = array.iter().cloned().collect();
  let mut max_len = 0;

  for &num in &set {
    // Only start counting if num-1 is not in the set (start of a sequence)
    if !set.contains(&(num - 1)) {
      let mut current = num;
      let mut length = 1;
      while set.contains(&(current + 1)) {
        current += 1;
        length += 1;
      }
      max_len = max_len.max(length);
    }
  }

  max_len
}

// LeetCode #1 - Two Sum
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

// LeetCode #11 - Container With Most Water
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

// LeetCode #15 - 3Sum
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

// LeetCode #16 - 3Sum Closest
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
  let mut nums = nums;
  nums.sort_unstable();
  let mut closest = nums[0] + nums[1] + nums[2];

  for i in 0..nums.len() {
    if i > 0 && nums[i] == nums[i - 1] {
      continue;
    }

    let mut left = i + 1;
    let mut right = nums.len() - 1;

    while left < right {
      let sum = nums[i] + nums[left] + nums[right];
      if (sum - target).abs() < (closest - target).abs() {
        closest = sum;
      }
      if sum < target {
        left += 1;
      } else {
        right -= 1;
      }
    }
  }

  closest
}

// LeetCode #26 - Remove Duplicates from Sorted Array
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

// LeetCode #33 - Search in Rotated Sorted Array
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

// LeetCode #46 - Permutations
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

// LeetCode #53 - Maximum Subarray
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

// LeetCode #268 - Missing Number
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

// LeetCode #322 - Coin Change
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

// LeetCode #518 - Coin Change II
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

// LeetCode #55 - Jump Game
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

// LeetCode #56 - Merge Intervals
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

// LeetCode #287 - Find the Duplicate Number
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
  let n = nums.len() + 1;
  let mut counter = vec![0; n];

  for n in nums {
    if counter[n as usize] == 1 {
      return n;
    }

    counter[n as usize] = 1;
  }

  0
}

// LeetCode #349 - Intersection of Two Arrays
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  let mut map = HashMap::new();

  for n in nums1 {
    map.entry(n).or_insert(true);
  }

  let mut result = vec![];
  for n in nums2 {
    match map.get_mut(&n) {
      Some(val) => {
        if *val {
          result.push(n);
        }
        *val = false;
      }
      _ => {}
    }
  }

  result
}

// LeetCode #344 - Reverse String
pub fn reverse_string(s: &mut Vec<char>) {
  let mut left = 0;
  let mut right = s.len() - 1;

  while left < right {
    let c = s[left];
    s[left] = s[right];
    s[right] = c;

    left += 1;
    right -= 1;
  }
}

// LeetCode #189 - Rotate Array
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
  let n = nums.len();
  if n == 0 {
    return;
  }
  let k = (k as usize) % n;
  if k == 0 {
    return;
  }

  let tmp = nums[n - k..].to_vec();
  for i in (0..n - k).rev() {
    nums[i + k] = nums[i];
  }

  for i in 0..k {
    nums[i] = tmp[i];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rotate_array() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);

    let mut nums = vec![1];
    rotate(&mut nums, 0);
    assert_eq!(nums, vec![1]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![2, 1]);
  }

  #[test]
  fn test_reverse_string() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
  }

  #[test]
  fn test_intersection() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    assert_eq!(intersection(nums1, nums2), vec![2]);

    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    assert_eq!(intersection(nums1, nums2), vec![9, 4]);

    let nums1 = vec![1, 2, 3];
    let nums2 = vec![4, 5, 6];
    assert_eq!(intersection(nums1, nums2), vec![]);
  }

  #[test]
  fn test_find_duplicate() {
    let nums = vec![1, 3, 4, 2, 2];
    assert_eq!(find_duplicate(nums), 2);

    let nums = vec![3, 1, 3, 4, 2];
    assert_eq!(find_duplicate(nums), 3);

    let nums = vec![1, 1];
    assert_eq!(find_duplicate(nums), 1);

    let nums = vec![1, 2, 3, 4, 5, 5];
    assert_eq!(find_duplicate(nums), 5);
  }

  #[test]
  fn test_product_except_self() {
    let nums = vec![1, 2, 3, 4];
    let result = product_except_self(nums);
    assert_eq!(result, vec![24, 12, 8, 6]);

    let nums = vec![0, 1];
    let result = product_except_self(nums);
    assert_eq!(result, vec![1, 0]);

    let nums = vec![1];
    let result = product_except_self(nums);
    assert_eq!(result, vec![1]);
  }

  #[test]
  fn test_move_zeros() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);

    let mut nums = vec![0, 0, 1];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 0, 0]);

    let mut nums = vec![1, 2, 3];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 2, 3]);
  }

  #[test]
  fn test_longest_consecutive_sequence() {
    assert_eq!(longest_consecutive_sequence(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
      longest_consecutive_sequence(vec![0, 3, 7, 2, 5, 8, 4, 6, 1]),
      9
    );
    assert_eq!(longest_consecutive_sequence(vec![1, 2, 0, 1]), 3);
    assert_eq!(longest_consecutive_sequence(vec![]), 0);
    assert_eq!(longest_consecutive_sequence(vec![1]), 1);
  }

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

  #[test]
  fn test_find_median_in_sorted_arrays() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);

    let nums1 = vec![0, 0];
    let nums2 = vec![0, 0];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);

    let nums1 = vec![];
    let nums2 = vec![1];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);
  }

  #[test]
  fn test_three_sum_closest() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    assert_eq!(three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    assert_eq!(three_sum_closest(vec![1, 1, -1, -1, 3], -1), -1);
    assert_eq!(three_sum_closest(vec![1, 2, 5, 10, 11], 12), 13);
    assert_eq!(three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    assert_eq!(three_sum_closest(vec![1, 1, 1, 1], 3), 3);
  }
}

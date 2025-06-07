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
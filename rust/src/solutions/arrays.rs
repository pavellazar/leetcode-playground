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

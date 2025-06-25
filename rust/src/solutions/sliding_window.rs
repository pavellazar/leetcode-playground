use std::collections::HashMap;
use std::collections::VecDeque;

use crate::common::string_utils::signature;

// LeetCode #438. Find All Anagrams in a String
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
  if p.len() > s.len() {
    return vec![];
  }

  let p_hash = signature(&p);
  let mut slide_hash = signature(&s[0..p.len()]);
  let mut result = vec![];

  if p_hash == slide_hash {
    result.push(0);
  }

  for i in 1..=s.len() - p.len() {
    let sub_byte = s.as_bytes()[i - 1] - b'A';
    let add_byte = s.as_bytes()[i + p.len() - 1] - b'A';

    slide_hash[sub_byte as usize] -= 1;
    slide_hash[add_byte as usize] += 1;

    if p_hash == slide_hash {
      result.push(i as i32);
    }
  }

  result
}

// LeetCode #76. Minimum Window Substring
pub fn min_window_substring(s: String, t: String) -> String {
  if t.is_empty() || s.is_empty() {
    return "".to_string();
  }

  let mut t_count = HashMap::new();
  for c in t.chars() {
    *t_count.entry(c).or_insert(0) += 1;
  }

  let mut window_count = HashMap::new();
  let (mut left, mut right) = (0, 0);
  let (mut formed, required) = (0, t_count.len());
  let s_chars: Vec<char> = s.chars().collect();
  let mut ans = (usize::MAX, 0, 0);

  while right < s_chars.len() {
    let c = s_chars[right];
    *window_count.entry(c).or_insert(0) += 1;

    if t_count.contains_key(&c) && window_count[&c] == t_count[&c] {
      formed += 1;
    }

    while left <= right && formed == required {
      if right - left + 1 < ans.0 {
        ans = (right - left + 1, left, right);
      }
      let c = s_chars[left];
      *window_count.entry(c).or_insert(0) -= 1;
      if t_count.contains_key(&c) && window_count[&c] < t_count[&c] {
        formed -= 1;
      }
      left += 1;
    }
    right += 1;
  }

  if ans.0 == usize::MAX {
    "".to_string()
  } else {
    s_chars[ans.1..=ans.2].iter().collect()
  }
}

// LeetCode #239. Sliding Window Maximum
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
  let mut result = Vec::new();
  let mut deque = VecDeque::new();

  for i in 0..nums.len() {
    // Remove indices out of window
    if let Some(&front) = deque.front() {
      if front + k as usize <= i {
        deque.pop_front();
      }
    }
    // Remove smaller values from the back
    while let Some(&back) = deque.back() {
      if nums[back] < nums[i] {
        deque.pop_back();
      } else {
        break;
      }
    }
    deque.push_back(i);

    // The front is the max for the window
    if i + 1 >= k as usize {
      result.push(nums[*deque.front().unwrap()]);
    }
  }
  result
}

// LeetCode #424 - Longest Repeating Character Replacement
pub fn character_replacement(s: String, k: i32) -> i32 {
  let k = k as usize;
  let chars: Vec<char> = s.chars().collect();

  let mut max = 0;
  let mut map = HashMap::new();
  let (mut left, mut right) = (0, 0);
  let mut max_count = 0;

  while right < chars.len() {
    let count = map.entry(chars[right]).or_insert(0);
    *count += 1;
    max_count = max_count.max(*count);

    while right - left + 1 - max_count > k {
      let left_count = map.get_mut(&chars[left]).unwrap();
      *left_count -= 1;
      left += 1;
    }

    max = max.max(right - left + 1);
    right += 1;
  }

  max as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_character_replacement() {
    assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    assert_eq!(character_replacement("A".to_string(), 0), 1);
    assert_eq!(character_replacement("AA".to_string(), 1), 2);
    assert_eq!(character_replacement("AABBA".to_string(), 1), 3);
    assert_eq!(character_replacement("AABBA".to_string(), 2), 5);
  }

  #[test]
  fn test_max_sliding_window() {
    assert_eq!(
      max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
      vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(max_sliding_window(vec![], 0), vec![]);
    assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    assert_eq!(max_sliding_window(vec![9, 11], 2), vec![11]);
    assert_eq!(max_sliding_window(vec![4, -2], 2), vec![4]);
  }

  #[test]
  fn test_min_window_substring() {
    assert_eq!(
      min_window_substring("ADOBECODEBANC".to_string(), "ABC".to_string()),
      "BANC".to_string()
    );
    assert_eq!(
      min_window_substring("a".to_string(), "a".to_string()),
      "a".to_string()
    );
    assert_eq!(
      min_window_substring("a".to_string(), "aa".to_string()),
      "".to_string()
    );
  }

  #[test]
  fn test_find_anagrams() {
    assert_eq!(find_anagrams("af".to_string(), "be".to_string()), vec![]);

    assert_eq!(
      find_anagrams("abab".to_string(), "ab".to_string()),
      vec![0, 1, 2]
    );
    assert_eq!(
      find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
      vec![0, 6]
    );
  }
}

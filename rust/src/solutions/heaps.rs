use std::collections::BinaryHeap;

// LeetCode #215 - Kth Largest Element in an Array
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
  let mut heap = BinaryHeap::new();
  for num in nums.iter() {
    heap.push(*num);
  }

  for _ in 0..k - 1 {
    heap.pop();
  }

  *heap.peek().unwrap()
}

// LeetCode #347 - Top K Frequent Elements
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
  use std::cmp::Reverse;
  use std::collections::{BinaryHeap, HashMap};

  let mut freq = HashMap::new();
  for num in nums {
    *freq.entry(num).or_insert(0) += 1;
  }

  // Min-heap of (frequency, number)
  let mut heap = BinaryHeap::new();
  for (&num, &count) in freq.iter() {
    heap.push(Reverse((count, num)));
    if heap.len() > k as usize {
      heap.pop();
    }
  }

  heap.into_iter().map(|Reverse((_count, num))| num).collect()
}

// LeetCode #621 - Task Scheduler
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
  use std::collections::{BinaryHeap, HashMap, VecDeque};

  // Step 1: Count frequencies
  let mut freq = HashMap::new();
  for &task in &tasks {
    *freq.entry(task).or_insert(0) += 1;
  }

  // Step 2: Max-heap for available tasks (by count)
  let mut heap = BinaryHeap::new();
  for &count in freq.values() {
    heap.push(count);
  }

  // Step 3: Queue for cooldown (count, available_time)
  let mut cooldown = VecDeque::new();
  let mut time = 0;

  while !heap.is_empty() || !cooldown.is_empty() {
    time += 1;

    // If a task's cooldown is over, push it back to heap
    if let Some(&(ready_count, ready_time)) = cooldown.front() {
      if ready_time == time {
        cooldown.pop_front();
        heap.push(ready_count);
      }
    }

    // Schedule the most frequent available task
    if let Some(mut count) = heap.pop() {
      count -= 1;
      if count > 0 {
        cooldown.push_back((count, time + n + 1));
      }
    }
    // else: idle (do nothing, just increment time)
  }

  time
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_kth_largest() {
    assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    assert_eq!(find_kth_largest(vec![1], 1), 1);
  }

  #[test]
  fn test_top_k_frequent() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![2, 1]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
  }

  #[test]
  fn test_least_interval() {
    // Placeholder for future tests
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
    assert_eq!(least_interval(vec!['A', 'B'], 2), 2);
  }
}

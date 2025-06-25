// LeetCode #75 - Sort Colors
pub fn sort_colors(nums: &mut Vec<i32>) {
  let mut counter = vec![0; 3];
  for &i in nums.iter() {
    counter[i as usize] += 1;
  }

  let mut start = 0;
  for c in 0..counter.len() {
    for i in 0..counter[c] {
      nums[start + i] = c as i32;
    }

    start += counter[c];
  }
}

// LeetCode #2402 - Meeting Rooms III
pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
  let n = n as usize;
  let mut meetings = meetings;
  meetings.sort();

  // Min-heap for available rooms (by index)
  let mut available: std::collections::BinaryHeap<std::cmp::Reverse<usize>> =
    (0..n).map(std::cmp::Reverse).collect();

  // Min-heap for ongoing meetings: (end_time, room_index)
  let mut ongoing: std::collections::BinaryHeap<std::cmp::Reverse<(i64, usize)>> =
    std::collections::BinaryHeap::new();

  let mut room_count = vec![0; n];

  for meeting in meetings {
    let start = meeting[0] as i64;
    let end = meeting[1] as i64;

    // Free up rooms that have finished before this meeting starts
    while let Some(&std::cmp::Reverse((end_time, room_idx))) = ongoing.peek() {
      if end_time <= start {
        ongoing.pop();
        available.push(std::cmp::Reverse(room_idx));
      } else {
        break;
      }
    }

    if let Some(std::cmp::Reverse(room_idx)) = available.pop() {
      // Assign to available room
      ongoing.push(std::cmp::Reverse((end, room_idx)));
      room_count[room_idx] += 1;
    } else {
      // All rooms busy: pick the one that gets free the earliest (and smallest index)
      let std::cmp::Reverse((end_time, room_idx)) = ongoing.pop().unwrap();
      ongoing.push(std::cmp::Reverse((end_time + (end - start), room_idx)));
      room_count[room_idx] += 1;
    }
  }

  // Find the room with the most meetings (smallest index in case of tie)
  let mut max_meetings = 0;
  let mut result = 0;
  for (i, &count) in room_count.iter().enumerate() {
    if count > max_meetings {
      max_meetings = count;
      result = i;
    }
  }
  result as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sort_colors() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
  }

  #[test]
  fn test_most_booked() {
    let meetings = vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];
    assert_eq!(most_booked(3, meetings), 1);
  }

  #[test]
  fn test_most_booked_edge_case() {
    let n = 2;
    let meetings = vec![
      vec![1, 10],
      vec![2, 10],
      vec![3, 10],
      vec![4, 10],
      vec![5, 10],
      vec![6, 10],
      vec![7, 10],
    ];
    assert_eq!(most_booked(n, meetings), 0);
  }
}

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::data_structures::list_node::ListNode;

pub fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let rev_l1 = reverse_list(l1);
  let rev_l2 = reverse_list(l2);
  
  let mut dummy = Box::new(ListNode::new(0));
  let mut tail = &mut dummy;
  let mut carry = 0;
  let mut l1 = rev_l1;
  let mut l2 = rev_l2;

  while l1.is_some() || l2.is_some() || carry > 0 {
    let val1 = l1.as_ref().map_or(0, |node| node.val);
    let val2 = l2.as_ref().map_or(0, |node| node.val);

    let sum = val1 + val2 + carry;
    carry = sum / 10;
    tail.next = Some(Box::new(ListNode::new(sum % 10)));
    tail = tail.next.as_mut().unwrap();

    l1 = l1.and_then(|node| node.next);
    l2 = l2.and_then(|node| node.next);
  }

  reverse_list(dummy.next)
}

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  if head.is_none() || head.as_ref().unwrap().next.is_none() {
    return head;
  }

  let mut odd_head = None;
  let mut even_head = None;
  let mut odd_tail = &mut odd_head;
  let mut even_tail = &mut even_head;

  let mut current = head;
  let mut is_odd = true;

  while let Some(mut node) = current {
    current = node.next.take();
    if is_odd {
      odd_tail = &mut odd_tail.insert(node).next;
    } else {
      even_tail = &mut even_tail.insert(node).next;
    }
    is_odd = !is_odd;
  }

  // Connect odd list to even list
  *odd_tail = even_head;
  odd_head
}

pub fn is_palindrome_optimized(head: Option<Box<ListNode>>) -> bool {
  fn check(node: &Option<Box<ListNode>>, front: &mut Option<Box<ListNode>>) -> bool {
    match node {
      None => true,
      Some(n) => {
        check(&n.next, front) && {
          let result = front.as_ref().unwrap().val == n.val;
          *front = front.as_mut().unwrap().next.take();
          result
        }
      }
    }
  }

  let mut front = head.clone();
  check(&head, &mut front)
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
  let mut head = head;
  let mut data = vec![];

  while let Some(node) = head {
    data.push(node.val);
    head = node.next;
  }

  let mut left = 0;
  let mut right = data.len() - 1;

  while left < right {
    if data[left] != data[right] {
      return false;
    }

    left += 1;
    right -= 1;
  }

  true
}

pub fn get_intersection_node(
  head_a: Option<Box<ListNode>>,
  head_b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut a = head_a.clone();
  let mut b = head_b.clone();
  while a != b {
    a = if let Some(node) = a {
      node.next
    } else {
      head_b.clone()
    };
    b = if let Some(node) = b {
      node.next
    } else {
      head_a.clone()
    };
  }
  a
}

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  if head.is_none() || head.as_ref().unwrap().next.is_none() {
    return head;
  }
  let (left, right) = split(head);
  let left = sort_list(left);
  let right = sort_list(right);
  merge_two_lists(left, right)
}

fn get_length(head: &Option<Box<ListNode>>) -> usize {
  let mut res = 0;
  let mut current_node = head;
  while current_node.is_some() {
    current_node = &current_node.as_ref().unwrap().next;
    res += 1;
  }
  res
}

fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
  let len = get_length(&head) / 2;
  let mut curr = &mut head;
  for _ in 0..len {
    let curr_inner = curr.as_mut().unwrap();
    curr = &mut curr_inner.next;
  }
  let tail = curr.take();
  (head, tail)
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
  let mut dummy = Box::new(ListNode { val: 0, next: head });
  let mut fast: *mut Box<ListNode> = &mut dummy;
  let mut slow: *mut Box<ListNode> = &mut dummy;

  // Move fast n steps ahead
  for _ in 0..n {
    unsafe {
      fast = match (*fast).next.as_mut() {
        Some(node) => node,
        None => return dummy.next,
      };
    }
  }

  // Move both pointers until fast reaches the end
  unsafe {
    while (*fast).next.is_some() {
      fast = (*fast).next.as_mut().unwrap();
      slow = (*slow).next.as_mut().unwrap();
    }
    // Remove the nth node from the end
    (*slow).next = (*slow).next.as_mut().and_then(|node| node.next.take());
  }

  dummy.next
}

pub fn merge_two_lists(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut left = list1;
  let mut right = list2;

  let mut dummy = Box::new(ListNode::new(0));
  let mut tail = &mut dummy;

  while left.is_some() && right.is_some() {
    let l_val = left.as_ref().unwrap().val;
    let r_val = right.as_ref().unwrap().val;
    if l_val < r_val {
      let next = left.as_mut().unwrap().next.take();
      tail.next = left;
      left = next;
    } else {
      let next = right.as_mut().unwrap().next.take();
      tail.next = right;
      right = next;
    }
    tail = tail.next.as_mut().unwrap();
  }

  tail.next = if left.is_some() { left } else { right };
  dummy.next
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
  let mut lists = lists;
  let mut heap = BinaryHeap::new();
  for (i, node) in lists.iter_mut().enumerate() {
    if let Some(n) = node.as_mut() {
      heap.push(Reverse((n.val, i)));
    }
  }

  let mut dummy = Box::new(ListNode::new(0));
  let mut tail = &mut dummy;

  while let Some(Reverse((_, i))) = heap.pop() {
    let next = lists[i].as_mut().unwrap().next.take();

    tail.next = lists[i].take();
    tail = tail.next.as_mut().unwrap();

    if let Some(n) = next {
      lists[i] = Some(n);
      heap.push(Reverse((lists[i].as_ref().unwrap().val, i)));
    }
  }

  dummy.next
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut slow = head.clone();
  let mut fast = head.clone();

  while fast.is_some() {
    fast = fast.as_ref().unwrap().next.clone();

    if fast.is_none() {
      break;
    }

    fast = fast.as_ref().unwrap().next.clone();
    slow = slow.as_ref().unwrap().next.clone();
  }

  slow
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut prev = None;
  let mut current = head;

  while let Some(mut node) = current {
    current = node.next.take();
    node.next = prev;
    prev = Some(node);
  }

  prev
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_nth_from_end() {
    let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = remove_nth_from_end(list, 2);
    assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 5]);

    let list = ListNode::from_vec(vec![1]);
    let result = remove_nth_from_end(list, 1);
    assert_eq!(ListNode::to_vec(result), vec![]);

    let list = ListNode::from_vec(vec![1, 2]);
    let result = remove_nth_from_end(list, 1);
    assert_eq!(ListNode::to_vec(result), vec![1]);
  }

  #[test]
  fn test_merge_two_lists() {
    let list1 = ListNode::from_vec(vec![1, 2, 4]);
    let list2 = ListNode::from_vec(vec![1, 3, 4]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(ListNode::to_vec(result), vec![1, 1, 2, 3, 4, 4]);

    let list1 = ListNode::from_vec(vec![]);
    let list2 = ListNode::from_vec(vec![]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(ListNode::to_vec(result), vec![]);

    let list1 = ListNode::from_vec(vec![0]);
    let list2 = ListNode::from_vec(vec![]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(ListNode::to_vec(result), vec![0]);
  }

  #[test]
  fn test_merge_k_lists() {
    let list1 = ListNode::from_vec(vec![1, 4, 5]);
    let list2 = ListNode::from_vec(vec![1, 3, 4]);
    let list3 = ListNode::from_vec(vec![2, 6]);
    let lists = vec![list1, list2, list3];

    let result = merge_k_lists(lists);
    assert_eq!(ListNode::to_vec(result), vec![1, 1, 2, 3, 4, 4, 5, 6]);

    let empty_lists: Vec<Option<Box<ListNode>>> = vec![];
    let result = merge_k_lists(empty_lists);
    assert_eq!(ListNode::to_vec(result), vec![]);

    let single_list = ListNode::from_vec(vec![1]);
    let lists = vec![single_list];
    let result = merge_k_lists(lists);
    assert_eq!(ListNode::to_vec(result), vec![1]);
  }

  #[test]
  fn test_middle_node() {
    let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = middle_node(list);
    assert_eq!(ListNode::to_vec(result), vec![3, 4, 5]);

    let list = ListNode::from_vec(vec![1, 2, 3, 4]);
    let result = middle_node(list);
    assert_eq!(ListNode::to_vec(result), vec![3, 4]);

    let list = ListNode::from_vec(vec![1]);
    let result = middle_node(list);
    assert_eq!(ListNode::to_vec(result), vec![1]);
  }

  #[test]
  fn test_reverse_list() {
    let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = reverse_list(list);
    assert_eq!(ListNode::to_vec(result), vec![5, 4, 3, 2, 1]);

    let list = ListNode::from_vec(vec![1]);
    let result = reverse_list(list);
    assert_eq!(ListNode::to_vec(result), vec![1]);

    let list = ListNode::from_vec(vec![]);
    let result = reverse_list(list);
    assert_eq!(ListNode::to_vec(result), vec![]);
  }

  #[test]
  fn test_sort_list() {
    let list = ListNode::from_vec(vec![4, 2, 1, 3]);
    let result = sort_list(list);
    assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 4]);

    let list = ListNode::from_vec(vec![]);
    let result = sort_list(list);
    assert_eq!(ListNode::to_vec(result), vec![]);

    let list = ListNode::from_vec(vec![1]);
    let result = sort_list(list);
    assert_eq!(ListNode::to_vec(result), vec![1]);
  }

  #[test]
  fn test_get_intersection_node() {
    let mut list1 = ListNode::from_vec(vec![4, 6]);
    let mut list2 = ListNode::from_vec(vec![5, 0, 1]);
    let intersection = ListNode::from_vec(vec![8, 4, 5]);

    // Attach intersection to list1
    let mut tail1 = &mut list1;
    while let Some(ref mut node) = tail1 {
      if node.next.is_none() {
        node.next = intersection.clone();
        break;
      }
      tail1 = &mut node.next;
    }

    // Attach intersection to list2
    let mut tail2 = &mut list2;
    while let Some(ref mut node) = tail2 {
      if node.next.is_none() {
        node.next = intersection;
        break;
      }
      tail2 = &mut node.next;
    }

    let result = get_intersection_node(list1, list2);
    assert_eq!(ListNode::to_vec(result), vec![8, 4, 5]);
  }

  #[test]
  fn test_is_palindrome() {
    let list = ListNode::from_vec(vec![1, 2, 2, 1]);
    let result = is_palindrome(list);
    assert!(result);

    let list = ListNode::from_vec(vec![1, 2]);
    let result = is_palindrome(list);
    assert!(!result);
  }

  #[test]
  fn test_odd_even_list() {
    let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = odd_even_list(list);
    assert_eq!(ListNode::to_vec(result), vec![1, 3, 5, 2, 4]);

    let list = ListNode::from_vec(vec![2, 1, 3, 5, 6, 4]);
    let result = odd_even_list(list);
    assert_eq!(ListNode::to_vec(result), vec![2, 3, 6, 1, 5, 4]);
  }
}

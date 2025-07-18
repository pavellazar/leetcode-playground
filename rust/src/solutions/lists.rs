use std::{cmp::Reverse, collections::BinaryHeap};

use crate::data_structures::list_node::ListNode;

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
}

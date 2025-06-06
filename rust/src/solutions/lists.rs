#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for v in values {
      let new_node = Box::new(ListNode::new(v));
      *tail = Some(new_node);
      tail = &mut tail.as_mut().unwrap().next;
    }

    head
  }

  pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = head;
    while let Some(node) = current {
      vec.push(node.val);
      current = node.next;
    }
    vec
  }
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
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
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

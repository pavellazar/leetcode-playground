use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MedianFinder {
  max_heap: BinaryHeap<i32>,
  min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
  pub fn new() -> Self {
    Self {
      max_heap: BinaryHeap::new(),
      min_heap: BinaryHeap::new(),
    }
  }

  pub fn add_num(&mut self, num: i32) {
    // Step 1: Add to max_heap
    self.max_heap.push(num);

    // Step 2: Move the largest from max_heap to min_heap
    if let Some(max) = self.max_heap.pop() {
      self.min_heap.push(Reverse(max));
    }

    // Step 3: Rebalance if min_heap has more elements
    if self.min_heap.len() > self.max_heap.len() {
      if let Some(Reverse(min)) = self.min_heap.pop() {
        self.max_heap.push(min);
      }
    }
  }

  pub fn find_median(&self) -> f64 {
    if self.max_heap.len() > self.min_heap.len() {
      *self.max_heap.peek().unwrap() as f64
    } else if self.max_heap.len() < self.min_heap.len() {
      self.min_heap.peek().unwrap().0 as f64
    } else {
      (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_median_finder() {
    let mut mf = MedianFinder::new();
    mf.add_num(1);
    mf.add_num(2);
    assert_eq!(mf.find_median(), 1.5);
    mf.add_num(3);
    assert_eq!(mf.find_median(), 2.0);
    mf.add_num(4);
    assert_eq!(mf.find_median(), 2.5);
  }
}

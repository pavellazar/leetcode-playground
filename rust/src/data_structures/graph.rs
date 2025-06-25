use std::collections::HashMap;

pub struct Graph {
  adjacency: HashMap<i32, Vec<(i32, i32)>>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      adjacency: HashMap::new(),
    }
  }

  pub fn add_node(&mut self, node: i32) {
    self.adjacency.entry(node).or_default();
  }

  pub fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
    self.adjacency.entry(from).or_default().push((to, weight));
    self.adjacency.entry(to).or_default();
  }

  pub fn neighbors(&self, node: i32) -> Option<&Vec<(i32, i32)>> {
    self.adjacency.get(&node)
  }

  pub fn has_node(&self, node: i32) -> bool {
    self.adjacency.contains_key(&node)
  }

  pub fn topological_sort(&self) -> Option<Vec<i32>> {
    let mut in_degree = std::collections::HashMap::new();
    for (&node, neighbors) in &self.adjacency {
      in_degree.entry(node).or_insert(0);
      for &(neighbor, _) in neighbors {
        *in_degree.entry(neighbor).or_insert(0) += 1;
      }
    }

    let mut queue: Vec<i32> = in_degree
      .iter()
      .filter(|&(_, &deg)| deg == 0)
      .map(|(&node, _)| node)
      .collect();

    let mut result = vec![];
    let adjacency = self.adjacency.clone();

    while let Some(node) = queue.pop() {
      result.push(node);
      if let Some(neighbors) = adjacency.get(&node) {
        for &(neighbor, _) in neighbors {
          if let Some(deg) = in_degree.get_mut(&neighbor) {
            *deg -= 1;
            if *deg == 0 {
              queue.push(neighbor);
            }
          }
        }
      }
    }

    if result.len() == in_degree.len() {
      Some(result)
    } else {
      None
    }
  }
}

use crate::data_structures::graph::Graph;

// LeetCode #207 - Course Schedule
pub fn can_finish(_: i32, prerequisites: Vec<Vec<i32>>) -> bool {
  let mut graph = Graph::new();

  for prerequisite in prerequisites {
    let course = prerequisite[0];
    let prerequisite_course = prerequisite[1];
    graph.add_edge(prerequisite_course, course, 1);
  }

  let sorted = graph.topological_sort();
  match sorted {
    Some(_) => true,
    None => false,
  }
}

// LeetCode #210 - Course Schedule II
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
  let mut graph = Graph::new();

  // Add all courses to the graph
  for course in 0..num_courses {
    graph.add_node(course);
  }

  for prerequisite in prerequisites {
    let course = prerequisite[0];
    let prerequisite_course = prerequisite[1];
    graph.add_edge(prerequisite_course, course, 1);
  }

  let sorted = graph.topological_sort();
  match sorted {
    Some(sorted) => sorted,
    None => vec![],
  }
}

// LeetCode #547 - Number of Provinces
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
  fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, node: usize) {
    for neighbor in 0..is_connected.len() {
      if is_connected[node][neighbor] == 1 && !visited[neighbor] {
        visited[neighbor] = true;
        dfs(is_connected, visited, neighbor);
      }
    }
  }

  let n = is_connected.len();
  let mut visited = vec![false; n];
  let mut circles = 0;

  for i in 0..n {
    if !visited[i] {
      circles += 1;
      visited[i] = true;
      dfs(&is_connected, &mut visited, i);
    }
  }

  circles
}

// LeetCode #684 - Redundant Connection
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
  fn find(parents: &Vec<i32>, node: i32) -> i32 {
    let mut node = node;
    while node != parents[node as usize] {
      node = parents[node as usize];
    }
    node
  }

  fn union(parents: &mut Vec<i32>, u: i32, v: i32) {
    let root_u = find(parents, u);
    let root_v = find(parents, v);

    if root_u != root_v {
      parents[root_u as usize] = root_v
    }
  }

  let n: i32 = edges.len() as i32;
  let mut parents: Vec<i32> = (0..=n).collect();

  for edge in edges {
    if find(&parents, edge[0]) == find(&parents, edge[1]) {
      return vec![edge[0], edge[1]];
    } else {
      union(&mut parents, edge[0], edge[1]);
    }
  }

  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_redundant_connection() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(find_redundant_connection(edges), vec![2, 3]);

    let edges = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
    assert_eq!(find_redundant_connection(edges), vec![1, 3]);

    let edges = vec![vec![1, 2], vec![2, 1]];
    assert_eq!(find_redundant_connection(edges), vec![2, 1]);
  }

  #[test]
  fn test_find_circles() {
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    assert_eq!(find_circle_num(is_connected), 2);

    let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    assert_eq!(find_circle_num(is_connected), 3);

    let is_connected = vec![vec![1]];
    assert_eq!(find_circle_num(is_connected), 1);
  }

  #[test]
  fn test_can_finish() {
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    assert!(!can_finish(2, prerequisites));

    let prerequisites = vec![vec![1, 0]];
    assert!(can_finish(2, prerequisites));

    let prerequisites = vec![];
    assert!(can_finish(1, prerequisites));

    let prerequisites = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
    assert!(!can_finish(3, prerequisites));
  }

  #[test]
  fn test_find_order() {
    let prerequisites = vec![vec![1, 0], vec![2, 1]];
    let order = find_order(3, prerequisites);
    assert_eq!(order, vec![0, 1, 2]);

    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    let order = find_order(2, prerequisites);
    assert_eq!(order, vec![]);

    let prerequisites = vec![];
    let order = find_order(1, prerequisites);
    assert_eq!(order, vec![0]);
  }
}

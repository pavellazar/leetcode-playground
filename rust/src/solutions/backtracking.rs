// LeetCode #51 - N-Queens and #52 - N-Queens II
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
  fn backtrack(
    row: usize,
    size: usize,
    cols: &mut Vec<bool>,
    diag_a: &mut Vec<bool>,
    diag_b: &mut Vec<bool>,
    board: &mut Vec<usize>,
    solutions: &mut Vec<Vec<String>>,
  ) {
    // solution
    if row == size {
      let mut sol = Vec::new();
      for &col in board.iter() {
        let mut row_str = vec!['.'; size];
        row_str[col] = 'Q';
        sol.push(row_str.iter().collect());
      }
      solutions.push(sol);
      return;
    }

    for col in 0..size {
      if cols[col] || diag_a[row + col] || diag_b[size - 1 + row - col] {
        continue;
      }

      cols[col] = true;
      diag_a[row + col] = true;
      diag_b[size - 1 + row - col] = true;

      board.push(col);
      backtrack(row + 1, size, cols, diag_a, diag_b, board, solutions);
      board.pop();

      cols[col] = false;
      diag_a[row + col] = false;
      diag_b[size - 1 + row - col] = false;
    }
  }

  let size = n as usize;
  let mut solutions = Vec::new();
  let mut cols = vec![false; size];
  let mut diag_a = vec![false; 2 * size - 1];
  let mut diag_b = vec![false; 2 * size - 1];
  let mut board = Vec::new();

  backtrack(
    0,
    size,
    &mut cols,
    &mut diag_a,
    &mut diag_b,
    &mut board,
    &mut solutions,
  );
  solutions
}

// LeetCode #78 - Subsets
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
  fn backtrack(
    nums: &Vec<i32>,
    start: usize,
    current: &mut Vec<i32>,
    solutions: &mut Vec<Vec<i32>>,
  ) {
    solutions.push(current.clone());
    for i in start..nums.len() {
      current.push(nums[i]);
      backtrack(nums, i + 1, current, solutions);
      current.pop();
    }
  }

  let mut solutions = Vec::new();
  let mut current = Vec::new();
  backtrack(&nums, 0, &mut current, &mut solutions);
  solutions
}

// LeetCode #39 - Combination Sum
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  fn backtrack(
    candidates: &Vec<i32>,
    index: usize,
    current: &mut Vec<i32>,
    solutions: &mut Vec<Vec<i32>>,
    target: i32,
  ) {
    if target == 0 {
      solutions.push(current.clone());
      return;
    }

    if target < 0 || index == candidates.len() {
      return;
    }

    let current_value = candidates[index];

    // don't use current and move forward
    backtrack(candidates, index + 1, current, solutions, target);
    // use current and don't move forward
    current.push(current_value);
    backtrack(
      candidates,
      index,
      current,
      solutions,
      target - current_value,
    );
    current.pop();
  }

  let mut solutions = Vec::new();
  let mut current = Vec::new();
  backtrack(&candidates, 0, &mut current, &mut solutions, target);
  solutions
}

// LeetCode #79 - Word Search
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
  fn solve(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    chars: &Vec<char>,
    index: usize,
  ) -> bool {
    if index == chars.len() {
      return true;
    }

    if row >= board.len() || col >= board[0].len() {
      return false;
    }

    if board[row][col] != chars[index] || visited[row][col] {
      return false;
    }

    visited[row][col] = true;

    if solve(board, visited, row + 1, col, chars, index + 1)
      || solve(board, visited, row, col + 1, chars, index + 1)
      || (row > 0 && solve(board, visited, row - 1, col, chars, index + 1))
      || (col > 0 && solve(board, visited, row, col - 1, chars, index + 1))
    {
      return true;
    }

    visited[row][col] = false;
    false
  }

  let rows = board.len();
  if rows == 0 {
    return false;
  }

  let cols = board[0].len();
  if cols == 0 {
    return false;
  }

  let mut visited = vec![vec![false; board[0].len()]; board.len()];
  let word = word.chars().collect();
  for r in 0..rows {
    for c in 0..cols {
      if solve(&board, &mut visited, r, c, &word, 0) {
        return true;
      }
    }
  }

  false
}

// LeetCode #17 - Letter Combinations of a Phone Number
pub fn letter_combinations(digits: String) -> Vec<String> {
  let mut combinations = vec![];

  let mut map = std::collections::HashMap::new();
  map.insert('2', vec!['a', 'b', 'c']);
  map.insert('3', vec!['d', 'e', 'f']);
  map.insert('4', vec!['g', 'h', 'i']);
  map.insert('5', vec!['j', 'k', 'l']);
  map.insert('6', vec!['m', 'n', 'o']);
  map.insert('7', vec!['p', 'q', 'r', 's']);
  map.insert('8', vec!['t', 'u', 'v']);
  map.insert('9', vec!['w', 'x', 'y', 'z']);

  if digits.is_empty() {
    return combinations;
  }

  fn backtrack(
    digits: &str,
    index: usize,
    current: &mut String,
    map: &std::collections::HashMap<char, Vec<char>>,
    combinations: &mut Vec<String>,
  ) {
    if index == digits.len() {
      combinations.push(current.clone());
      return;
    }

    if let Some(chars) = map.get(&digits.chars().nth(index).unwrap()) {
      for &c in chars {
        current.push(c);
        backtrack(digits, index + 1, current, map, combinations);
        current.pop();
      }
    }
  }

  let mut current = String::new();
  backtrack(&digits, 0, &mut current, &map, &mut combinations);
  combinations
    .into_iter()
    .map(|s| s.chars().collect::<String>())
    .collect()
}

// LeetCode #22 - Generate Parentheses
pub fn generate_parenthesis(n: i32) -> Vec<String> {
  fn backtrack(s: String, open: i32, close: i32, max: i32, result: &mut Vec<String>) {
    if s.len() == (max * 2) as usize {
      result.push(s);
      return;
    }
    if open < max {
      backtrack(format!("{}(", s), open + 1, close, max, result);
    }
    if close < open {
      backtrack(format!("{})", s), open, close + 1, max, result);
    }
  }

  let mut result = Vec::new();
  backtrack(String::new(), 0, 0, n, &mut result);
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_parenthesis() {
    let result = generate_parenthesis(3);
    assert_eq!(
      result,
      vec![
        "((()))".to_string(),
        "(()())".to_string(),
        "(())()".to_string(),
        "()(())".to_string(),
        "()()()".to_string()
      ]
    );
  }
  
  #[test]
  fn test_letter_combinations() {
    assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());

    let mut result = letter_combinations("2".to_string());
    result.sort();
    assert_eq!(result, vec!["a", "b", "c"]);

    let mut result = letter_combinations("23".to_string());
    result.sort();
    assert_eq!(
      result,
      vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );

    let result = letter_combinations("1".to_string());
    assert_eq!(result, Vec::<String>::new());

    let mut result = letter_combinations("234".to_string());
    result.sort();
    assert!(result.contains(&"adg".to_string()));
    assert!(result.contains(&"beh".to_string()));
    assert!(result.contains(&"cfi".to_string()));
    assert_eq!(result.len(), 27);
  }

  #[test]
  fn test_solve_n_queens() {
    assert_eq!(
      solve_n_queens(4),
      vec![
        vec![".Q..", "...Q", "Q...", "..Q."],
        vec!["..Q.", "Q...", "...Q", ".Q.."]
      ]
    );
    assert_eq!(solve_n_queens(1), vec![vec!["Q"]]);
  }

  #[test]
  fn test_subsets() {
    assert_eq!(
      subsets(vec![1, 2, 3]),
      vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 3],
        vec![2],
        vec![2, 3],
        vec![3]
      ]
    );
  }

  #[test]
  fn test_combinations_sum() {
    assert_eq!(
      combination_sum(vec![2, 3, 6, 7], 7),
      vec![vec![7], vec![2, 2, 3]]
    );
  }

  #[test]
  fn test_exist() {
    let board = vec![
      vec!['A', 'B', 'C', 'E'],
      vec!['S', 'F', 'C', 'S'],
      vec!['A', 'D', 'E', 'E'],
    ];
    assert!(exist(board.clone(), "ABCCED".to_string()));
    assert!(exist(board, "ABCCEG".to_string()) == false);
    let board = vec![
      vec!['A', 'B', 'C', 'E'],
      vec!['S', 'F', 'C', 'S'],
      vec!['A', 'D', 'E', 'E'],
    ];
    assert!(!exist(board.clone(), "ABCBCCED".to_string()));
    assert!(exist(vec![vec!['a']], "a".to_string()));
  }
}

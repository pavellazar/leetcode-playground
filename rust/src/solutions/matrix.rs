pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
  transpose(matrix);
  reverse(matrix);
}

fn transpose(matrix: &mut Vec<Vec<i32>>) {
  for i in 0..matrix.len() {
    for j in i + 1..matrix[i].len() {
      matrix[i][j] ^= matrix[j][i];
      matrix[j][i] ^= matrix[i][j];
      matrix[i][j] ^= matrix[j][i];
    }
  }
}

fn reverse(matrix: &mut Vec<Vec<i32>>) {
  for row in matrix.iter_mut() {
    let mut left = 0;
    let mut right = row.len() - 1;
    while left < right {
      row[left] ^= row[right];
      row[right] ^= row[left];
      row[left] ^= row[right];
      left += 1;
      right -= 1;
    }
  }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
  let rows = matrix.len();
  if rows == 0 {
    return Vec::new();
  }
  let columns = matrix[0].len();
  if columns == 0 {
    return Vec::new();
  }

  let mut result = Vec::new();
  let mut top = 0;
  let mut bottom = rows - 1;
  let mut left = 0;
  let mut right = columns - 1;

  while top <= bottom && left <= right {
    for i in left..=right {
      result.push(matrix[top][i]);
    }
    top += 1;

    for i in top..=bottom {
      result.push(matrix[i][right]);
    }
    if right == 0 {
      break;
    }
    right = right.saturating_sub(1);

    if top <= bottom {
      for i in (left..=right).rev() {
        result.push(matrix[bottom][i]);
      }
      if bottom == 0 {
        break;
      }
      bottom = bottom.saturating_sub(1);
    }

    if left <= right {
      for i in (top..=bottom).rev() {
        result.push(matrix[i][left]);
      }
      left += 1;
    }
  }

  result
}

fn safe_access(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
  if row < matrix.len() && col < matrix[0].len() {
    matrix[row][col]
  } else {
    0
  }
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
  if m == 0 || n == 0 {
    return 0;
  }

  let rows = m as usize;
  let cols = n as usize;

  let mut matrix = vec![vec![0; cols]; rows];
  matrix[rows - 1][cols - 1] = 1;

  for row in (0..rows).rev() {
    for col in (0..cols).rev() {
      if row == rows - 1 && col == cols - 1 {
        continue; // Don't overwrite the destination cell
      }

      matrix[row][col] = safe_access(&matrix, row + 1, col) + safe_access(&matrix, row, col + 1);
    }
  }

  matrix[0][0]
}

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
  let mut matrix = grid.clone();
  let rows = matrix.len();

  if rows == 0 {
    return 0;
  }

  let cols = matrix[0].len();

  if cols == 0 {
    return 0;
  }

  for row in (0..rows).rev() {
    for col in (0..cols).rev() {
      if row == rows - 1 && col == cols - 1 {
        continue; // Don't overwrite the destination cell
      }

      if col + 1 < cols && row + 1 < rows {
        matrix[row][col] += matrix[row + 1][col].min(matrix[row][col + 1]);
      } else if col + 1 < cols {
        matrix[row][col] += matrix[row][col + 1];
      } else if row + 1 < rows {
        matrix[row][col] += matrix[row + 1][col];
      }
    }
  }

  matrix[0][0]
}

// Leet Code #200 - Number of Islands
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  fn consume(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if i >= grid.len() || j >= grid[0].len() || visited[i][j] || grid[i][j] == '0' {
      return;
    }
    visited[i][j] = true;
    if i > 0 {
      consume(grid, visited, i - 1, j);
    }
    if j > 0 {
      consume(grid, visited, i, j - 1);
    }
    consume(grid, visited, i + 1, j);
    consume(grid, visited, i, j + 1);
  }

  let rows = grid.len();
  let cols = grid[0].len();
  let mut visited = vec![vec![false; cols]; rows];
  let mut islands = 0;
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == '1' && !visited[i][j] {
        islands += 1;
        consume(&grid, &mut visited, i, j);
      }
    }
  }

  islands
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_num_islands() {
    let grid = vec![
      vec!['1', '1', '0', '0', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '1', '0', '0'],
      vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(num_islands(grid), 3);

    let grid2 = vec![vec!['1']];
    assert_eq!(num_islands(grid2), 1);

    let grid3: Vec<Vec<char>> = vec![];
    assert_eq!(num_islands(grid3), 0); // Edge case
  }

  #[test]
  fn test_min_path_sum() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(min_path_sum(grid), 7);

    let grid2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(min_path_sum(grid2), 12);

    let grid3 = vec![vec![1]];
    assert_eq!(min_path_sum(grid3), 1);

    let grid4: Vec<Vec<i32>> = vec![];
    assert_eq!(min_path_sum(grid4), 0); // Edge case
  }

  #[test]
  fn test_unique_paths() {
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
    assert_eq!(unique_paths(3, 3), 6);
    assert_eq!(unique_paths(1, 1), 1);
    assert_eq!(unique_paths(0, 0), 0); // Edge case
  }

  #[test]
  fn test_rotate() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

    let mut matrix2 = vec![vec![1, 2], vec![3, 4]];
    rotate(&mut matrix2);
    assert_eq!(matrix2, vec![vec![3, 1], vec![4, 2]]);
  }

  #[test]
  fn test_spiral_order() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = spiral_order(matrix);
    assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

    let matrix2 = vec![vec![2], vec![3]];
    let result2 = spiral_order(matrix2);
    assert_eq!(result2, vec![2, 3]);
  }
}

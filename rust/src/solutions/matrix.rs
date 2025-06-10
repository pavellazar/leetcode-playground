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

#[cfg(test)]
mod tests {
  use super::*;

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

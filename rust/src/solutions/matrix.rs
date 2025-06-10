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
}

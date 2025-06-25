// LeetCode #50. Pow(x, n)
pub fn my_pow(x: f64, n: i32) -> f64 {
  let mut result: f64 = 1.0;
  let mut pow = n.abs();
  let x = if n < 0 { 1.0 / x } else { x };

  while pow > 0 {
    result *= x;
    pow -= 1;
  }

  result
}

pub fn my_pow_exponentiation(mut x: f64, n: i32) -> f64 {
  let mut result = 1.0;
  let mut pow = if n == i32::MIN {
    (i32::MAX as u32) + 1
  } else {
    n.abs() as u32
  };

  while pow > 0 {
    if pow % 2 == 1 {
      result *= x;
    }
    x *= x;
    pow /= 2;
  }

  if n < 0 {
    1.0 / result
  } else {
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_my_pow() {
    assert!((my_pow(2.0, 10) - 1024.0).abs() < 1e-6);
    assert!((my_pow(2.1, 3) - 9.261).abs() < 1e-6);
    assert!((my_pow_exponentiation(2.0, -2) - 0.25).abs() < 1e-6);
    assert!((my_pow_exponentiation(2.0, -2147483648) - 0.0).abs() < 1e-6);
  }
}

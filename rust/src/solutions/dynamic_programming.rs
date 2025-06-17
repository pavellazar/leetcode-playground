pub fn climb_stairs(n: i32) -> i32 {
  fn step(n: i32) -> i32 {
    if n == 0 || n == 1 {
      1
    } else {
      step(n - 1) + step(n - 2)
    }
  }
  step(n)
}

pub fn climb_stairs_math(n: i32) -> i32 {
  if n == 0 || n == 1 {
    return 1;
  }
  
  let mut a = 1; // f(0)
  let mut b = 1; // f(1)
  
  for _ in 2..=n {
    let temp = a + b;
    a = b;
    b = temp;
  }
  
  b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);
    assert_eq!(climb_stairs(6), 13);
    assert_eq!(climb_stairs_math(44), 1134903170);
  }
}

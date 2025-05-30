pub fn atoi(s: &str) -> i32 {
  let mut result: i32 = 0;
  let mut sign = 1;
  let mut accept_sign = true;
  let mut accept_space = true;

  for c in s.chars() {
    if c.is_whitespace() {
      if accept_space {
        continue;
      }

      break;
    }

    if c.is_alphabetic() {
      break;
    }

    if c == '-' || c == '+' {
      if !accept_sign {
        break;
      }

      if c == '-' {
        sign = -1;
      }

      accept_space = false;
      accept_sign = false;
      continue;
    }

    if let Some(digit) = c.to_digit(10) {
      accept_space = false;
      accept_sign = false;

      // check for overflow/underflow before updating result
      if sign == 1 && (result > (i32::MAX - digit as i32) / 10) {
        return i32::MAX;
      }
      if sign == -1 && (result < (i32::MIN + digit as i32) / 10) {
        return i32::MIN;
      }

      result = result * 10 + digit as i32 * sign;
      continue;
    }

    break;
  }

  result
}

#[test]
fn test_atoi() {
  assert_eq!(42, atoi("+42"));
  assert_eq!(42, atoi("42"));
  assert_eq!(-42, atoi(" -042"));
  assert_eq!(1337, atoi("1337c0d3"));
  assert_eq!(0, atoi("0-1"));
  assert_eq!(0, atoi("words and 987"));
}

pub fn is_palindrome(n: i32) -> bool {
  if n < 0 {
    return false;
  }

  let r = reverse(n);
  r == n
}

pub fn reverse(x: i32) -> i32 {
  let mut result: i32 = 0;
  let mut work = x;

  let max_div_10 = i32::MAX / 10;
  let max_last_digit = i32::MAX % 10;
  let min_div_10 = i32::MIN / 10;
  let min_last_digit = i32::MIN % 10;

  while work != 0 {
    let digit = work % 10;

    if result > max_div_10 || (result == max_div_10 && digit > max_last_digit) {
      return 0;
    }
    if result < min_div_10 || (result == min_div_10 && digit < min_last_digit) {
      return 0;
    }
    result = result * 10 + digit;
    work /= 10;
  }

  result
}

#[test]
fn test_reverse() {
  assert_eq!(0, reverse(1534236469));
  assert_eq!(123, reverse(321));
  assert_eq!(1, reverse(10));
  assert_eq!(0, reverse(-2147483648));
}

#[test]
fn test_palindrome() {
  assert_eq!(true, is_palindrome(121));
  assert_eq!(false, is_palindrome(1234));
}

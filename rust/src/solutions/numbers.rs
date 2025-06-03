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

pub fn integer_to_roman(num: i32) -> String {
  if num < 1 || num > 3999 {
    return "".to_string();
  }

  let roman = [
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"], // Ones
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"], // Tens
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"], // Hundreds
    ["", "M", "MM", "MMM", "", "", "", "", "", ""],               // Thousands
  ];

  let mut digits = Vec::new();
  let mut n = num.abs();
  while n > 0 {
    digits.push(n % 10);
    n /= 10;
  }
  let mut result = String::new();
  for (i, d) in digits.iter().enumerate().rev() {
    result.push_str(roman[i][*d as usize]);
  }
  result
}

pub fn roman_to_integer(s: &str) -> i32 {
  let mut result = 0;
  let mut prev = 0;

  fn value(c: char) -> i32 {
    match c {
      'I' => 1,
      'V' => 5,
      'X' => 10,
      'L' => 50,
      'C' => 100,
      'D' => 500,
      'M' => 1000,
      _ => 0,
    }
  }

  for c in s.chars().rev() {
    let v = value(c);
    if v < prev {
      result -= v;
    } else {
      result += v;
    }
    prev = v;
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

#[test]
fn test_to_roman() {
  assert_eq!("I", integer_to_roman(1));
  assert_eq!("IV", integer_to_roman(4));
  assert_eq!("IX", integer_to_roman(9));
  assert_eq!("XLII", integer_to_roman(42));
  assert_eq!("XCIX", integer_to_roman(99));
  assert_eq!("CXXIII", integer_to_roman(123));
  assert_eq!("MCMXCIV", integer_to_roman(1994));
  assert_eq!("MMMCMXCIX", integer_to_roman(3999));
  assert_eq!("MMMDCCXLIX", integer_to_roman(3749));
}

#[test]
fn test_roman_to_integer() {
  assert_eq!(roman_to_integer("I"), 1);
  assert_eq!(roman_to_integer("IV"), 4);
  assert_eq!(roman_to_integer("IX"), 9);
  assert_eq!(roman_to_integer("XLII"), 42);
  assert_eq!(roman_to_integer("XCIX"), 99);
  assert_eq!(roman_to_integer("CXXIII"), 123);
  assert_eq!(roman_to_integer("MCMXCIV"), 1994);
  assert_eq!(roman_to_integer("MMMCMXCIX"), 3999);
  assert_eq!(roman_to_integer("MMMDCCXLIX"), 3749);
}
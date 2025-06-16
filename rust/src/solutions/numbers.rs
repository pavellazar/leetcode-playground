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

pub fn number_to_words(num: u32) -> String {
  fn helper(num: u32) -> String {
    const UNDER_TWENTY: [&str; 20] = [
      "Zero",
      "One",
      "Two",
      "Three",
      "Four",
      "Five",
      "Six",
      "Seven",
      "Eight",
      "Nine",
      "Ten",
      "Eleven",
      "Twelve",
      "Thirteen",
      "Fourteen",
      "Fifteen",
      "Sixteen",
      "Seventeen",
      "Eighteen",
      "Nineteen",
    ];

    const TENS: [&str; 10] = [
      "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];

    const MULTIPLIER_WORDS: [&str; 4] = ["Hundred", "Thousand", "Million", "Billion"];
    const MULTIPLIER_VALS: [u32; 4] = [100, 1000, 1_000_000, 1_000_000_000];

    if num < 20 {
      return UNDER_TWENTY[num as usize].to_string();
    }

    if num < 100 {
      let count = num / 10;
      let remainder = num % 10;

      if remainder > 0 {
        return format!(
          "{} {}",
          TENS[count as usize].to_string(),
          helper(remainder).to_string()
        );
      } else {
        return TENS[count as usize].to_string();
      }
    }

    for i in (0..MULTIPLIER_VALS.len()).rev() {
      if num >= MULTIPLIER_VALS[i] {
        let remainder = num % MULTIPLIER_VALS[i];

        if remainder > 0 {
          return format!(
            "{} {} {}",
            helper(num / MULTIPLIER_VALS[i]).to_string(),
            MULTIPLIER_WORDS[i].to_string(),
            helper(remainder).to_string()
          );
        } else {
          return format!(
            "{} {}",
            helper(num / MULTIPLIER_VALS[i]).to_string(),
            MULTIPLIER_WORDS[i].to_string()
          );
        }
      }
    }

    "".to_string()
  }

  helper(num)
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn test_number_to_words() {
    assert_eq!(number_to_words(20), "Twenty");
    assert_eq!(number_to_words(0), "Zero");
    assert_eq!(number_to_words(123), "One Hundred Twenty Three");
    assert_eq!(
      number_to_words(12345),
      "Twelve Thousand Three Hundred Forty Five"
    );
    assert_eq!(number_to_words(1000), "One Thousand");
    assert_eq!(
      number_to_words(1234567),
      "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
    assert_eq!(number_to_words(1100), "One Thousand One Hundred");
    assert_eq!(number_to_words(101), "One Hundred One");
    assert_eq!(number_to_words(1001), "One Thousand One");
    assert_eq!(number_to_words(1000000), "One Million");
    assert_eq!(number_to_words(1001000), "One Million One Thousand");
    assert_eq!(number_to_words(1000100), "One Million One Hundred");
  }
}

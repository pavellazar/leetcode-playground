mod solutions;

fn main() {
  println!("{:?}", solutions::arrays::two_sum(vec![2, 7, 11, 15], 9));
  println!("{:?}", solutions::strings::longest_palindrome("babad"));
  println!("{:?}", solutions::strings::convert("PAYPALISHIRING", 3));
  println!("{:?}", solutions::numbers::reverse(123));
  println!("{:?}", solutions::strings::atoi("123"));
  println!("{:?}", solutions::numbers::is_palindrome(121));
  println!("{:?}", solutions::strings::is_match("aa", "a"));
}

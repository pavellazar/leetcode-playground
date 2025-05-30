mod solutions;

fn main() {
  println!("{:?}", solutions::two_sum::two_sum(vec![2, 7, 11, 15], 9));
  println!(
    "{:?}",
    solutions::longest_palindrome::longest_palindrome("babad".to_string())
  );
  println!(
    "{:?}",
    solutions::zig_zag_convert::convert("PAYPALISHIRING".to_string(), 3)
  );
  println!("{:?}", solutions::numbers::reverse(123));
  println!("{:?}", solutions::string_to_integer::atoi("123"));
  println!("{:?}", solutions::numbers::is_palindrome(121));
}

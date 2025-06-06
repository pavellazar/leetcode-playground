mod solutions;

fn main() {
  println!("{:?}", solutions::arrays::two_sum(vec![2, 7, 11, 15], 9));
  println!("{:?}", solutions::strings::longest_palindrome("babad"));
  println!("{:?}", solutions::strings::convert("PAYPALISHIRING", 3));
  println!("{:?}", solutions::numbers::reverse(123));
  println!("{:?}", solutions::strings::atoi("123"));
  println!("{:?}", solutions::numbers::is_palindrome(121));
  println!("{:?}", solutions::strings::is_match("aa", "a"));

  let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
  println!("{:?}", solutions::arrays::container(heights));
  println!("{:?}", solutions::numbers::integer_to_roman(121));
  println!("{:?}", solutions::numbers::roman_to_integer("MMM"));
  println!(
    "{:?}",
    solutions::strings::longest_common_prefix(vec![
      "flow".to_string(),
      "flight".to_string(),
      "flower".to_string()
    ])
  );
  println!(
    "{:?}",
    solutions::arrays::three_zero_sum(vec![-1, 0, 1, 2, -1, -4])
  );
  let list = solutions::lists::ListNode::from_vec(vec![1, 2, 3, 4, 5]);
  let result = solutions::lists::remove_nth_from_end(list, 2);
  println!("{:?}", solutions::lists::ListNode::to_vec(result));
}

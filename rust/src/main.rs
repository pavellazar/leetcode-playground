mod solutions;

fn main() {
    println!("{:?}", solutions::two_sum::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", solutions::longest_palindrome::longest_palindrome("babad".to_string()));
}

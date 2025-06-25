use std::collections::VecDeque;

// LeetCode #412. Fizz Buzz
pub fn fizz_buzz(n: i32) -> Vec<String> {
  // i divisible by 3 and 5 -> FizzBuzz
  // i dibisible by 3 -> Fizz
  // i divisible by 5 -> Buzz

  let mut result = vec![];
  for i in 1..=n {
    if i % 15 == 0 {
      result.push("FizzBuzz".to_string())
    } else if i % 5 == 0 {
      result.push("Buzz".to_string())
    } else if i % 3 == 0 {
      result.push("Fizz".to_string())
    } else {
      result.push(i.to_string())
    }
  }

  result
}

// LeetCode #150. Evaluate Reverse Polish Notation
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
  let mut operands = VecDeque::new();
  for op in tokens {
    match op.as_str() {
      "+" => {
        let lhs = operands.pop_front().unwrap();
        let rhs = operands.pop_front().unwrap();

        operands.push_front(rhs + lhs);
      }
      "-" => {
        let lhs = operands.pop_front().unwrap();
        let rhs = operands.pop_front().unwrap();

        operands.push_front(rhs - lhs);
      }
      "*" => {
        let lhs = operands.pop_front().unwrap();
        let rhs = operands.pop_front().unwrap();

        operands.push_front(rhs * lhs);
      }
      "/" => {
        let lhs = operands.pop_front().unwrap();
        let rhs = operands.pop_front().unwrap();

        operands.push_front(rhs / lhs);
      }
      op => {
        let operand = op.parse::<i32>().unwrap();
        operands.push_front(operand);
      }
    }
  }

  operands.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  // Test for LeetCode #150 - Evaluate Reverse Polish Notation
  #[test]
  fn test_eval_rpn() {
    assert_eq!(
      eval_rpn(vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string()
      ]),
      9
    );

    assert_eq!(
      eval_rpn(vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
      ]),
      6
    );

    assert_eq!(
      eval_rpn(vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
      ]),
      22
    );
  }

  #[test]
  fn test_fizz_buzz() {
    assert_eq!(
      fizz_buzz(15),
      vec![
        "1".to_string(),
        "2".to_string(),
        "Fizz".to_string(),
        "4".to_string(),
        "Buzz".to_string(),
        "Fizz".to_string(),
        "7".to_string(),
        "8".to_string(),
        "Fizz".to_string(),
        "Buzz".to_string(),
        "11".to_string(),
        "Fizz".to_string(),
        "13".to_string(),
        "14".to_string(),
        "FizzBuzz".to_string()
      ]
    );
    assert_eq!(fizz_buzz(1), vec!["1".to_string()]);
  }
}

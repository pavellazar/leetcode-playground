// LeetCode #121 - Best Time to Buy and Sell Stock
pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut min_price = i32::MAX;
  let mut max_profit = 0;
  for price in prices {
    if price < min_price {
      min_price = price;
    } else if price - min_price > max_profit {
      max_profit = price - min_price;
    }
  }
  max_profit
}

// LeetCode #169 - Majority Element
pub fn majority_element(nums: Vec<i32>) -> i32 {
  let mut count = 1;
  let mut candidate = nums[0];

  for i in 1..nums.len() {
    if nums[i] == candidate {
      count += 1;
    } else {
      count -= 1;

      if count == 0 {
        candidate = nums[i];
        count = 1;
      }
    }
  }

  candidate
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_max_profit() {
    assert_eq!(super::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(super::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(super::max_profit(vec![2, 4, 1]), 2);
  }

  #[test]
  fn test_majority_element() {
    assert_eq!(super::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(super::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
  }
}

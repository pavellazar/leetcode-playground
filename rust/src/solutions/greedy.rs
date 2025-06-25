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

#[cfg(test)]
mod tests {
  #[test]
  fn test_max_profit() {
    assert_eq!(super::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(super::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(super::max_profit(vec![2, 4, 1]), 2);
  }
}

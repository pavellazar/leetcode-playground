use std::collections::HashMap;

pub fn two_sum(array: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in array.iter().enumerate() {
        match map.get(&(target - num)) {
            Some(&index) => return vec![index as i32, i as i32],
            None => map.insert(num, i),
        };
    }

    vec![]
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
use crate::common::string_utils::signature;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
  if p.len() > s.len() {
    return vec![];
  }

  let p_hash = signature(&p);
  let mut slide_hash = signature(&s[0..p.len()]);
  let mut result = vec![];

  if p_hash == slide_hash {
    result.push(0);
  }

  for i in 1..=s.len() - p.len() {
    let sub_byte = s.as_bytes()[i - 1] - b'a';
    let add_byte = s.as_bytes()[i + p.len() - 1] - b'a';

    slide_hash[sub_byte as usize] -= 1;
    slide_hash[add_byte as usize] += 1;

    if p_hash == slide_hash {
      result.push(i as i32);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_anagrams() {
    assert_eq!(find_anagrams("af".to_string(), "be".to_string()), vec![]);

    assert_eq!(
      find_anagrams("abab".to_string(), "ab".to_string()),
      vec![0, 1, 2]
    );
    assert_eq!(
      find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
      vec![0, 6]
    );
  }
}

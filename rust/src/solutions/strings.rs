use std::collections::HashMap;

pub fn longest_palindrome(s: &str) -> String {
  let s_chars: Vec<char> = s.chars().collect();
  let mut start = 0;
  let mut max_len = 0;

  for i in 0..s_chars.len() {
    // Check for odd-length palindromes
    let (odd_start, odd_len) = expand_around_center(&s_chars, i, i);
    if odd_len > max_len {
      start = odd_start;
      max_len = odd_len;
    }

    // Check for even-length palindromes
    let (even_start, even_len) = expand_around_center(&s_chars, i, i + 1);
    if even_len > max_len {
      start = even_start;
      max_len = even_len;
    }
  }

  s_chars[start..start + max_len].iter().collect()
}

fn expand_around_center(s: &[char], left: usize, right: usize) -> (usize, usize) {
  let mut lhs: i32 = left as i32;
  let mut rhs: i32 = right as i32;

  while lhs >= 0 && lhs <= rhs && rhs < s.len() as i32 && s[lhs as usize] == s[rhs as usize] {
    lhs -= 1;
    rhs += 1;
  }

  ((lhs + 1) as usize, (rhs - lhs - 1) as usize)
}

pub fn convert(s: &str, num_rows: i32) -> String {
  if num_rows == 1 {
    return s.to_string();
  }

  let num_rows = num_rows as usize;
  let mut rows = vec![String::new(); num_rows];
  let mut cur_row = 0;
  let mut going_down = false;

  for c in s.chars() {
    rows[cur_row].push(c);
    if cur_row == 0 || cur_row == num_rows - 1 {
      going_down = !going_down;
    }
    if going_down {
      cur_row += 1;
    } else {
      cur_row -= 1;
    }
  }

  rows.concat()
}

pub fn atoi(s: &str) -> i32 {
  let mut result: i32 = 0;
  let mut sign = 1;
  let mut accept_sign = true;
  let mut accept_space = true;

  for c in s.chars() {
    if c.is_whitespace() {
      if accept_space {
        continue;
      }

      break;
    }

    if c.is_alphabetic() {
      break;
    }

    if c == '-' || c == '+' {
      if !accept_sign {
        break;
      }

      if c == '-' {
        sign = -1;
      }

      accept_space = false;
      accept_sign = false;
      continue;
    }

    if let Some(digit) = c.to_digit(10) {
      accept_space = false;
      accept_sign = false;

      // check for overflow/underflow before updating result
      if sign == 1 && (result > (i32::MAX - digit as i32) / 10) {
        return i32::MAX;
      }
      if sign == -1 && (result < (i32::MIN + digit as i32) / 10) {
        return i32::MIN;
      }

      result = result * 10 + digit as i32 * sign;
      continue;
    }

    break;
  }

  result
}

pub fn is_match(s: &str, p: &str) -> bool {
  fn helper(s: &[u8], p: &[u8]) -> bool {
    if p.is_empty() {
      return s.is_empty();
    }

    let first_match = !s.is_empty() && (p[0] == b'.' || p[0] == s[0]);

    if p.len() >= 2 && p[1] == b'*' {
      helper(s, &p[2..]) || (first_match && helper(&s[1..], p))
    } else {
      first_match && helper(&s[1..], &p[1..])
    }
  }
  helper(s.as_bytes(), p.as_bytes())
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  if strs.is_empty() {
    return String::new();
  }
  let mut prefix: Vec<char> = strs[0].chars().collect();

  for s in &strs[1..] {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < prefix.len() && i < chars.len() && prefix[i] == chars[i] {
      i += 1;
    }
    prefix.truncate(i);
    if prefix.is_empty() {
      break;
    }
  }

  prefix.into_iter().collect()
}

pub fn is_valid_parentheses(s: String) -> bool {
  let open_parantheses = vec!['(', '{', '['];
  let close_parantheses = vec![')', '}', ']'];

  let mut stack = Vec::new();
  for c in s.chars() {
    if open_parantheses.contains(&c) {
      stack.push(c);
    } else if close_parantheses.contains(&c) {
      let last = stack.pop();
      let index = close_parantheses.iter().position(|&x| x == c);

      if let Some(index) = index {
        if let Some(last) = last {
          if last != open_parantheses[index] {
            return false;
          }
        } else {
          return false;
        }
      } else {
        return false;
      }
    } else {
      return false;
    }
  }

  stack.is_empty()
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
  let mut map: HashMap<String, Vec<String>> = HashMap::new();

  for s in strs {
    let key = sort_string(&s);
    map.entry(key).or_insert_with(Vec::new).push(s);
  }

  map.into_values().collect()
}

fn sort_string(s: &str) -> String {
  let mut chars: Vec<char> = s.chars().collect();
  chars.sort_unstable();
  chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_atoi() {
    assert_eq!(42, atoi("+42"));
    assert_eq!(42, atoi("42"));
    assert_eq!(-42, atoi(" -042"));
    assert_eq!(1337, atoi("1337c0d3"));
    assert_eq!(0, atoi("0-1"));
    assert_eq!(0, atoi("words and 987"));
  }

  #[test]
  fn test_convert() {
    assert_eq!(convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
  }

  #[test]
  fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone"), "wfdfw");
    assert_eq!(longest_palindrome("cbbd"), "bb");
    assert_eq!(longest_palindrome("a"), "a");
    assert_eq!(longest_palindrome("ac"), "a");
  }

  #[test]
  fn test_is_match() {
    assert_eq!(false, is_match("aa", "a"));
    assert_eq!(true, is_match("aa", "a*"));
    assert_eq!(true, is_match("aa", ".*"));
    assert_eq!(false, is_match("ab", ".*c"));
    assert_eq!(true, is_match("aab", "c*a*b"));
    assert_eq!(true, is_match("aaa", "a*a"));
  }

  #[test]
  fn test_longest_common_prefix() {
    assert_eq!(
      longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string()
      ]),
      "fl"
    );
    assert_eq!(
      longest_common_prefix(vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string()
      ]),
      ""
    );
    assert_eq!(
      longest_common_prefix(vec![
        "interspecies".to_string(),
        "interstellar".to_string(),
        "interstate".to_string()
      ]),
      "inters"
    );
    assert_eq!(
      longest_common_prefix(vec!["throne".to_string(), "throne".to_string()]),
      "throne"
    );
    assert_eq!(
      longest_common_prefix(vec!["".to_string(), "".to_string()]),
      ""
    );
    assert_eq!(longest_common_prefix(vec!["prefix".to_string()]), "prefix");
    assert_eq!(longest_common_prefix(Vec::<String>::new()), "");
  }

  #[test]
  fn test_is_valid_parantheses() {
    assert_eq!(is_valid_parentheses("()".to_string()), true);
    assert_eq!(is_valid_parentheses("()[]{}".to_string()), true);
    assert_eq!(is_valid_parentheses("(]".to_string()), false);
    assert_eq!(is_valid_parentheses("([)]".to_string()), false);
    assert_eq!(is_valid_parentheses("{[]}".to_string()), true);
    assert_eq!(is_valid_parentheses("".to_string()), true);
    assert_eq!(is_valid_parentheses("((()))".to_string()), true);
    assert_eq!(is_valid_parentheses("({[()]})".to_string()), true);
    assert_eq!(is_valid_parentheses("({[()]}){".to_string()), false);
  }

  #[test]
  fn test_group_anagrams() {
    let input = vec![
      "eat".to_string(),
      "tea".to_string(),
      "tan".to_string(),
      "ate".to_string(),
      "nat".to_string(),
      "bat".to_string(),
    ];
    let result = group_anagrams(input);
    assert_eq!(result.len(), 3);
    assert!(result.iter().any(|v| v == &vec!["bat".to_string()]));
    assert!(result.iter().any(|v| v == &vec!["eat".to_string(), "tea".to_string(), "ate".to_string()]));
    assert!(result.iter().any(|v| v == &vec!["tan".to_string(), "nat".to_string()]));
  }
}

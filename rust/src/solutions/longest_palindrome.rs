pub fn longest_palindrome(s: String) -> String {
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

#[test]
fn test_longest_palindrome() {
  assert_eq!(longest_palindrome("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone".to_string()), "wfdfw".to_string());
  assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
  assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
  assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
}

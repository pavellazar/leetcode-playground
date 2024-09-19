pub fn longest_palindrome(s: String) -> String {
    let mut max_len = 0;
    let mut max_str = String::new();
    let mut i = 0;
    let mut j;
    let s_len = s.len();
    while i < s_len {
        j = i;
        while j < s_len {
            if s[i..j+1] == s[i..j+1].chars().rev().collect::<String>() {
                if (j - i + 1) > max_len {
                    max_len = j - i + 1;
                    max_str = s[i..j+1].to_string();
                }
            }
            j += 1;
        }
        i += 1;
    }
    max_str        
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone".to_string()), "wfdfw".to_string());
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
}
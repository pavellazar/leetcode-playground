pub fn sort_string(s: &str) -> String {
  let mut chars: Vec<char> = s.chars().collect();
  chars.sort_unstable();
  chars.into_iter().collect()
}

pub fn signature(s: &str) -> [u8; 26] {
  let mut freq = [0u8; 26];
  for b in s.bytes() {
    freq[(b - b'a') as usize] += 1;
  }
  freq
}

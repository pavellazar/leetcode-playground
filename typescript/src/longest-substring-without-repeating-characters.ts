export const lengthOfLongestSubstring = (s: string): number => {
  let longest = 0;
  let start = 0;
  let end = 0;
  for (let i = 0; i < s.length; i++) {
    const char = s[i];
    const index = s.indexOf(char, start);
    if (index >= start && index < end) {
      start = index + 1;
    }
    end = i + 1;
    longest = Math.max(longest, end - start);
  }
  return longest;
};

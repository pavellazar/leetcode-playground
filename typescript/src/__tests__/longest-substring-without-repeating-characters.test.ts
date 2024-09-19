import { lengthOfLongestSubstring } from '../longest-substring-without-repeating-characters';

describe('longest-substring-without-repeating-characters', () => {
  it('should return the length of the longest substring without repeating characters', () => {
    expect(lengthOfLongestSubstring('abcabcbb')).toEqual(3);
    expect(lengthOfLongestSubstring('bbbbb')).toEqual(1);
    expect(lengthOfLongestSubstring('pwwkew')).toEqual(3);
    expect(lengthOfLongestSubstring('dvdf')).toEqual(3);
  });
});

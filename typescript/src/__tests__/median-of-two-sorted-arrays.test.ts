import { findMedianSortedArrays } from '../median-of-two-sorted-arrays';

describe("median-of-two-sorted-arrays", () => {
  it("should find the median of two sorted arrays", () => {
    expect(findMedianSortedArrays([1, 3], [2])).toEqual(2);
    expect(findMedianSortedArrays([1, 2], [3, 4])).toEqual(2.5);
    expect(findMedianSortedArrays([0, 0], [0, 0])).toEqual(0);
    expect(findMedianSortedArrays([], [1])).toEqual(1);
  });
});

export const findMedianSortedArrays = (
  left: number[],
  right: number[]
): number => {
  const totalLength = left.length + right.length;
  if (totalLength % 2 === 1) {
    return findKthElement(left, right, Math.floor(totalLength / 2) + 1);
  } else {
    return (
      (findKthElement(left, right, totalLength / 2) +
        findKthElement(left, right, totalLength / 2 + 1)) /
      2
    );
  }
};

const findKthElement = (
  left: number[],
  right: number[],
  k: number
): number => {
  let leftIdx = 0;
  let rightIdx = 0;

  while (true) {
    // If left is exhausted, return the k-th number in right
    if (leftIdx === left.length) {
      return right[rightIdx + k - 1];
    }
    // If right is exhausted, return the k-th number in left
    if (rightIdx === right.length) {
      return left[leftIdx + k - 1];
    }
    // If k is 1, return the smaller number among the first number of left and the first number of right
    if (k === 1) {
      return Math.min(left[leftIdx], right[rightIdx]);
    }

    // Check the half k-th number in left and right
    const half = Math.floor(k / 2);
    const newLeftIdx = Math.min(leftIdx + half, left.length) - 1;
    const newRightIdx = Math.min(rightIdx + half, right.length) - 1;
    const leftPivot = left[newLeftIdx];
    const rightPivot = right[newRightIdx];
    if (leftPivot <= rightPivot) {
      k -= newLeftIdx - leftIdx + 1;
      leftIdx = newLeftIdx + 1;
    } else {
      k -= newRightIdx - rightIdx + 1;
      rightIdx = newRightIdx + 1;
    }
  }
};

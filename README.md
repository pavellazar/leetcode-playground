# LeetCode Playground

This repository contains my solutions to LeetCode problems. I will be adding more solutions as I solve more problems.

---

## Arrays & Hashing

### 1. Two Sum

Uses a hash map to subtract the remainder in order to find the solutions.

### 26. Remove Duplicates from Sorted Array

Uses two pointers to overwrite duplicates in a sorted array.

### 53. Maximum Subarray

Uses Kadane's algorithm to find the contiguous subarray with the maximum sum.

Kadane's Algorithm  

> 1. Initialize two variables: `max_current` and `max_global` to the first element of the array.  
> 2. Iterate through the array starting from the second element.  
> 3. For each element, update `max_current` to be the maximum of the current element and the sum of `max_current` and the current element.  
> 4. If `max_current` is greater than `max_global`, update `max_global`.  
> 5. After iterating through the array, `max_global` will contain the maximum subarray sum.

### 238. Product of Array Except Self

### 283. Move Zeroes

### 287. Find the Duplicate Number

### 349. Intersection of Two Arrays

### 560. Subarray Sum Equals K

### 1060. Find Missing Number in a Sorted Array

Use binary search to find the missing number in a sorted array of unique integers. The array contains numbers from 0 to n, with one number missing. Tests binary search and edge case handling.

---

## Sliding Window

### 3. Longest Substring Without Repeating Characters

Uses a sliding window with a hash set to track the longest substring without duplicates.

### 13. Sliding Window

### 76. Minimum Window Substring

### 239. Sliding Window Maximum

### 424. Longest Repeating Character Replacement

### 438. Find All Anagrams in a String

Solution: Use a sliding window and substract and add the characters from sliding hash to be compared with anagram hash

---

## Two Pointers

### 11. Container With Most Water

Uses two pointers to maximize area between heights in an array.

### 15. 3Sum

Uses two pointers with a sorted array to find triplets summing to zero.

---

## Sorting

### 4. Median of Two Sorted Arrays

Merges two sorted arrays and finds the median by tracking middle elements.

### 27. Sort Colors

Sort an array of 0s, 1s, and 2s. Tests Dutch national flag algorithm.

### 253. Meeting Rooms II

Find the minimum number of meeting rooms needed. Tests interval sorting.

---

## Stack & Monotonic Stack

### 16. Daily Temperatures (LeetCode #739)

Find the number of days until a warmer day. Tests monotonic stack.

### 20. Valid Parentheses

Uses a stack to check if parentheses are balanced and properly nested.

### 23. Next Greater Element I (LeetCode #496)

Find the next greater element for each element in an array. Tests monotonic stack.

### 42. Trapping Rain Water

Calculate the amount of water trapped between bars. Tests stack or two-pointer approaches.

---

## Heap / Priority Queue

### 23. Merge k Sorted Lists

Uses a priority queue to merge k sorted linked lists into one sorted list.

### 215. Kth Largest Element in an Array

### 295. Find Median from Data Stream

Uses a max heap and a min heap to maintain the median of a data stream in O(log n) time for insertion and O(1) time for finding the median.

### 347. Top K Frequent Elements

### 621. Task Scheduler

Schedule tasks with a cooldown period. Common for greedy and heap usage.

---

## Greedy

### 55. Jump Game

Uses a greedy approach to determine if you can reach the last index of an array by checking the maximum reachable index at each step.

### 121. Best Time to Buy and Sell Stock

### 169. Majority Element

---

## Dynamic Programming

### 5. Longest Palindromic Substring

Expands around centers to find the longest substring that reads the same forward and backward.

### 10. Regular Expression Matching

Uses dynamic programming to match a string against a pattern with wildcards.

### 62. Unique Paths

Uses dynamic programming to count the number of unique paths from the top-left corner to the bottom-right corner of a grid.

### 64. Minimum Path Sum

Uses dynamic programming to find the minimum path sum from the top-left to the bottom-right of a grid by accumulating the minimum sums from adjacent cells.

### 70. Climbing Stairs

Uses dynamic programming to count the number of distinct ways to reach the nth step by summing the ways to reach the previous two steps.

### 198. House Robber

### 322. Coin Change

Uses dynamic programming to find the minimum number of coins needed to make a given amount.

### 416. Partition Equal Subset Sum

### 518. Coin Change II

Uses dynamic programming to find the number of combinations to make a given amount with a set of coins.

### 647. Palindromic Substrings

### 337. House Robber III

---

## Backtracking & Recursion

### 28. Subsets (LeetCode #78)

Generate all subsets of a set. Tests recursive backtracking.

### 39. Combination Sum

### 46. Permutations

Uses backtracking to generate all permutations of a list of numbers.

### 51. N-Queens

Place N queens on an NxN board. Classic recursion and backtracking problem.

### 78. Subsets

### 79. Word Search

---

## Divide and Conquer

### 24. Construct Binary Tree from Preorder and Inorder Traversal (LeetCode #105)

Build a binary tree from traversals. Tests recursive splitting.

### 273. Integer to English Words

Divide and conquer approach to convert an integer to English words by breaking it down into thousands, hundreds, tens, and units.

---

## Binary Search

### 33. Search in Rotated Sorted Array

Uses binary search with left and right pointers to find an element in a rotated sorted array.

### 34. Find First and Last Position of Element in Sorted Array

### 153. Find Minimum in Rotated Sorted Array

### 704. Binary Search

---

## Linked List

### 2. Add Two Numbers

Iterates through two linked lists, adding digits and carrying over to sum them.

### 19. Remove Nth Node From End of List

Uses two pointers with a gap of n to remove the nth node from the end.

### 21. Merge Two Sorted Lists

Uses a dummy node to merge two sorted linked lists into one sorted list.

### 141. Linked List Cycle

### 142. Linked List Cycle II

### 148. Sort List

### 206. Reverse Linked List

### 234. Palindrome Linked List

### 328. Odd Even Linked List

### 445. Add Two Numbers II

### 876. Middle of the Linked List

Use a slow and fast pointer to find the middle node of a linked list.

---

## Tree / Binary Tree / BST

### 94. Binary Tree Inorder Traversal

### 98. Validate Binary Search Tree

### 100. Same Tree

### 101. Symmetric Tree

### 102. Binary Tree Level Order Traversal

### 103. Binary Tree Zigzag Level Order Traversal

### 104. Maximum Depth of Binary Tree

### 105. Construct Binary Tree from Preorder and Inorder Traversal

### 108. Convert Sorted Array to Binary Search Tree

### 110. Balanced Binary Tree

### 112. Path Sum

### 113. Path Sum II

### 114. Flatten Binary Tree to Linked List

### 124. Binary Tree Maximum Path Sum

### 173. Binary Search Tree Iterator

### 230. Kth Smallest Element in a BST

### 236. Lowest Common Ancestor of a Binary Tree

Recursively finds the lowest common ancestor of two nodes in a binary tree by checking if the current node is one of the nodes or if both nodes are in different subtrees.

### 297. Serialize and Deserialize Binary Tree

### 543. Diameter of Binary Tree

### 572. Subtree of Another Tree

### 617. Merge Two Binary Trees

---

## Trie

### 208. Implement Trie (Prefix Tree)

Implement a trie with insert, search, and startsWith methods. Core trie problem.

### 212. Word Search II

Find all words in a board that exist in a dictionary. Tests trie with DFS.

---

## Graphs & Union-Find

### 200. Number of Islands

### 207. Course Schedule

### 210. Course Schedule II

### 547. Friend Circles

Find the number of friend circles in a matrix. Tests union-find basics.

### 684. Redundant Connection

Find an edge that can be removed to make a graph a tree. Tests cycle detection.

---

## Intervals

### 56. Merge Intervals

Sorts intervals by start time and merges overlapping intervals into a single interval.

### 57. Insert Interval

Insert a new interval into a list of non-overlapping intervals. Tests interval merging.

---

## Matrix

### 48. Rotate Image

Rotates a 2D matrix by transposing it and then reversing each row.

### 54. Spiral Matrix

Uses a layer-by-layer approach to traverse a 2D matrix in spiral order.

### 73. Set Matrix Zeroes

### 75. Sort Colors

---

## Bit Manipulation

### 136. Single Number

Find the number that appears once in an array. Tests XOR operations.

### 191. Number of 1 Bits

Count the number of 1s in a binary number. Tests bit manipulation basics.

---

## Math

### 50. Pow(x, n)

Implement power function with exponentiation. Tests fast exponentiation.

---

## Design

### 146. LRU Cache

### 155. Min Stack

### 355. Design Twitter

Design a simplified Twitter with follow and newsfeed features. Tests system design in coding.

---

## Queue

### 622. Design Circular Queue

Implement a circular queue. Tests queue implementation and edge cases.

---

## Advanced Data Structures

### 307. Range Sum Query - Mutable

Update and query sum of a range in an array. Tests binary indexed tree.

### 315. Count of Smaller Numbers After Self

Count smaller numbers to the right. Tests segment tree or BIT.

### 362. Design Hit Counter

Design a hit counter to record hits in the last 5 minutes. Tests queue or array with timestamps.

---

## System Design

### 1603. Design a Parking Lot

Implement a parking lot system to manage vehicle parking and availability. Tests object-oriented design and system thinking.

### 359. Logger Rate Limiter

Design a logger that limits message printing based on timestamps. Tests system design with time-based constraints.

---

## Concurrency

### 1114. Print in Order

Ensure three threads print in a specific order using synchronization primitives (e.g., locks, semaphores). Tests thread coordination.

### 1188. Design Bounded Blocking Queue

Implement a thread-safe bounded queue for producer-consumer scenarios. Tests synchronization and queue design.

---

## String Parsing / Processing

### 71. Simplify Path

Simplify a Unix-style file path (e.g., "/a/./b/../../c/" to "/c"). Tests string parsing and stack-based processing.

### 394. Decode String

Decode a string with nested brackets (e.g., "3[a]2[bc]" to "aaabcbc"). Tests stack and recursive parsing.

### 125. Valid Palindrome

### 242. Valid Anagram

---

## Miscellaneous

### 12. Integer to Roman

Converts an integer to Roman numerals using a mapping of values to symbols.

### 13. Roman to Integer

Sums values of Roman numeral characters, adjusting for subtractive cases.

### 14. Longest Common Prefix

Compares characters across strings to find the longest shared prefix.

### 52. N-Queens

### 128. Longest Consecutive Sequence

### 150. Evaluate Reverse Polish Notation

### 152. Maximum Product Subarray

### 160. Intersection of Two Linked Lists

### 189. Rotate Array

### 199. Binary Tree Right Side View

### 252. Meeting Rooms

### 268. Missing Number

### 300. Longest Increasing Subsequence

### 344. Reverse String

### 412. Fizz Bizz

---

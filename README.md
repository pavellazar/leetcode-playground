# LeetCode Playground

This repository contains my solutions to LeetCode problems. I will be adding more solutions as I solve more problems.

## Arrays & Hashing

This category focuses on problems that leverage arrays and hash-based data structures (e.g., hash maps, hash sets) to optimize lookups, store frequency counts, or handle data efficiently.

### 1. Two Sum

[DONE] Uses a hash map to subtract the remainder in order to find the solutions.

### 49. Group Anagrams

[DONE] Groups anagrams by sorting each string and using a hash map to collect them.

### 128. Longest Consecutive Sequence

[DONE] Uses a hash set to track consecutive sequences by checking for the start of each sequence and counting its length.

### 1742. Maximum number of Balls in a Box

[DONE] Counts the maximum number of balls in a box based on their colors and positions. Uses a hash map to track counts.

### 1773. Count Items Matching a Rule

[DONE] Counts items that match a given rule using a hash map to filter based on criteria.

### 1774. Closest Dessert Cost

[DONE] Finds the closest dessert cost to a target by exploring combinations of two dessert costs. Uses a set to track possible costs and find the closest one.

### 238. Product of Array Except Self

[DONE] Calculates the product of all elements except self using two arrays to store left and right products, avoiding division.

### 287. Find the Duplicate Number

[DONE] Uses a hash set to track seen numbers and find the first duplicate in an array.

### 349. Intersection of Two Arrays

[DONE] Uses hash sets to find the intersection of two arrays, returning unique elements.

## Two Pointers

This category involves problems solved using two pointers to traverse arrays or lists, often to optimize solutions by reducing time complexity or to handle sorted data efficiently.

### 11. Container With Most Water

[DONE] Uses two pointers to maximize area between heights in an array.

### 15. 3Sum

[DONE] Uses two pointers with a sorted array to find triplets summing to zero.

### 26. Remove Duplicates from Sorted Array

[DONE] Uses two pointers to overwrite duplicates in a sorted array.

### 283. Move Zeroes

[DONE] Uses two pointers to move all zeroes to the end of an array while maintaining order.

## Sliding Window

Sliding window techniques are used for problems involving contiguous subarrays or substrings, maintaining a "window" that expands or contracts to meet conditions.

### 3. Longest Substring Without Repeating Characters

[DONE] Uses a sliding window with a hash set to track the longest substring without duplicates.

### 76. Minimum Window Substring

[DONE] Uses a sliding window to find the minimum substring containing all characters of a target string.

### 239. Sliding Window Maximum

[DONE] Uses a deque to maintain the maximum values in a sliding window over an array.

### 424. Longest Repeating Character Replacement

[DONE] Uses a sliding window to find the longest substring with at most k replacements of characters.

### 438. Find All Anagrams in a String

[DONE] Solution: Use a sliding window and subtract and add the characters from sliding hash to be compared with anagram hash.

## Dynamic Programming

Dynamic programming solves problems by breaking them into overlapping subproblems and storing results to avoid redundant computations, often used for optimization problems.

### 5. Longest Palindromic Substring

[DONE] Expands around centers to find the longest substring that reads the same forward and backward.

### 10. Regular Expression Matching

[DONE] Uses dynamic programming to match a string against a pattern with wildcards.

### 53. Maximum Subarray

[DONE] Uses Kadane's algorithm to find the contiguous subarray with the maximum sum.

Kadane's Algorithm  

> 1. Initialize two variables: `max_current` and `max_global` to the first element of the array.  
> 2. Iterate through the array starting from the second element.  
> 3. For each element, update `max_current` to be the maximum of the current element and the sum of `max_current` and the current element.  
> 4. If `max_current` is greater than `max_global`, update `max_global`.  
> 5. After iterating through the array, `max_global` will contain the maximum subarray sum.

### 62. Unique Paths

[DONE] Uses dynamic programming to count the number of unique paths from the top-left corner to the bottom-right corner of a grid.

### 64. Minimum Path Sum

[DONE] Uses dynamic programming to find the minimum path sum from the top-left to the bottom-right of a grid by accumulating the minimum sums from adjacent cells.

### 70. Climbing Stairs

[DONE] Uses dynamic programming to count the number of distinct ways to reach the nth step by summing the ways to reach the previous two steps.

### 152. Maximum Product Subarray

[DONE] Uses dynamic programming to find the contiguous subarray with the maximum product by tracking both maximum and minimum products at each step.

### 198. House Robber

[DONE] Uses dynamic programming to determine the maximum amount of money that can be robbed from a series of houses without robbing two adjacent houses.

### 300. Longest Increasing Subsequence

[DONE] Uses dynamic programming to find the length of the longest increasing subsequence in an array by maintaining an array of lengths of increasing subsequences.

### 322. Coin Change

[DONE] Uses dynamic programming to find the minimum number of coins needed to make a given amount.

### 416. Partition Equal Subset Sum

[DONE] Uses dynamic programming to determine if a set can be partitioned into two subsets with equal sum by checking if a subset with half the total sum exists.

### 518. Coin Change II

[DONE] Uses dynamic programming to find the number of combinations to make a given amount with a set of coins.

### 560. Subarray Sum Equals K

[DONE] Uses a hash map to count the number of subarrays that sum to a given value k by tracking cumulative sums.

### 647. Palindromic Substrings

[DONE] Uses dynamic programming to count all palindromic substrings in a string by expanding around centers and checking for palindromes.

### 337. House Robber III

[DONE] Uses dynamic programming with recursion to find the maximum amount of money that can be robbed from a binary tree of houses, ensuring no two adjacent houses (parent-child) are robbed.

## Backtracking & Recursion

Backtracking involves exploring all possible solutions by building candidates incrementally and abandoning invalid paths, often used for combinatorial problems.

### 39. Combination Sum

[DONE] Uses backtracking to find all unique combinations of numbers that sum to a target value, allowing for repeated use of numbers.

### 46. Permutations

[DONE] Uses backtracking to generate all permutations of a list of numbers.

### 51. N-Queens

[DONE] Place N queens on an NxN board. Classic recursion and backtracking problem.

### 52. N-Queens II

[DONE] Counts the number of distinct solutions for the N-Queens problem. Tests backtracking and recursion.

### 78. Subsets

[DONE] Generate all subsets of a set. Tests recursive backtracking.

### 79. Word Search

[DONE] Uses backtracking to search for a word in a 2D board by exploring all possible paths.

## Sorting

Sorting problems involve arranging data to enable efficient processing, often as a preprocessing step for other algorithms like binary search or two pointers.

### 4. Median of Two Sorted Arrays

[DONE] Merges two sorted arrays and finds the median by tracking middle elements.

### 75. Sort Colors

[DONE] Sorts an array of colors (0s, 1s, 2s) using a three-pointer approach (Dutch National Flag problem).

### 2402. Meeting Rooms III

[DONE] Sorts meeting intervals by start time and end time to determine the maximum number of meeting rooms required at any time.

## Stack & Monotonic Stack

Stack-based solutions are used for problems requiring last-in-first-out processing, while monotonic stacks maintain elements in a sorted order for efficient lookups.

### 20. Valid Parentheses

[DONE] Uses a stack to check if parentheses are balanced and properly nested.

### 42. Trapping Rain Water

[DONE] Uses a stack to calculate the amount of water trapped between heights in an array.

### 496. Next Greater Element I

[DONE] Uses a stack to find the next greater element for each element in an array, returning results based on indices.

### 739. Daily Temperatures

[DONE] Uses a monotonic stack to find the number of days until a warmer temperature for each day in an array of temperatures.

## Heap / Priority Queue

Heap-based solutions use priority queues to efficiently manage elements based on priority, often for problems requiring top-k elements or scheduling.

### 23. Merge k Sorted Lists

[DONE] Uses a priority queue to merge k sorted linked lists into one sorted list.

### 215. Kth Largest Element in an Array

[DONE] Uses a min heap to find the kth largest element in an array by maintaining a heap of size k and returning the root.

### 295. Find Median from Data Stream

[DONE] Uses a max heap and a min heap to maintain the median of a data stream in O(log n) time for insertion and O(1) time for finding the median.

### 347. Top K Frequent Elements

[DONE] Uses a priority queue to find the k most frequent elements in an array by counting frequencies and maintaining a heap of size k.

### 621. Task Scheduler

[DONE] Uses a priority queue to schedule tasks with cooldown periods, ensuring tasks are executed in the order of their frequencies.

## Greedy

Greedy algorithms make locally optimal choices at each step to achieve a globally optimal solution, often for problems involving resource allocation or scheduling.

### 55. Jump Game

[DONE] Uses a greedy approach to determine if you can reach the last index of an array by checking the maximum reachable index at each step.

### 121. Best Time to Buy and Sell Stock

### 169. Majority Element

## Divide and Conquer

Divide and conquer breaks problems into smaller subproblems, solves them recursively, and combines results, often used for tree or array-based problems.

### 105. Construct Binary Tree from Preorder and Inorder Traversal

Build a binary tree from traversals. Tests recursive splitting.

### 273. Integer to English Words

[DONE] Divide and conquer approach to convert an integer to English words by breaking it down into thousands, hundreds, tens, and units.

## Binary Search

Binary search efficiently finds elements or boundaries in sorted data by repeatedly dividing the search space in half.

### 33. Search in Rotated Sorted Array

[DONE] Uses binary search with left and right pointers to find an element in a rotated sorted array.

### 34. Find First and Last Position of Element in Sorted Array

### 153. Find Minimum in Rotated Sorted Array

### 704. Binary Search

### 1060. Find Missing Number in a Sorted Array

Use binary search to find the missing number in a sorted array of unique integers. The array contains numbers from 0 to n, with one number missing. Tests binary search and edge case handling.

## Linked List

Linked list problems involve manipulating or traversing singly or doubly linked lists, often requiring pointer manipulation or dummy nodes.

### 2. Add Two Numbers

[DONE] Iterates through two linked lists, adding digits and carrying over to sum them.

### 19. Remove Nth Node From End of List

[DONE] Uses two pointers with a gap of n to remove the nth node from the end.

### 21. Merge Two Sorted Lists

[DONE] Uses a dummy node to merge two sorted linked lists into one sorted list.

### 141. Linked List Cycle

### 142. Linked List Cycle II

### 148. Sort List

### 160. Intersection of Two Linked Lists

### 206. Reverse Linked List

[DONE] Uses a three-pointer approach to reverse a singly linked list in place.

### 234. Palindrome Linked List

### 328. Odd Even Linked List

### 445. Add Two Numbers II

### 876. Middle of the Linked List

[DONE] Use a slow and fast pointer to find the middle node of a linked list.

## Tree / Binary Tree / BST

Tree problems involve traversing or manipulating binary trees or binary search trees, often using recursion or iterative approaches with stacks/queues.

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

[DONE] Recursively finds the lowest common ancestor of two nodes in a binary tree by checking if the current node is one of the nodes or if both nodes are in different subtrees.

### 297. Serialize and Deserialize Binary Tree

### 543. Diameter of Binary Tree

### 572. Subtree of Another Tree

### 617. Merge Two Binary Trees

### 199. Binary Tree Right Side View

## Trie

Trie (prefix tree) problems involve storing and retrieving strings efficiently, often for prefix-based or dictionary-based searches.

### 208. Implement Trie (Prefix Tree)

Implement a trie with insert, search, and startsWith methods. Core trie problem.

### 212. Word Search II

Find all words in a board that exist in a dictionary. Tests trie with DFS.

## Graphs & Union-Find

Graph problems involve traversing graphs (DFS/BFS) or detecting cycles, while union-find is used for connectivity or grouping problems.

### 200. Number of Islands

[DONE] Uses DFS to count connected components in a grid representing islands.

### 207. Course Schedule

[DONE] Uses topological sorting to determine if all courses can be finished given prerequisites, testing cycle detection in directed graphs.

### 210. Course Schedule II

[DONE] Returns the order of courses to take based on prerequisites using topological sorting.

### 547. Number of Provinces

[DONE] Uses DFS to count connected components in a graph representing provinces.

### 684. Redundant Connection

[DONE] To solve it uses union-find to detect cycles in a graph, identifying the redundant edge that creates a cycle.

## Intervals

Interval problems involve manipulating or merging time or range-based intervals, often requiring sorting as a preprocessing step.

### 56. Merge Intervals

[DONE] Sorts intervals by start time and merges overlapping intervals into a single interval.

### 57. Insert Interval

### 252. Meeting Rooms

## Matrix

Matrix problems involve manipulating 2D arrays, often requiring traversal in specific patterns or in-place modifications.

### 48. Rotate Image

[DONE] Rotates a 2D matrix by transposing it and then reversing each row.

### 54. Spiral Matrix

[DONE] Uses a layer-by-layer approach to traverse a 2D matrix in spiral order.

### 73. Set Matrix Zeroes

## Bit Manipulation

Bit manipulation problems use bitwise operations (e.g., XOR, AND, OR) to solve problems efficiently, often for low-level data processing.

### 136. Single Number

Find the number that appears once in an array. Tests XOR operations.

### 191. Number of 1 Bits

Count the number of 1s in a binary number. Tests bit manipulation basics.

## Math

Math problems involve numerical computations, number theory, or mathematical properties to derive solutions.

### 12. Integer to Roman

[DONE] Converts an integer to Roman numerals using a mapping of values to symbols.

### 13. Roman to Integer

[DONE] Sums values of Roman numeral characters, adjusting for subtractive cases.

### 50. Pow(x, n)

[DONE] Implement power function with exponentiation. Tests fast exponentiation.

### 268. Missing Number

## String Parsing / Processing

String problems involve parsing, manipulating, or validating strings, often using stacks, pointers, or character frequency counts.

### 6. Zig Zag Conversion

[DONE] Converts a string to a zigzag pattern based on a given number of rows and reads it row by row.

### 7. Reverse Integer

[DONE] Reverses the digits of an integer while handling overflow and negative signs.

### 8. String to Integer

[DONE] Converts a string to an integer while handling leading/trailing spaces, signs, and overflow.

### 9. Palindrome Number

[DONE] Checks if an integer is a palindrome by comparing digits from both ends.

### 14. Longest Common Prefix

[DONE] Compares characters across strings to find the longest shared prefix.

### 71. Simplify Path

Simplify a Unix-style file path (e.g., "/a/./b/../../c/" to "/c"). Tests string parsing and stack-based processing.

### 125. Valid Palindrome

### 242. Valid Anagram

### 394. Decode String

Decode a string with nested brackets (e.g., "3[a]2[bc]" to "aaabcbc"). Tests stack and recursive parsing.

## Design

Design problems focus on implementing data structures or systems with specific functionality, often simulating real-world applications.

### 146. LRU Cache

### 155. Min Stack

### 355. Design Twitter

Design a simplified Twitter with follow and newsfeed features. Tests system design in coding.

### 1603. Design a Parking Lot

Implement a parking lot system to manage vehicle parking and availability. Tests object-oriented design and system thinking.

### 359. Logger Rate Limiter

Design a logger that limits message printing based on timestamps. Tests system design with time-based constraints.

## Queue

Queue problems involve implementing or using FIFO data structures, often for scheduling or buffering tasks.

### 622. Design Circular Queue

Implement a circular queue. Tests queue implementation and edge cases.

## Advanced Data Structures

These problems involve complex data structures like segment trees or binary indexed trees for efficient range queries or updates.

### 307. Range Sum Query - Mutable

Update and query sum of a range in an array. Tests binary indexed tree.

### 315. Count of Smaller Numbers After Self

Count smaller numbers to the right. Tests segment tree or BIT.

### 362. Design Hit Counter

Design a hit counter to record hits in the last 5 minutes. Tests queue or array with timestamps.

## System Design & Concurrency

- System design problems focus on designing scalable systems or components, often requiring object-oriented or time-based logic.
- Concurrency problems involve managing multiple threads or processes, ensuring proper synchronization and order of execution.

### 1114. Print in Order

Ensure three threads print in a specific order using synchronization primitives (e.g., locks, semaphores). Tests thread coordination.

### 1188. Design Bounded Blocking Queue

Implement a thread-safe bounded queue for producer-consumer scenarios. Tests synchronization and queue design.

## Miscellaneous

This category includes problems that don't fit neatly into other categories, often combining multiple techniques or unique approaches.

### 150. Evaluate Reverse Polish Notation

[DONE] Evaluates expressions in Reverse Polish Notation using a stack to handle operands and operators.

### 189. Rotate Array

[DONE] Rotates an array in place by reversing segments of the array.

### 344. Reverse String

[DONE] Reverses a string in place using two pointers.

### 412. Fizz Buzz

[DONE] Generates a FizzBuzz sequence based on divisibility rules for 3 and 5.

# 321. Create Maximum Number

### Difficulty: Hard

### Description

You are given two integer arrays nums1 and nums2 of lengths m and n respectively. nums1 and nums2 represent the digits of two numbers. You are also given an integer k.

Create the maximum number of length k <= m + n from digits of the two numbers. The relative order of the digits from the same array must be preserved.

Return an array of the k digits representing the answer.

### Examples

Example 1:

```rs
Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
Output: [9,8,6,5,3]
```

Example 2:

```rs
Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
Output: [6,7,6,0,4]
```

Example 3:

```rs
Input: nums1 = [3,9], nums2 = [8,9], k = 3
Output: [9,8,9]
```

### Constraints:

- `m == nums1.length`
- `n == nums2.length`
- `1 <= m, n <= 500`
- `0 <= nums1[i], nums2[i] <= 9`
- `1 <= k <= m + n`


---

### Explanation

This is a divide and conquer problem, the problem can be divided into two subproblems, each:
- Find the maximum number of length x from an array
- Find the maximum number of length y = (k - x) from the other array

Then these two subproblems can be merged to get the maximum number of length k.

A overview of the solution is as follows:
1. Find a subarray of length x from nums1 such that its the maximum number possible (of length x).
2. Find the maximum number of length (k - x) forming the largest number from nums2.
3. Merge two arrays to get a new array of length k
4. If the merged array's length higher than the current max's length, update the max.
5. If the merged array's length is equal to the current max's length, compare the two arrays and update the max if the new array is larger.
6. Repeat the above steps for all possible values of x from 0 to k.

Further details can be found in the code comments.

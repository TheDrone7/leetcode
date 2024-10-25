# 1590. Make Sum Divisible by P

### Difficulty: Medium

### Description

Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the remaining elements is divisible by p. It is not allowed to remove the whole array.

Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.

A subarray is defined as a contiguous block of elements in the array.

### Examples

Example 1:

```rs
Input: nums = [3,1,4,2], p = 6
Output: 1
```

Example 2:

```rs
Input: nums = [6,3,5,2], p = 9
Output: 2
```

Example 3:

```rs
Input: nums = [1,2,3], p = 3
Output: 0
```

### Constraints:


- `1 <= nums.length <= 105`
- `1 <= nums[i] <= 109`
- `1 <= p <= 109`

---

### Explanation

The problem can be solved using a hashmap and prefix sum.

The steps are as follows:

- Calculate the sum of the array.
- Calculate the remainder of the sum when divided by p.
- If the sum is divisible by p, return 0.
- Otherwise, loop over the array.
- Keep track of the sum of elements (mod p).
- Add the sum to a hashmap
  - key: sum, value: index (at which the key was the sum)
- Calculate the difference between sum and remainder.
- If the difference is in the hashmap
  - The subarray between the index and the current index is a valid subarray
  - Check the length of the subarray,
  - Update the minimum to the minimum of the current minimum and the length of the subarray.
- If the result is unchanged, there is no valid subarray, return -1.
- Otherwise, return the minimum.

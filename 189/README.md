# 189. Rotate Array

### Difficulty: Medium

### Description

Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

### Examples

Example 1:

```rs
Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
```

Example 2:

```rs
Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
```

### Constraints:

- `1 <= nums.length <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`

---

### Explanation

This problem can be solved by reversing the array in two steps.

First, we need to avoid unnecessary rotations.
Since the array is cyclic, we can rotate the array by `k % n` steps where `n` is the length of the array.

After that, we can reverse the whole array.
Then, we can reverse the first `k` elements and the last `n - k` elements separately.

By doing this, we can rotate the array to the right by `k` steps.

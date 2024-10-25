# 16. 3Sum Closest

### Difficulty: Medium

### Description

Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.

### Examples

Example 1:

```rs
Input: nums = [-1,2,1,-4], target = 1
Output: 2
```

Example 2:

```rs
Input: nums = [0,0,0], target = 1
Output: 0
```

### Constraints

- `3 <= nums.length <= 500`
- `-10^3 <= nums[i] <= 10^3`
- `-10^4 <= target <= 10^4`

---

### Explanation

We solve this problem by using the two-pointer technique.

First sort the array, then iterate through the array.

Also set the `closest` variable to a high value.
A simple way to do this would be to set it to `i32::MAX`.
Or we can set it to the sum of the first three elements in the array.

For each element, we set two pointers, `left`, and `right`.
- `left` is the element after the current element.
- `right` is the last element in the array.

We calculate the sum of the three elements and compare it with the target.

If the difference between the new sum and target is less than the difference
between the current `closest` and target, we update the `closest` variable.

- If the new sum is greater than the target, we decrement the `right` pointer (decrease the value of the sum).
- If the new sum is less than the target, we increment the `left` pointer (increase the value of the sum).
- If the new sum is equal to the target, we return the sum (perfect match).

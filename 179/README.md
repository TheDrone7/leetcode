# 179. Largest Number

### Difficulty: Medium

### Description

Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.

Since the result may be very large, so you need to return a string instead of an integer.

### Examples

Example 1:

```rs
Input: nums = [10,2]
Output: "210"
```

Example 2:

```rs
Input: nums = [3,30,34,5,9]
Output: "9534330"
```

### Constraints:

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 10^9`

---

### Explanation

This problem can be simply solved by sorting the numbers in a way that the concatenation of the numbers will be the largest.
The key is to compare the concatenation of two numbers in two different ways.
For example, if we have two numbers `10` and `2`, we can compare them by comparing `210` and `102`.
If `210` is larger than `102`, then `2` should be placed before `10`.
Otherwise, `10` should be placed before `2`.
By doing this, we can sort the numbers in a way that the concatenation of the numbers will be the largest.

After sorting the numbers, we can simply concatenate the numbers to get the largest number. The time complexity of this approach is `O(nlogn)` where `n` is the number of elements in the list. The space complexity of this approach is `O(n)` where `n` is the number of elements in the list.

Just as a special case, if the first element of the sorted list is `0`,
it means that the list only has `0`s, if that were not the case,
`0` suffixed to any number will be larger than `0` as a prefix.
In this case, we can simply return `0` as the result.

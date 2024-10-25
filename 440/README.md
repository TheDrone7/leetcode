# 440. K-th Smallest in Lexicographical Order

### Difficulty: Hard

### Description

Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].F

### Examples

Example 1:

```rs
Input: n = 13, k = 2
Output: 10
```

Example 2:

```rs
Input: n = 1, k = 1
Output: 1
```

### Constraints

- `1 <= k <= n <= 10^9`

---

### Explanation

We first initialize the current number "res" to 1.
And also reduce k by 1 (since 1 is the smallest number)

Then, we loop while k is greater than 0.
In each iteration
- count the number of numbers with the prefix "res" (i.e. the number of numbers that are lexicographically larger than res but less than n)
- if the count is less than k, we increment the prefix
  - res += 1 (increment the prefix)
  - k -= count (reduce k by the count)
- if count is greater than k, the answer is one of the numbers with the prefix "res"
  - res *= 10 (move to the next digit)
  - k -= 1 (reduce k by 1)

This process will finally give us the kth lexicographically smallest number in the range [1, n]

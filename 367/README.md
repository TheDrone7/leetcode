# 367. Valid Perfect Square

### Difficulty: Easy

### Description

Given a positive integer num, return true if num is a perfect square or false otherwise.

A perfect square is an integer that is the square of an integer. In other words, it is the product of some integer with itself.

You must not use any built-in library function, such as sqrt.

### Examples

Example 1:

```rs
Input: num = 16
Output: true
```

Example 2:

```rs
Input: num = 14
Output: false
```

### Constraints

- `1 <= num <= 2^31 - 1`

---

### Explanation

This one is pretty simple, simply iterate from 1 while `i * i <= num`
and if `i * i == num` return true, otherwise return false.

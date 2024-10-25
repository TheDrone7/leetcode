# 214. Shortest Palindrome

### Difficulty: Hard

### Description

You are given a string s. You can convert s to a palindrome by adding characters in front of it.

Return the shortest palindrome you can find by performing this transformation.

### Examples

Example 1:

```rs
Input: s = "aacecaaa"
Output: "aaacecaaa"
```

Example 2:

```rs
Input: s = "abcd"
Output: "dcbabcd"
```

### Constraints:

- `0 <= s.length <= 5 * 10^4`
- `s` consists of lowercase English letters only.

---

### Explanation

This problem can be solved by using the [KMP (Knuth-Morris-Pratt)](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm) algorithm.
The idea is to find the longest prefix of the string that is also a suffix. This will help us to find the shortest palindrome that can be formed by adding characters in front of the string.

Let's take an example to understand this:

```
s = "abacd"
```

The longest prefix that is also a suffix is `a`. So, the shortest palindrome that can be formed by adding characters in front of the string is `dcabacd`.

The KMP algorithm is used to find the longest prefix that is also a suffix. The algorithm is as follows:

To find the shortest palindrome that can be formed by adding characters in front of the string, we can use the following steps:

1. Reverse the string and append it to the original string with a separator character in between.
2. Find the longest prefix that is also a suffix for the combined string using the KMP algorithm.
3. The length of the longest prefix that is also a suffix will give us the length of the shortest palindrome that can be formed by adding characters in front of the string.
4. Append the remaining characters from the reversed string to the original string to form the shortest palindrome.

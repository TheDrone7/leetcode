# 1233. Remove Sub-Folders from the Filesystem

### Difficulty: Medium

### Description

Given a list of folders folder, return the folders after removing all sub-folders in those folders. You may return the answer in any order.

If a folder[i] is located within another folder[j], it is called a sub-folder of it. A sub-folder of folder[j] must start with folder[j], followed by a "/". For example, "/a/b" is a sub-folder of "/a", but "/b" is not a sub-folder of "/a/b/c".

The format of a path is one or more concatenated strings of the form: '/' followed by one or more lowercase English letters.

For example, "/leetcode" and "/leetcode/problems" are valid paths while an empty string and "/" are not.

### Examples

Example 1:

```rs
Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
Output: ["/a","/c/d","/c/f"]
```

Example 2:

```rs
Input: folder = ["/a","/a/b/c","/a/b/d"]
Output: ["/a"]
```

Example 3:

```rs
Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
Output: ["/a/b/c","/a/b/ca","/a/b/d"]
```

### Constraints:

- `1 <= folder.length <= 4 * 10^4`
- `2 <= folder[i].length <= 100`
- `folder[i]` contains only lowercase letters and '/'
- `folder[i]` always starts with character '/'
- Each folder name is unique

---

### Explanation

This problem can be solved by sorting the folders.
Then, going through the sorted folders, we can check if the current folder is a sub-folder of the previous folder.
If it is not a sub-folder, we can add it to the result.

By doing this, we can remove all sub-folders from the list of folders.

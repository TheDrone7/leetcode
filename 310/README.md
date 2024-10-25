# 310. Minimum Height Trees

### Difficulty: Medium

### Description:

A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.

Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).

Return a list of all MHTs' root labels. You can return the answer in any order.

The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.

### Examples:

Example 1:

```rs
Input: n = 4, edges = [[1,0],[1,2],[1,3]]
Output: [1]
```

Example 2:

```rs
Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
Output: [3,4]
```

### Constraints

- `1 <= n <= 2 * 10^4`
- `edges.length == n - 1`
- `0 <= a(i), b(i) < n`
- `a(i) != b(i)`
- All the pairs `(a(i), b(i))` are distinct.
- The given input is **guaranteed** to be a tree and there will be no **repeated** edges.

---

### Explanation

First, we build a graph using 2 vectors
- `g`: a vector of vectors of integers.
  - An element `g[i]` contains all the nodes that are connected to node `i`.
  - For example, if `g[0] = [1, 2]`, then nodes `0` is connected to nodes `1`, and `2`.
- `d`: A vector of integers.
  - An element `d[i]` contains the number of nodes that are connected to node `i` (degree of `i`).
  - For example, if `d[0] = 2`, then node `0` is connected to `2` nodes.

Next we build a list of leaves in the graph.
Leaves can be identified by the fact that they have a degree of `1`.

We then remove the leaves,
- update degree of neighbors of the leaf nodes
- check for new set of leaves

The "trimming of leaves" is repeated until there are only 1 or 2 nodes left in the graph.

The remaining nodes are the roots of the minimum height trees.

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let res;

        // Build the graph
        let mut g = vec![vec![]; n as usize];
        let mut d = vec![0; n as usize];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;

            g[u].push(v);
            g[v].push(u);

            d[u] += 1;
            d[v] += 1;
        }

        // Find the leaves
        let mut leaves = vec![];

        for i in 0..n as usize {
            if d[i] <= 1 {
                leaves.push(i);
            }
        }

        let mut count = n;

        // Remove leaves level by level
        while count > 2 {
            let mut new_leaves = vec![];
            count -= leaves.len() as i32;

            for leaf in leaves.iter() {
                let leaf = *leaf as usize;
                // Update the degree of the neighbors
                for &neighbor in g[leaf].iter() {
                    d[neighbor] -= 1;
                    // Check if the neighbor becomes a leaf
                    if d[neighbor] == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }

            leaves = new_leaves;
        }

        res = leaves.iter().map(|&x| x as i32).collect();

        res
    }
}

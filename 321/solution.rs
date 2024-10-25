impl Solution {
    /// Find the maximum number that can be obtained by selecting k numbers from nums1 and nums2
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max = vec![];
        let mut start = 0;
        let mut end = k;

        // 6. Iterate through all possible values of x and y
        // x = start, y = end = k - start
        while start <= k && start <= nums1.len() as i32 && end >= 0 {
            // 1. Find the maximum subarray of length x in nums1
            let v1 = Self::max_sub_array(&nums1, start);
            // 2. Find the maximum subarray of length y in nums2
            let v2 = Self::max_sub_array(&nums2, end);
            // 3. Merge the two arrays to get the maximum number of length k
            let v = Self::merge(&v1, &v2);

            // 4. Compare the length and (5.) Compare the values
            if v.len() > max.len() || v > max {
                max = v;
            }

            // Update the values of x and y (start and end)
            start += 1;
            end -= 1;
        }

        max
    }

    /// Find the maximum subarray of length k
    fn max_sub_array(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = vec![];
        let mut drop = nums.len() as i32 - k;

        // Iterate through all the numbers in the array
        for &num in nums.iter() {
            // If last element in the stack is less than the current number
            // and we still have to drop some elements
            // then pop the last element from the stack
            while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < num {
                stack.pop();
                drop -= 1;
            }

            // Push the current number to the stack
            stack.push(num);
        }

        // Truncate the stack to get the maximum subarray of length k
        // This can happen if the array is already sorted in descending order
        stack.truncate(k as usize);
        stack
    }

    /// Merge two arrays to get the maximum number
    /// 
    /// Similar to classic merge operation in merge sort
    fn merge(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;

        while i < v1.len() || j < v2.len() {
            if i == v1.len() {
                res.push(v2[j]);
                j += 1;
            } else if j == v2.len() {
                res.push(v1[i]);
                i += 1;
            } else if v1[i] > v2[j] {
                res.push(v1[i]);
                i += 1;
            } else if v1[i] < v2[j] {
                res.push(v2[j]);
                j += 1;
            } else {
                let mut x = i;
                let mut y = j;

                while x < v1.len() && y < v2.len() && v1[x] == v2[y] {
                    x += 1;
                    y += 1;
                }

                if x == v1.len() {
                    res.push(v2[j]);
                    j += 1;
                } else if y == v2.len() {
                    res.push(v1[i]);
                    i += 1;
                } else if v1[x] > v2[y] {
                    res.push(v1[i]);
                    i += 1;
                } else {
                    res.push(v2[j]);
                    j += 1;
                }
            }
        }

        res
    }
}

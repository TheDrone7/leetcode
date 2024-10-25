use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        // Calculate the target remainder
        // Needs to be temporarily converted to i64 to avoid overflow
        let target = (nums.iter().map(|&x| x as i64).sum::<i64>() % p as i64) as i32;

        // Check if the target is 0
        if target == 0 {
            return 0;
        }

        let mut map = HashMap::new();
        map.insert(0, -1);

        // Initialize the sum and result
        let n = nums.len();
        let mut sum = 0;
        let mut result = n as i32;

        // Iterate through the array
        for i in 0..n {
            // Calculate the sum and the difference
            sum = (sum + nums[i]) % p;
            let diff = (sum - target + p) % p;

            // Check if the difference is in the map
            if let Some(&j) = map.get(&diff) {
                result = result.min(i as i32 - j);
            }

            // Insert the sum into the map
            map.insert(sum, i as i32);
        }

        // Check if the result is the same as the length of the array
        if result == n as i32 {
            return -1;
        }

        result
    }
}

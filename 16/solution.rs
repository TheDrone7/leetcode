impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // Sort the array
        let mut nums = nums;
        nums.sort();

        // Initialize the result
        let mut res = nums[0] + nums[1] + nums[2];

        // Iterate through the array
        for i in 0..nums.len() {
            // Initialize the left and right pointers
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            // Iterate through the array
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                // Update the result if the sum is closer to the target
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }

                // Update the pointers
                if sum > target {
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    return sum;
                }
            }
        }

        res
    }
}

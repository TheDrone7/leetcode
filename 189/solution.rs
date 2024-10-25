impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // k can be greater than nums.len()
        // so we need to take modulo of k with nums.len()
        let k = k as usize % nums.len();
        
        // reverse the whole array
        nums.reverse();
        
        // reverse the first k elements
        nums[..k].reverse();
        // reverse the remaining elements
        nums[k..].reverse();
    }
}

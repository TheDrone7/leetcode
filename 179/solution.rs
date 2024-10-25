impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        // Sort by custom comparator
        nums.sort_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ba.cmp(&ab)
        });

        // Concatenate the sorted numbers
        let result = nums.iter().map(|num| num.to_string()).collect::<String>();

        // Check if the result is 0
        if result.starts_with("0") {
            return "0".to_string();
        }

        // Return the result
        result
    }
}

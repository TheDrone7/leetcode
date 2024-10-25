impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i: i64 = 1;

        while i * i <= num as i64 {
            if i * i == num as i64 {
                return true;
            }

            i += 1;
        }

        return false;
    }
}

impl Solution {
    /// Count the number of integers less than or equal to n
    /// that have the prefix `current`.
    fn count(mut current: i64, n: i32) -> i32 {
        let mut next = current + 1;
        let mut count = 0;

        while current <= n as i64 {
            count += std::cmp::min(n as i64 - current + 1, next - current);
            next *= 10;
            current *= 10;
        }

        count as i32
    }

    /// Find the k-th lexicographically smallest integer in the range [1, n].
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut res: i32 = 1;
        k -= 1;

        while k > 0 {
            let cnt = Self::count(res as i64, n);
            if k >= cnt {
                res += 1;
                k -= cnt;
            } else {
                k -= 1;
                res *= 10;
            }
        }

        res
    }
}

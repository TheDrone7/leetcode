impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // 1. Reverse the string
        let mut rev = s.clone();
        rev = rev.chars().rev().collect();

        // 1. Append the reversed string to the original string
        // With a special character, `#` in between.
        let s_new = format!("{}#{}", s, rev);
        let s_new = s_new.as_bytes();

        // 2. Build the prefix table
        let mut p = vec![0; s_new.len()];

        // 3. Find the longest palindrome prefix
        for i in 1..s_new.len() {
            let mut j = p[i - 1];
            while j > 0 && s_new[i] != s_new[j] {
                j = p[j - 1];
            }

            if s_new[i] == s_new[j] {
                j += 1;
            }

            p[i] = j;
        }

        // 4. Generate and return the result
        let add = rev[0..(s.len() - p[s_new.len() - 1])].to_string();
        add + &s
    }
}

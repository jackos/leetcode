use std::collections::HashMap;

struct Solution;
impl Solution {
    /// Loops through the string putting the current iterator
    /// against the character in a hashmap.
    /// It counts the total characters by subtracting the iterator
    /// from where the first character started.
    /// If the character is duplicated when inserting into the hashmap,
    /// the new start becomes where the first duplicated character occurred,
    /// So anything after that is still valid as unique string
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();

        let mut i = 0;
        let mut start = -1;
        let mut result = 0;

        for c in s.chars() {
            if let Some(last) = map.insert(c, i) {
                let last = last;
                start = std::cmp::max(start, last);
            }
            result = std::cmp::max(result, i - start);
            i += 1;
        }
        result
    }
}

fn main() {
    let s = "abcsdfbasdba".to_string();
    let ans = Solution::length_of_longest_substring(s);
    println!("{}", ans);
}

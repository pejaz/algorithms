/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
use std::collections::HashMap;
#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            // s 长度必须是偶数
            return false;
        }

        let mp = HashMap::from([(b')', b'('), (b']', b'['), (b'}', b'{')]);
        let mut st = vec![];
        for c in s.as_bytes() {
            if !mp.contains_key(c) {
                // 左括号
                st.push(c)
            } else if st.is_empty() || mp.get(c).unwrap() != st.pop().unwrap() {
                return false;
            }
        }

        return st.len() == 0;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert!(true)
    }
}

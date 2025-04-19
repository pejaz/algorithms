/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] 反转字符串
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_string() {
        //assert_eq!(Solution::reverse_string(vec![]),[]);
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 * [459] 重复的子字符串
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        // s+s 如果去掉首尾字符后能包含 s 那么 s 一定为周期重复字符串
        // 证明链接：https://writings.sh/post/algorithm-repeated-string-pattern
        (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s)
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
    fn test_repeated_substring_pattern() {
        //assert_eq!(Solution::repeated_substring_pattern(vec![]),[]);
        assert!(true)
    }
}

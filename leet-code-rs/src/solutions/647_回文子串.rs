/*
 * @lc app=leetcode.cn id=647 lang=rust
 *
 * [647] 回文子串
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：（其实是暴力）
     *  1. dp[i]: 前 i 个元素的回文字串的数目
     *  2. 递推公式 dp[i] = dp[i-1] + 以 i 结尾的回文字串
     *  3. 初始化：dp[0] = 1；
     *
     * 可以单独用一个变量来做累加。利用 dp 来做缓存，防止重复比价。
     */
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut dp = vec![0; s.len()];
        dp[0] = 1;

        for i in 1..s.len() {
            for j in 0..=i {
                if s[j..=i].iter().eq(s[j..=i].iter().rev()) {
                    // 回文子串
                    dp[i] += 1
                }
            }

            dp[i] += dp[i - 1];
        }

        return dp[s.len() - 1];
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
    fn test_count_substrings() {
        //assert_eq!(Solution::count_substrings(vec![]),[]);
        assert!(true)
    }
}

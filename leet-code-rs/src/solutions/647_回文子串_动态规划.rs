/*
 * @lc app=leetcode.cn id=647 lang=rust
 *
 * [647] 回文子串
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：动态规划单独用一个变量来做累加。利用 dp 来做缓存，防止重复比较。
     *  1. dp[i][j]: i 为起点，j 为终点的字串是否是回文
     *  2. 递推公式
     *     - s[i] == s[j]:
     *      - 如果 j-1 == 0 或者 j-i ==1：那么是回文，
     *      - >1 时如果dp[i+1][j-1]==true，那么是回文，否则不是
     *     - s[i] != s[j]: 不是
     *  3. 遍历顺序：dp[i][j]依赖dp[i+1][j-1]，所以 i 要从大到小，j 要从小到大
     */
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut res = 0;
        let mut dp = vec![vec![false; s.len()]; s.len()];

        // 第一层遍历是终点j，要从小到大
        for j in 0..s.len() {
            // 第二层遍历是起点 i，要从大到小
            for i in (0..=j).rev() {
                if s[i] == s[j] && (j - i <= 1 || dp[i + 1][j - 1]) {
                    res += 1;
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
        }

        return res;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;
    // aabaa
    // 10000
    // 11000
    // 00100
    // 01010
    // 10011
    #[test]
    fn test_count_substrings() {
        assert_eq!(Solution::count_substrings(String::from("aaa")), 6);
        assert!(true)
    }
}

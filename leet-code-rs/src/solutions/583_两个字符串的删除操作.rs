/*
 * @lc app=leetcode.cn id=583 lang=rust
 *
 * [583] 两个字符串的删除操作
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：本题是字符串编辑记录删与不删问题（选和不选问题）
     *  1. dp[i][j]: s1 前 i 个元素和 s2 前 j 个元素删除的最小步数
     *  2. 递推公式：
     *     - s1[i] == s2[i]（不用处理）: dp[i+1][j+1] = dp[i][j];
     *     - s1[i] != s2[i]（删除 s1 或者 s2）: dp[i+1][j+1] = min(dp[i][j+1]，dp[i+1][j]) + 1
     *  3. 初始化：
     *      - 求最小，字符串长度最大为 500，所以初始化为 500 即可
     *      根据 dp 含义：dp[i][0] = i,dp[0][j] = j
     *  4. 返回dp最后一个元素
     *
     * 思路二: 先求出 s1和 s2 的最长公共子序列max_sub，
     *  最后用 length(s1+s2) - max_sub*2 即为最小步数
     */
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        let mut dp = vec![vec![500; word2.len() + 1]; word1.len() + 1];
        // 初始化 DP[i][0] 和 DP[0][j]
        for i in 0..=word1.len() {
            dp[i][0] = i;
        }
        for j in 0..=word2.len() {
            dp[0][j] = j;
        }

        for i in 0..word1.len() {
            for j in 0..word2.len() {
                dp[i + 1][j + 1] = if word1[i] == word2[j] {
                    dp[i][j]
                } else {
                    dp[i][j + 1].min(dp[i + 1][j]) + 1
                }
            }
        }

        return dp[word1.len()][word2.len()] as i32;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;
    //      e  a  t
    // [[0, 1, 2, 3],
    //s [1, 1, 2, 3],
    //e [2, 1, 1, 3],
    //a [3, 1, 1, 1]]
    #[test]
    fn test_min_distance() {
        assert_eq!(
            Solution::min_distance(String::from("leetcode"), String::from("etco")),
            4
        );
        assert!(true)
    }
}

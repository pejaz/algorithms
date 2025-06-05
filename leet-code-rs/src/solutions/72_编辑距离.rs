/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：本题是删与不删问题。
     *  1. dp[i+1][j+1] 定义：s1 前i个元素于 s2 前j个元素的最少操作数
     *  2. 递推公式：
     *     - s1[i] == s2[j]: dp[i+1][j+1] = dp[i][j]
     *     - s1[i] != s2[j]: dp[i+1][j+1] = min(dp[i+1][j],dp[i][j+1],dp[i][j]) + 1
     *       1. s1插入字符：相当于删除 s2，dp[i+1][j+1] = dp[i+1][j] + 1
     *       2. 删除字符：dp[i+1][j+1] = dp[i][j+1] + 1
     *       3. 替换字符：也相当于插入一个和 s2 相等的字符，然后同时删除该字符，dp[i+1][j+1] = dp[i][j] + 1
     *  3. 初始化：dp[i][0] = i， dp[0][j]=j        
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
                    dp[i][j + 1].min(dp[i + 1][j]).min(dp[i][j]) + 1
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

    #[test]
    fn test_min_distance() {
        //assert_eq!(Solution::min_distance(vec![]),[]);
        assert!(true)
    }
}

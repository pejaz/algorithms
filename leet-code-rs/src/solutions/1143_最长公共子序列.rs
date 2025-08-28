/* 🔖
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  LCS：选或不选的问题。
     *  1. dp[i][j]含义：代表前 i 个元素和前 j个元素的最长公共子序列长度
     *  2. 递推公式：
     *      选：text1[i] == text2[j]: dp[i+1][j+1] = dp[i][j] + 1;
     *      不选：text[i] != text2[j]: dp[i+1][j+1] = max(dp[i][j+1],dp[i+1][j]);
     *  3. 下标偏移 1，可以不同特殊处理 dp[0] 的情况
     *  4. 区间含义，最优解在最后一个元素
     */
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.chars().collect::<Vec<char>>();
        let text2 = text2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                dp[i + 1][j + 1] = if text1[i] == text2[j] {
                    dp[i][j] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }

        return dp[text1.len()][text2.len()];
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
    fn test_longest_common_subsequence() {
        //assert_eq!(Solution::longest_common_subsequence(vec![]),[]);
        assert!(true)
    }
}

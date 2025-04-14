/*
 * @lc app=leetcode.cn id=1143 lang=javascript
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
/**
 * @param {string} text1
 * @param {string} text2
 * @return {number}
 */
var longestCommonSubsequence = function (text1, text2) {
  let max = 0
  const dp = Array(text1.length + 1).fill(0).map(() => Array(text2.length + 1).fill(0))
  for (let i = 1; i <= text1.length; i++) {
    const num = text1[i - 1];
    for (let j = 1; j <= text2.length; j++) {
      const num2 = text2[j - 1];
      if (num === num2) {
        dp[i][j] = dp[i - 1][j - 1] + 1
        max = Math.max(max, dp[i][j])
      } else {
        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1])
      }
    }
  }
  return max
};
// @lc code=end


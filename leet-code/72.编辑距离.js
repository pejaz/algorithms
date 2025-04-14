/*
 * @lc app=leetcode.cn id=72 lang=javascript
 *
 * [72] 编辑距离
 */

// @lc code=start
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function (word1, word2) {
  const dp = Array.from({ length: word1.length + 1 }, () => Array(word2.length + 1).fill(0))
  for (let i = 0; i <= word1.length; i++) {
    dp[i][0] = i
  }
  for (let i = 0; i <= word2.length; i++) {
    dp[0][i] = i
  }
  for (let i = 1; i <= word1.length; i++) {
    const str1 = word1[i - 1];
    for (let j = 1; j <= word2.length; j++) {
      const str2 = word2[j - 1];
      if (str1 === str2) {
        dp[i][j] = dp[i - 1][j - 1]
      } else {
        // 增和删其实都是一样的，
        // 替换要求dp[i-1][j - 1]要相等的前提下 + 1
        dp[i][j] = Math.min(dp[i - 1][j] + 1, dp[i][j - 1] + 1, dp[i - 1][j - 1] + 1)
      }
    }
  }
  return dp[word1.length][word2.length]
};
// @lc code=end


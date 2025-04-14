/*
 * @lc app=leetcode.cn id=516 lang=javascript
 *
 * [516] 最长回文子序列
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var longestPalindromeSubseq = function (s) {
  // dp[i][j]含义: 从下标[i,j]所形成的字符串为回文串，j大于等于i下标
  // 同时 dp[i][j]的状态依赖于dp[i + 1][j - 1]，所以顺序i的顺序要倒序,j的顺序为顺序
  const len = s.length
  const dp = Array.from({ length: len }, () => Array(len).fill(0))
  // 初始化
  for (let i = 0; i < len; i += 1) {
    dp[i][i] = 1
  }

  for (let i = len - 1; i >= 0; i -= 1) {
    // j要大于等于i，j等于i的情况已经在初始化阶段处理了，
    // 所以保证j大于i，j初始值为i+1
    for (let j = i + 1; j < len; j += 1) {
      if (s[i] === s[j]) {
        dp[i][j] = dp[i + 1][j - 1] + 2
      } else {
        dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1])
      }
    }
  }
  return dp[0][len - 1]
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = longestPalindromeSubseq;
// @after-stub-for-debug-end
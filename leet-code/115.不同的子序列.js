/*
 * @lc app=leetcode.cn id=115 lang=javascript
 *
 * [115] 不同的子序列
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {number}
 */
var numDistinct = function (s, t) {
  // dp[i][j]:代表以i-1为结尾的t子序列中出现以j-1为结尾的s的个数为dp[i][j]。
  const dp = Array(t.length + 1).fill(0).map(() => Array(s.length + 1).fill(0))
  for (let i = 0; i < s.length; i++) {
    // t为0，表示空串，那么s不管以什么结尾，删到最后一定也会为空串，即包含空串的t
    dp[0][i] = 1
  }
  for (let i = 1; i <= t.length; i++) {
    const tStr = t[i - 1];
    for (let j = 1; j <= s.length; j++) {
      const sStr = s[j - 1];
      if (tStr === sStr) {
        dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1]
      } else {
        dp[i][j] = dp[i][j - 1]
      }
    }
  }
  return dp[t.length][s.length]
};
// @lc code=end


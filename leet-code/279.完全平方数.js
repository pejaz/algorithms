/*
 * @lc app=leetcode.cn id=279 lang=javascript
 *
 * [279] 完全平方数
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var numSquares = function (n) {
  const dp = Array(n + 1).fill(Infinity)
  dp[0] = 0
  for (let i = 1; i <= Math.sqrt(n); i += 1) {
    // 物品
    const num = i ** 2
    for (let j = num; j <= n; j += 1) {
      dp[j] = Math.min(dp[j], dp[j - num] + 1)
    }
  }
  return dp[n]
};
// @lc code=end


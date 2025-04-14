/*
 * @lc app=leetcode.cn id=96 lang=javascript
 *
 * [96] 不同的二叉搜索树
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var numTrees = function (n) {
  dp = []
  dp[0] = 1
  dp[1] = 1
  dp[2] = 2
  for (let i = 3; i <= n; i++) {
    dp[i] = 0
    for (let j = 0; j < i; j++) {
      dp[i] += dp[j] * dp[(i - 1) - j]
    }
  }
  return dp[n]
};
// @lc code=end


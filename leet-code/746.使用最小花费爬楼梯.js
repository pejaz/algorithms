/*
 * @lc app=leetcode.cn id=746 lang=javascript
 *
 * [746] 使用最小花费爬楼梯
 */

// @lc code=start
/**
 * @param {number[]} cost
 * @return {number}
 */
var minCostClimbingStairs = function (cost) {
  let dp = [cost[0], cost[1]]
  for (let i = 2; i < cost.length; i++) {
    const res1 = dp[0] + cost[i]
    const res2 = dp[1] + cost[i]
    dp[0] = dp[1]
    dp[1] = Math.min(res1, res2)
  }
  return Math.min(dp[0], dp[1])
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = minCostClimbingStairs;
// @after-stub-for-debug-end
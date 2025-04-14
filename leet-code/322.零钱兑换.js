/*
 * @lc app=leetcode.cn id=322 lang=javascript
 *
 * [322] 零钱兑换
 */

// @lc code=start
/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function (coins, amount) {
  const dp = Array(amount + 1).fill(Infinity)
  dp[0] = 0
  // 1. 物品
  for (let i = 0; i < coins.length; i++) {
    const coin = coins[i];
    // 2. 背包
    for (let j = coin; j <= amount; j += 1) {
      dp[j] = Math.min(dp[j], dp[j - coin] + 1)
    }
  }
  return dp[amount] === Infinity ? -1 : dp[amount]
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = coinChange;
// @after-stub-for-debug-end
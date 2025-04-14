/*
 * @lc app=leetcode.cn id=121 lang=javascript
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function (prices) {
  const left = Array(prices.length).fill(0)
  left[0] = prices[0]
  for (let i = 1; i < prices.length; i++) {
    left[i] = Math.min(left[i - 1], prices[i])
  }

  let max = 0
  for (let i = 1; i < prices.length; i++) {
    max = Math.max(max, prices[i] - left[i])
  }
  return max
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = maxProfit;
// @after-stub-for-debug-end
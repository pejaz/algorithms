/*
 * @lc app=leetcode.cn id=1049 lang=javascript
 *
 * [1049] 最后一块石头的重量 II
 */

// @lc code=start
/**
 * @param {number[]} stones
 * @return {number}
 */
var lastStoneWeightII = function (stones) {
  const sum = stones.reduce((a, b) => a + b) / 2
  const target = sum | 0

  const dp = Array(target + 1).fill(0)
  // 遍历物品
  for (let i = 0; i < stones.length; i++) {
    const stone = stones[i];
    // 遍历背包，重量和价值一样都为stone
    for (let j = target; j >= stone; j--) {
      dp[j] = Math.max(dp[j], dp[j - stone] + stone)
    }
  }

  return (sum - dp[target]) * 2
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = lastStoneWeightII;
// @after-stub-for-debug-end
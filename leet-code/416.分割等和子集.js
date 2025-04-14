/*
 * @lc app=leetcode.cn id=416 lang=javascript
 *
 * [416] 分割等和子集
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canPartition = function (nums) {
  const sum = nums.reduce((a, b) => a + b)
  // 奇数，不能分割为两个子集
  if (sum % 2 !== 0) return false

  const subSum = sum / 2
  const dp = Array(subSum + 1).fill(0)

  // 一维背包，先遍历物品
  for (let i = 0; i <= nums.length - 1; i += 1) {
    // num为物品，每个物品重量和价值为num
    const num = nums[i]

    // 倒序遍历背包容量,背包容量小于物品重量时没有意义，放不进去物品
    for (let j = subSum; j >= num; j -= 1) {
      // dp[j]: 背包容量为j时的最大价值为j
      dp[j] = Math.max(dp[j], dp[j - num] + num)
    }
  }


  // 重量和价值是相等的，所以如果重量为subSum时，最大价值一定也为subSum，否则说明没有构成subSum的子集
  return dp[subSum] === subSum
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = canPartition;
// @after-stub-for-debug-end
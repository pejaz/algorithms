/*
 * @lc app=leetcode.cn id=377 lang=javascript
 *
 * [377] 组合总和 Ⅳ
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var combinationSum4 = function (nums, target) {
  // 完全背包之排列
  const dp = Array(target + 1).fill(0)
  dp[0] = 1

  // 1. 遍历背包
  for (let i = 0; i <= target; i++) {
    // 遍历物品
    for (let j = 0; j < nums.length; j++) {
      const num = nums[j];
      dp[i] += i - num >= 0 ? dp[i - num] : 0
    }
  }
  return dp[target]
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = combinationSum4;
// @after-stub-for-debug-end
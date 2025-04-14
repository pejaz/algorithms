/*
 * @lc app=leetcode.cn id=198 lang=javascript
 *
 * [198] 打家劫舍
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function (nums) {
  // dp[i]含义: 到i号屋所能偷取的最大金额
  // 偷i : dp[i-2] + nums[i] ,不偷i: dp[i-1]，取最大值
  // 递推公式dp[i] = Math.max(不偷i，偷i)

  const dp = Array(nums.length - 1).fill(0)
  dp[0] = nums[0]
  dp[1] = Math.max(nums[0], nums[1])

  for (let i = 2; i < nums.length; i++) {
    const num = nums[i];
    dp[i] = Math.max(dp[i - 2] + num, dp[i - 1])
  }

  return dp[nums.length - 1]
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=494 lang=javascript
 *
 * [494] 目标和
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var findTargetSumWays = function (nums, target) {
  // left + right = sum ,left - right = target; =>   left = (target + sum)/2 
  const sum = nums.reduce((a, b) => a + b)
  if (Math.abs(target) > sum) return 0
  // (targe+sum)为奇数的时候，/2会余0.5小数，nums数组都是整数，不可能和为 x.5
  if ((target + sum) % 2) return 0

  const leftSum = (target + sum) / 2
  // 此时转化为求nums的子项和为leftSum有多少中方式组合。
  // 转为背包问题就是找到装满容量为leftSum的方法数。
  // 递推公式为dp[j] +=dp[j-nums[i]]
  const dp = Array(leftSum + 1).fill(0)
  // 这里区别其他，dp[0]要初始化为1，表示默认什么都不装(装满容量为0)为1种方法
  dp[0] = 1
  for (let i = 0; i < nums.length; i++) {
    // 物品
    const num = nums[i];
    for (let j = leftSum; j >= num; j -= 1) {
      dp[j] += dp[j - num]
    }
  }
  return dp[leftSum]
};
// @lc code=end


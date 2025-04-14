/*
 * @lc app=leetcode.cn id=53 lang=javascript
 *
 * [53] 最大子数组和
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function (nums) {
  let pre = nums[0]
  let max = nums[0]
  for (let i = 1; i < nums.length; i++) {
    const num = nums[i];
    const cur = Math.max(pre + num, num)
    pre = cur
    max = Math.max(max, cur)
  }
  return max
};
// @lc code=end


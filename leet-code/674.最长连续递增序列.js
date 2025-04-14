/*
 * @lc app=leetcode.cn id=674 lang=javascript
 *
 * [674] 最长连续递增序列
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var findLengthOfLCIS = function (nums) {
  let max = 1
  let tempMax = 1
  for (let i = 1; i < nums.length; i++) {
    if (nums[i] > nums[i - 1]) {
      tempMax++
      max = Math.max(max, tempMax)
    } else {
      tempMax = 1
    }
  }
  return max
};
// @lc code=end


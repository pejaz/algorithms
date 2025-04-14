/*
 * @lc app=leetcode.cn id=209 lang=javascript
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
/**
 * @param {number} target
 * @param {number[]} nums
 * @return {number}
 */
var minSubArrayLen = function(target, nums) {
  let left = 0, right = 0, sum = 0, res = Infinity;
  while (right < nums.length) {
    sum += nums[right];
    while (sum >= target) {
      res = Math.min(res, right - left + 1);
      sum -= nums[left];
      left++;
    }
    right++;
  }
  return res === Infinity ? 0 : res;
};
// @lc code=end


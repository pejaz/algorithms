/*
 * @lc app=leetcode.cn id=15 lang=javascript
 *
 * [15] 三数之和
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function (nums) {
  if (nums.length < 3) return [];
  nums.sort((a, b) => a - b);

  let res = [];
  let idx = 0;
  while (idx < nums.length - 2) {
    if (nums[idx] > 0) break

    let left = idx + 1; right = nums.length - 1;

    while (left < right) {
      const sum = nums[idx] + nums[left] + nums[right];
      if (sum === 0) {
        res.push([nums[idx], nums[left], nums[right]])
        left++
        right--

        while (left < right && nums[left] === nums[left - 1]) { left++ }
        while (left < right && nums[right] === nums[right + 1]) { right-- }
      } else if (sum > 0) {
        right--
        while (left < right && nums[right] === nums[right + 1]) { right-- }
      } else {
        left++
        while (left < right && nums[left] === nums[left - 1]) { left++ }
      }
    }

    idx++
    while (idx < nums.length - 2 && nums[idx] === nums[idx - 1]) { idx++ }
  }
  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = threeSum;
// @after-stub-for-debug-end
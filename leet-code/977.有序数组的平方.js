/*
 * @lc app=leetcode.cn id=977 lang=javascript
 *
 * [977] 有序数组的平方
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var sortedSquares = function (nums) {
  let left = 0, right = idx = nums.length - 1
  let res = Array(nums.length)
  while (left <= right) {
    if (nums[left] ** 2 > nums[right] ** 2) {
      res[idx] = nums[left] ** 2
      left++
    } else {
      res[idx] = nums[right] ** 2
      right--
    }
    idx--
  }
  return res
}
// @lc code=end


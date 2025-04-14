/*
 * @lc app=leetcode.cn id=376 lang=javascript
 *
 * [376] 摆动序列
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var wiggleMaxLength = function (nums) {
  if (nums.length <= 1) return nums.length

  let res = 1
  let i = 1
  while (nums[i] === nums[i - 1]) {
    i++
  }
  let flag = nums[i] > nums[i - 1] ? true : false
  for (; i < nums.length; i++) {
    const num = nums[i]
    const pre = nums[i - 1]
    
    if (flag && num > pre) {
      res++
      flag = false
    }
    if (!flag && num < pre) {
      res++
      flag = true
    }
  }
  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = wiggleMaxLength;
// @after-stub-for-debug-end
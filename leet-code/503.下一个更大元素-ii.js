/*
 * @lc app=leetcode.cn id=503 lang=javascript
 *
 * [503] 下一个更大元素 II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var nextGreaterElements = function (nums) {
  const length = nums.length
  const res = Array(length).fill(-1)
  // 单调栈,使用取模%+两倍数组长度来模拟环的过程
  const stack = []
  for (let i = 0; i < length * 2; i++) {
    const idx = i % length
    const item = nums[idx];
    while (stack.length) {
      // 单调栈顶
      const topIdx = stack.pop()
      if (item > nums[topIdx]) {
        res[topIdx] = item
      } else {
        stack.push(topIdx)
        break
      }
    }
    stack.push(idx)
  }
  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = nextGreaterElements;
// @after-stub-for-debug-end
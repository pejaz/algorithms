/*
 * @lc app=leetcode.cn id=496 lang=javascript
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var nextGreaterElement = function (nums1, nums2) {
  const numMap = new Map()
  const res = Array(nums1.length).fill(-1)
  nums1.forEach((num, i) => {
    numMap.set(num, i)
  })

  // 单调栈
  const stack = []
  for (let i = 0; i < nums2.length; i++) {
    const item = nums2[i];
    while (stack.length) {
      const top = stack.pop()
      if (item > top) {
        if (numMap.has(top)) {
          res[numMap.get(top)] = item
        }
      } else {
        stack.push(top)
        break
      }
    }
    stack.push(item)
  }
  return res
};
// @lc code=end


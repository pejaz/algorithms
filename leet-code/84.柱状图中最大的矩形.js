/*
 * @lc app=leetcode.cn id=84 lang=javascript
 *
 * [84] 柱状图中最大的矩形
 */

// @lc code=start
/**
 * @param {number[]} heights
 * @return {number}
 */
var largestRectangleArea = function (heights) {
  // 双指针法，查找某一列左边和右边分别比它小形成的面积

  // 从左往右，存储下标
  const minLeft = []
  minLeft[0] = 0

  for (let i = 1; i < heights.length; i++) {
    const minIdx = minLeft[i - 1];
    minLeft[i] = heights[i] < heights[minIdx] ? i : minIdx
  }

  // 从右往左
  const minRight = []
  minRight[heights.length - 1] = heights.length - 1

  for (let i = heights.length - 2; i >= 0; i -= 1) {
    const minIdx = minRight[i + 1]
    minRight[i] = heights[i] < heights[minIdx] ? i : minIdx
  }

  // 查找最大矩形
  let maxArea = 0
  for (let i = 0; i < heights.length; i++) {
    const h = heights[i];
    if (i === 0) {
      // 第一个列 直接找右边比它小的
      const w = minRight[i]
      maxArea = Math.max(maxArea, h * w)
      continue
    }

    if (i === heights.length - 1) {
      // 最后一列 直接找左边比它小的
      const w = minLeft[i]
      maxArea = Math.max(maxArea, h * w)
      continue
    }

    let rightIdx = minRight[i]
    let leftIdx = minLeft[i]
    // 边界处理
    rightIdx === heights.length && rightIdx++
    leftIdx === 0 && leftIdx--

    const w = rightIdx - leftIdx - 1
    maxArea = Math.max(maxArea, h * w)
  }

  return maxArea
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = largestRectangleArea;
// @after-stub-for-debug-end
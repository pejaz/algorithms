/*
 * @lc app=leetcode.cn id=435 lang=javascript
 *
 * [435] 无重叠区间
 */

// @lc code=start
/**
 * @param {number[][]} intervals
 * @return {number}
 */
var eraseOverlapIntervals = function (intervals) {
  intervals.sort((a, b) => {
    if (a[0] === b[0]) {
      return a[1] - b[1]
    } else {
      return a[0] - b[0]
    }
  })
  let i = 0
  let res = 0
  while (i < intervals.length) {
    const point1 = intervals[i]
    let right = point1[1]
    let j = i + 1
    while (j < intervals.length) {
      const point2 = intervals[j]
      if (point2[0] >= right) break
      j++
      // 更新右边界
      right = Math.min(right, point2[1])
    }
    res++
    i = j
  }
  return intervals.length - res
};
// @lc code=end


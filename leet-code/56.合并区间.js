/*
 * @lc app=leetcode.cn id=56 lang=javascript
 *
 * [56] 合并区间
 */

// @lc code=start
/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
var merge = function (intervals) {
  intervals.sort((a, b) => {
    if (a[0] === b[0]) {
      return a[1] - b[1]
    } else {
      return a[0] - b[0]
    }
  })
  let i = 0
  const res = []
  while (i < intervals.length) {
    const interval1 = intervals[i]
    let right = interval1[1]
    let j = i + 1
    while (j < intervals.length) {
      const interval2 = intervals[j]
      if (interval2[0] > right) break
      j++
      // 更新右边界
      right = Math.max(right, interval2[1])
    }
    res.push([interval1[0], right])
    i = j
  }
  return res
};
// @lc code=end


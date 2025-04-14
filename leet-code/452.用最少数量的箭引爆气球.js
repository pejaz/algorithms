/*
 * @lc app=leetcode.cn id=452 lang=javascript
 *
 * [452] 用最少数量的箭引爆气球
 */

// @lc code=start
/**
 * @param {number[][]} points
 * @return {number}
 */
var findMinArrowShots = function (points) {
  points.sort((a, b) => {
    if (a[0] === b[0]) {
      return a[1] - b[1]
    } else {
      return a[0] - b[0]
    }
  })
  let i = 0
  const res = []
  while (i < points.length) {
    const point1 = points[i]
    let right = point1[1]
    let j = i + 1
    while (j < points.length) {
      const point2 = points[j]
      if (point2[0] > right) break
      j++
      // 更新右边界
      right = Math.min(right, point2[1])
    }
    res.push(points.slice(i, j + 1))
    i = j
  }
  return res.length
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = findMinArrowShots;
// @after-stub-for-debug-end
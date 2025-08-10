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

  let shot = 1
  for (let i = 1; i < points.length; i++) {
    if (points[i][0] > points[i - 1][1]) {
      // 下一个气球的左边界大于上一个气球的右边界
      shot++
    } else {
      // 更新右边界
      points[i][1] = Math.min(points[i][1], points[i - 1][1]);
    }
  }

  return shot
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = findMinArrowShots;
// @after-stub-for-debug-end
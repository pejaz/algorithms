/*
 * @lc app=leetcode.cn id=59 lang=javascript
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number[][]}
 */
var generateMatrix = function (n) {
  let startX = 0, startY = 0, i = 0, j = 0, count = 1,
    offset = 1
  matrix = new Array(n).fill(0).map(() => new Array(n).fill(0))
  loop = 0
  loopCount = n / 2 | 0

  while (loop < loopCount) {
    // 从左到右
    i = startY
    j = startX
    for (; j < n - offset; j++) {
      matrix[i][j] = count++
    }
    // 从上到下
    for (; i < n - offset; i++) {
      matrix[i][j] = count++
    }
    // 从右到左
    for (; j > startY; j--) {
      matrix[i][j] = count++
    }
    // 从下到上
    for (; i > startX; i--) {
      matrix[i][j] = count++
    }
    startX++
    startY++
    offset++
    loop++
  }
  // 奇数的情况
  if (n % 2 !== 0) matrix[startX][startY] = count
  return matrix
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = generateMatrix;
// @after-stub-for-debug-end
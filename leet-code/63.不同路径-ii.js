/*
 * @lc app=leetcode.cn id=63 lang=javascript
 *
 * [63] 不同路径 II
 */

// @lc code=start
/**
 * @param {number[][]} obstacleGrid
 * @return {number}
 */
var uniquePathsWithObstacles = function (obstacleGrid) {
  const dp = Array(obstacleGrid.length).fill(1).map(() => Array(obstacleGrid[0].length).fill(1))
  dp[0][0] = obstacleGrid[0][0] === 1 ? 0 : 1
  for (let i = 1; i < obstacleGrid[0].length; i++) {
    dp[0][i] = dp[0][i - 1] === 0 ? 0 : obstacleGrid[0][i] === 1 ? 0 : 1
  }
  if (obstacleGrid.length === 1) return dp[0].at(-1)
  for (let j = 1; j < obstacleGrid.length; j++) {
    dp[j][0] = dp[j - 1][0] === 0 ? 0 : obstacleGrid[j][0] === 1 ? 0 : 1
  }
  for (let i = 1; i < obstacleGrid.length; i++) {
    for (let j = 1; j < obstacleGrid[0].length; j++) {
      dp[i][j] = obstacleGrid[i][j] === 1 ? 0 : (dp[i - 1][j] + dp[i][j - 1])
    }
  }
  return dp.at(-1).at(-1)
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = uniquePathsWithObstacles;
// @after-stub-for-debug-end
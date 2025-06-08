/*
 * @lc app=leetcode.cn id=200 lang=javascript
 *
 * [200] 岛屿数量
 */

// @lc code=start
/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function (grid) {
  let landscount = 0
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      if (grid[i][j] == 1) {
        dfs(grid, i, j)
        landscount += 1
      }
    }
  }
  return landscount
};

const dfs = (grid, i, j) => {
  //此时形参grid存的是地址
  //修改grid外部的grid也会跟着修改
  if (!grid[i] || !grid[i][j] || grid[i][j] !== '1') {
    //1.出边界
    //2. 为海洋 0
    //3. 已访问过 2
    return null
  }
  //标记访问
  grid[i][j] = 2

  //遍历周边
  dfs(grid, i - 1, j)
  dfs(grid, i + 1, j)
  dfs(grid, i, j - 1)
  dfs(grid, i, j + 1)
}
// @lc code=end


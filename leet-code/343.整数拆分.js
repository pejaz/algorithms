/*
 * @lc app=leetcode.cn id=343 lang=javascript
 *
 * [343] 整数拆分
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var integerBreak = function (n) {
  const memo = []
  memo[1] = 1
  const dfs = (n) => {
    if (memo[n]) return memo[n]
    let max = 1
    for (let i = 1; i < n; i++) {
      max = Math.max(max, i * (n - i), i * dfs(n - i))
    }
    return memo[n] = max
  }
  let maxNum = dfs(n)
  return maxNum
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = integerBreak;
// @after-stub-for-debug-end
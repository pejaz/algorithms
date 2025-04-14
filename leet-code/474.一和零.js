/*
 * @lc app=leetcode.cn id=474 lang=javascript
 *
 * [474] 一和零
 */

// @lc code=start
/**
 * @param {string[]} strs
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var findMaxForm = function (strs, m, n) {
  // 转化为背包问题，dp[i][j] 表示 i个0，j个1 最大能装满dp[i][j]个子集
  // 物品的重量有两个纬度， x个0 和 y个1，最后求出dp[m][n]即可

  const dp = Array(m + 1).fill(0).map(() => Array(n + 1).fill(0))
  let numOfZeros, numOfOnes;
  // 1. 遍历物品
  for (const str of strs) {
    // 求出str中的0和1个数
    numOfZeros = 0;
    numOfOnes = 0;

    for (let c of str) {
      if (c === '0') {
        numOfZeros++;
      } else {
        numOfOnes++;
      }
    }

    // 2. 遍历背包容量,物品只能用一次选择倒序遍历，m,n内外可颠倒
    for (let i = m; i >= numOfZeros; i -= 1) {
      for (let j = n; j >= numOfOnes; j -= 1) {
        // 3. 递推公式，numOfZero为0的重量，numOfOnes为1的重量,
        // 最终为求子集个数，即每个物品价值为1，代表子集数加一个
        dp[i][j] = Math.max(dp[i][j], dp[i - numOfZeros][j - numOfOnes] + 1)
      }
    }
  }
  return dp[m][n]
};
// @lc code=end


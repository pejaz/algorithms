/*
 * @lc app=leetcode.cn id=647 lang=javascript
 *
 * [647] 回文子串
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var countSubstrings = function (s) {
  // dp[i][j]含义: 从下标[i,j]所形成的字符串为回文串，j大于等于i下标
  // 同时 dp[i][j]的状态依赖于dp[i + 1][j - 1]，所以顺序i的顺序要倒序,j的顺序为顺序
  const len = s.length
  const dp = Array.from({ length: len }, () => Array(len).fill(false))
  let res = 0
  for (let i = len - 1; i >= 0; i -= 1) {
    // j要大于等于i，所以j初始值为i
    for (let j = i; j < len; j += 1) {
      if (s[i] === s[j]) {
        if (j - i <= 1 || dp[i + 1][j - 1]) {
          dp[i][j] = true
          res++
        }
      }
    }
  }
  return res
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=583 lang=javascript
 *
 * [583] 两个字符串的删除操作
 */

// @lc code=start
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function (word1, word2) {
  const dp = Array(word1.length + 1).fill(0).map(() => Array(word2.length + 1).fill(0))
  let max = 0
  for (let i = 1; i <= word1.length; i++) {
    const str1 = word1[i - 1];
    for (let j = 1; j <= word2.length; j++) {
      const str2 = word2[j - 1];
      if (str1 === str2) {
        dp[i][j] = dp[i - 1][j - 1] + 1
        max = Math.max(max, dp[i][j])
      } else {
        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1])
      }
    }
  }

  return word1.length + word2.length - max * 2
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = minDistance;
// @after-stub-for-debug-end
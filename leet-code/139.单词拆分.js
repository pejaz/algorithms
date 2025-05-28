/*
 * @lc app=leetcode.cn id=139 lang=javascript
 *
 * [139] 单词拆分
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
var wordBreak = function (s, wordDict) {
  const dp = Array(s.length + 1).fill(false);
  dp[0] = true;

  // 1. 楼梯，对应背包重量，走到 s.length
  for (let j = 0; j <= s.length; j++) {
    // 步数
    for (let i = 0; i < wordDict.length; i++) {
      const str = wordDict[i];
      // 在dp[j-str]为 true（即能抵达的前提），再走 str 步能到达
      if (j >= str.length && s.slice(j - str.length, j) == str && dp[j - str.length] == true) {
        dp[j] = true
      }
    }
  }

  return dp[s.length];
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = wordBreak;
// @after-stub-for-debug-end
/*
 * @lc app=leetcode.cn id=14 lang=javascript
 *
 * [14] 最长公共前缀
 */

// @lc code=start
/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function (strs) {
  let res = strs[0];
  for (let i = 0; i < res.length; i++) {
    const s = res[i];
    for (let j = 1; j < strs.length; j++) {
      const str = strs[j];
      if (!str[i] || str[i] !== s) {
        return res.slice(0, i)
      }
    }
  }

  return res
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=392 lang=javascript
 *
 * [392] 判断子序列
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isSubsequence = function (s, t) {
  if (!s.length) return true
  let idx = 0
  let len = s.length
  for (let i = 0; i < t.length; i++) {
    const str = t[i];
    if (str === s[idx]) {
      idx++
      if (idx >= len) {
        return true
      }
    }
  }
  return false
};
// @lc code=end


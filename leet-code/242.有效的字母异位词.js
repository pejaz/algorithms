/*
 * @lc app=leetcode.cn id=242 lang=javascript
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function (s, t) {
  if (s.length !== t.length) return false;
  const resSet = new Array(26).fill(0);
  const base = "a".charCodeAt();
  for (const i of s) {
    resSet[i.charCodeAt() - base]++;
  }
  for (const i of t) {
    if (!resSet[i.charCodeAt() - base]) return false;
    resSet[i.charCodeAt() - base]--;
  }
  return true;
};
// @lc code=end


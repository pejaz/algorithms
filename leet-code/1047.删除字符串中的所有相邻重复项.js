/*
 * @lc app=leetcode.cn id=1047 lang=javascript
 *
 * [1047] 删除字符串中的所有相邻重复项
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var removeDuplicates = function (s) {
  const stack = []
  for (let i = 0; i < s.length; i++) {
    const str = s[i];
    const top = stack.pop()
    if (top !== str) {
      top && stack.push(top)
      stack.push(str)
    }
  }
  return stack.join('')
};
// @lc code=end


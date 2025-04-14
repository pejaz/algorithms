/*
 * @lc app=leetcode.cn id=20 lang=javascript
 *
 * [20] 有效的括号
 */

// @lc code=start
/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function (s) {
  if (s.length % 2 !== 0) return false
  const map = {
    '(': ')',
    '{': '}',
    '[': ']'
  }
  const stack = []
  for (let i = 0; i < s.length; i++) {
    const str = s[i];
    if (map[str]) {
      stack.push(map[str])
    } else {
      const matchStr = stack.pop()
      if (!matchStr || matchStr !== str) {
        return false
      }
    }
  }

  return stack.length === 0
};
// @lc code=end


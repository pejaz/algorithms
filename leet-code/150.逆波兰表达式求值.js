/*
 * @lc app=leetcode.cn id=150 lang=javascript
 *
 * [150] 逆波兰表达式求值
 */

// @lc code=start
/**
 * @param {string[]} tokens
 * @return {number}
 */
var evalRPN = function (tokens) {
  const signs = ['+', '-', '*', '/']
  const calcMap = {
    '+': (a, b) => a + b,
    '-': (a, b) => a - b,
    '*': (a, b) => a * b,
    '/': (a, b) => (a / b) | 0
  }
  const stack = []
  for (let i = 0; i < tokens.length; i++) {
    const el = tokens[i];
    if (!signs.includes(el)) {
      // 数字
      stack.push(el)
    } else {
      const item1 = stack.pop()
      const item2 = stack.pop() // 前进后出
      stack.push(calcMap[el](+item2, +item1))
    }
  }
  return stack.pop()
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = evalRPN;
// @after-stub-for-debug-end
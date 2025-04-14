/*
 * @lc app=leetcode.cn id=42 lang=javascript
 *
 * [42] 接雨水
 */

// @lc code=start
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function (height) {
  const stack = []
  let res = 0
  for (let i = 0; i < height.length; i++) {
    const num = height[i]

    // 大于栈顶，可以出栈横向接雨水
    while (stack.length && num > height[stack.at(-1)]) {
      const idx = stack.pop()

      if (!stack.length) {
        // 栈顶为空，不能形成 栈顶 -> idx -> i 凹型接雨水结构
        break
      }

      const h = Math.min(num, height[stack.at(-1)]) - height[idx]
      const w = i - stack.at(-1) - 1
      res += h * w
    }

    stack.push(i)
  }

  return res
};
// @lc code=end

/**
 *            =
 *  =         =
 *  =     =   =
 *  = =   = = =
 *  = =   = = =
 */

// @after-stub-for-debug-begin
module.exports = trap;
// @after-stub-for-debug-end
/*
 * @lc app=leetcode.cn id=739 lang=javascript
 *
 * [739] 每日温度
 */

// @lc code=start
/**
 * @param {number[]} temperatures
 * @return {number[]}
 */
var dailyTemperatures = function (temperatures) {
  // 利用单调栈
  const res = Array(temperatures.length).fill(0)
  const stack = []
  for (let i = 0; i < temperatures.length; i++) {
    const item = temperatures[i];
    while (stack.length) {
      const idx = stack.pop() // 栈顶
      if (item > temperatures[idx]) {
        // 大于,获取当前元素和栈顶的下标差
        res[idx] = i - idx
      } else {
        // 小于等于
        stack.push(idx)
        break
      }
    }
    stack.push(i)
  }
  return res
};
// @lc code=end


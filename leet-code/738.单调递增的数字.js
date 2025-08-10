/* 🔖
 * @lc app=leetcode.cn id=738 lang=javascript
 *
 * [738] 单调递增的数字
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var monotoneIncreasingDigits = function (n) {
  // 从右往左遍历，发现非递减则当前位 -1 ，往后低位补充 9
  // 不能从左往右，如 122045，会导致 2 减 1 后变成 121999还是不符合
  let arr = n.toString().split('')
  let fillIdx = -1
  for (let i = arr.length - 2; i >= 0; i -= 1) {
    if (arr[i] > arr[i + 1]) {
      // 非递减，不用考虑 arr[i-1] 的情况，因为下一个循环会自动比较去减
      arr[i] -= 1
      fillIdx = i + 1
    }
  }

  if (fillIdx !== -1) {
    arr.fill(9, fillIdx)
  }

  return +arr.join("")
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = monotoneIncreasingDigits;
// @after-stub-for-debug-end
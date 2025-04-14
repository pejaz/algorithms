/*
 * @lc app=leetcode.cn id=541 lang=javascript
 *
 * [541] 反转字符串 II
 */

// @lc code=start
/**
 * @param {string} s
 * @param {number} k
 * @return {string}
 */
var reverseStr = function (s, k) {
  const resArr = s.split("");
  const reverse = (arr, i, j) => {
    // 翻转[i,j)区间的字符串，不包括j
    j -= 1
    while (i < j) {
      [arr[i], arr[j]] = [arr[j], arr[i]]
      i++
      j--
    }
  }
  for (let i = 0; i < resArr.length; i += 2 * k) {
    if (i + k <= resArr.length) {
      reverse(resArr, i, i + k)
    } else {
      reverse(resArr, i, resArr.length)
    }
  }
  return resArr.join('')
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = reverseStr;
// @after-stub-for-debug-end
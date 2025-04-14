/*
 * @lc app=leetcode.cn id=1035 lang=javascript
 *
 * [1035] 不相交的线
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var maxUncrossedLines = function(nums1, nums2) {
  let max = 0
  const dp = Array(nums1.length + 1).fill(0).map(() => Array(nums2.length + 1).fill(0))
  for (let i = 1; i <= nums1.length; i++) {
    const num = nums1[i - 1];
    for (let j = 1; j <= nums2.length; j++) {
      const num2 = nums2[j - 1];
      if (num === num2) {
        dp[i][j] = dp[i - 1][j - 1] + 1
        max = Math.max(max, dp[i][j])
      } else {
        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1])
      }
    }
  }
  return max
};
// @lc code=end


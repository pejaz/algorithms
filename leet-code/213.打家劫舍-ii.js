/*
 * @lc app=leetcode.cn id=213 lang=javascript
 *
 * [213] 打家劫舍 II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function (nums) {
  if (nums.length === 1) return nums[0]
  if (nums.length === 2) return Math.max(...nums)
  
  // 不考虑成环情况下的函数
  const robArr = (arr) => {
    const dp = Array(arr.length).fill(0)
    dp[0] = arr[0]
    dp[1] = Math.max(arr[0], arr[1])
    for (let i = 2; i < arr.length; i++) {
      const num = arr[i];
      dp[i] = Math.max(dp[i - 2] + num, dp[i - 1])
    }
    return dp[arr.length - 1]
  }

  // 成环分为两种情况: 1.不考虑尾 2.不考虑首 取最大值即可
  // 不考虑尾的数组
  const nums1 = nums.slice(0, -1)
  // 不考虑首的数组
  const nums2 = nums.slice(1)
  return Math.max(robArr(nums1), robArr(nums2))
};
// @lc code=end


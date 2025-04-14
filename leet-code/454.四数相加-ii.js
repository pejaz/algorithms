/*
 * @lc app=leetcode.cn id=454 lang=javascript
 *
 * [454] 四数相加 II
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @param {number[]} nums3
 * @param {number[]} nums4
 * @return {number}
 */
var fourSumCount = function (nums1, nums2, nums3, nums4) {
  // 转为两数之和
  let count = 0
  const totalMap = new Map()
  for (const item1 of nums1) {
    for (const item2 of nums2) {
      const total = item1 + item2
      let value = totalMap.has(total) ? totalMap.get(total) : 0
      value++
      totalMap.set(total, value)
    }
  }

  for (const item3 of nums3) {
    for (const item4 of nums4) {
      const target = 0 - (item3 + item4)
      if (totalMap.has(target)) {
        count += totalMap.get(target)
      }
    }
  }
  return count
};
// @lc code=end


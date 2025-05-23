/*
 * @lc app=leetcode.cn id=349 lang=javascript
 *
 * [349] 两个数组的交集
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var intersection = function(nums1, nums2) {
  const nums1Set = new Set(nums1);
  const resSet = new Set();
  // for(const n of nums2) {
  //     nums1Set.has(n) && resSet.add(n);
  // }
  // 循环 比 迭代器快
  for(let i = nums2.length - 1; i >= 0; i--) {
      nums1Set.has(nums2[i]) && resSet.add(nums2[i]);
  }
  return Array.from(resSet);
};
// @lc code=end


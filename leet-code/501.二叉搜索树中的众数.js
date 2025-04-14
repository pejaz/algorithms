/*
 * @lc app=leetcode.cn id=501 lang=javascript
 *
 * [501] 二叉搜索树中的众数
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var findMode = function (root) {
  let pre = null
  let count = 1
  let maxCount = 1
  let res = []

  const traversal = (root) => {
    if (!root) return null

    // 左
    traversal(root.left)

    // 中
    if (pre) {
      if (pre.val === root.val) {
        count++
      } else {
        count = 1
      }
    }
    pre = root
    if (count === maxCount) {
      res.push(root.val)
    }
    if (count > maxCount) {
      maxCount = count
      res = res.slice(-1)
    }

    // 右
    traversal(root.right)
  }
  traversal(root)
  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = findMode;
// @after-stub-for-debug-end
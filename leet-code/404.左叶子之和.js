/*
 * @lc app=leetcode.cn id=404 lang=javascript
 *
 * [404] 左叶子之和
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
 * @return {number}
 */
var sumOfLeftLeaves = function (root) {
  if (!root) return 0
  // 叶子节点
  if (!root.left && !root.right) return 0
  // 左
  let leftNum = sumOfLeftLeaves(root.left)
  if (root.left && !root.left.left && !root.left.right) {
    // 如果是左叶子节点的父亲，左边num不再是叶子节点返回的0，而是左叶子节点的val
    leftNum = root.left.val
  }

  // 右，右节点正常递归查找是否存在左叶子节点
  const rightNum = sumOfLeftLeaves(root.right)

  // 中，累加左右节点的和
  return leftNum + rightNum
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = sumOfLeftLeaves;
// @after-stub-for-debug-end
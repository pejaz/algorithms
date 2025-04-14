/*
 * @lc app=leetcode.cn id=222 lang=javascript
 *
 * [222] 完全二叉树的节点个数
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
var countNodes = function (root) {
  // 利用完全二叉树的特性，判断节点是否为满二叉树，然后利用公式 2^n - 1 直接算出节点数
  if (!root) return 0
  let left = root.left;
  let right = root.right;
  let leftDepth = 0, rightDepth = 0;
  while (left) {
    left = left.left;
    leftDepth++;
  }
  while (right) {
    right = right.right;
    rightDepth++;
  }
  if (leftDepth === rightDepth) {
    // 如果左右外侧深度相等，说明是满二叉树，直接用公式计算
    return Math.pow(2, leftDepth + 1) - 1;
  }

  // 不相等则左右孩子数量相加
  const leftCount = countNodes(root.left)
  const rightCount = countNodes(root.right)
  return leftCount + rightCount + 1;
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=110 lang=javascript
 *
 * [110] 平衡二叉树
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
 * @return {boolean}
 */
var isBalanced = function (root) {
  const getHeight = (node) => {
    if (!node) return 0

    // 后序遍历
    // 左
    const leftHight = getHeight(node.left)
    if (leftHight === -1) return -1

    // 右
    const rightHight = getHeight(node.right)
    if (rightHight === -1) return -1

    // 中
    if (Math.abs(leftHight - rightHight) > 1) return -1

    return Math.max(leftHight, rightHight) + 1
  }

  return getHeight(root) !== -1
};
// @lc code=end


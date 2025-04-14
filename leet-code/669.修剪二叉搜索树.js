/*
 * @lc app=leetcode.cn id=669 lang=javascript
 *
 * [669] 修剪二叉搜索树
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
 * @param {number} low
 * @param {number} high
 * @return {TreeNode}
 */
var trimBST = function (root, low, high) {
  if (!root) return null
  if (root.val < low) {
    // 继续修剪右子树并返回给父级。左子树只会比root更小，一定也不满足区间
    const right = trimBST(root.right, low, high)
    return right
  }
  if (root.val > high) {
    // 大于区间修剪左子树
    return trimBST(root.left, low, high)
  }

  root.left = trimBST(root.left, low, high)
  root.right = trimBST(root.right, low, high)
  
  return root
};
// @lc code=end


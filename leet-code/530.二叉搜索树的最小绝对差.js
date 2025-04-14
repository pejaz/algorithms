/*
 * @lc app=leetcode.cn id=530 lang=javascript
 *
 * [530] 二叉搜索树的最小绝对差
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
var getMinimumDifference = function (root) {
  // 利用二叉搜索树，使用中序遍历计算差值

  if (!root) return;
  let pre = null
  let res = Infinity
  const inOrder = (root) => {
    if (root === null) return;
    inOrder(root.left);

    if (pre !== null) {
      res = Math.min(res, root.val - pre.val)
    }

    pre = root;

    inOrder(root.right);
  }
  inOrder(root);
  return res
};
// @lc code=end


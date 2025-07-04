/*
 * @lc app=leetcode.cn id=199 lang=javascript
 *
 * [199] 二叉树的右视图
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
var rightSideView = function (root) {
  if (!root) return []

  const res = [root.val]
  const dfs = (node, idx) => {
    if (!node) return null

    const left = dfs(node.left, idx + 1)
    const right = dfs(node.right, idx + 1)

    if (left || right) {
      res[idx] = right ?? left
    }

    return node.val
  }

  dfs(root, 1)

  return res
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=337 lang=javascript
 *
 * [337] 打家劫舍 III
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
var rob = function (root) {
  const dfs = (node) => {
    if (!node) return [0, 0]

    const left = dfs(node.left)
    const right = dfs(node.right)

    // 偷，孩子不能偷
    const rob = node.val + left[1] + right[1]

    // 不偷的话考虑左右孩子最大值，
    // 孩子有偷和不偷两种状态，偷不偷看他们自己最大值
    const noRob = Math.max(...left) + Math.max(...right)
    return [rob, noRob]
  }

  // 根节点偷不偷的状态,偷: rootRes[0] 不偷: rootRes[1]
  const rootRes = dfs(root)

  return Math.max(rootRes[0], rootRes[1])
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = rob;
// @after-stub-for-debug-end
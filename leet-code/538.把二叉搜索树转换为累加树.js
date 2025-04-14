/*
 * @lc app=leetcode.cn id=538 lang=javascript
 *
 * [538] 把二叉搜索树转换为累加树
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
 * @return {TreeNode}
 */
var convertBST = function (root) {
  let pre = null
  const traversal = (root) => {
    if (!root) return null

    // 从后往前遍历，中序的翻转，右中左
    // 右
    traversal(root.right)
    // 中
    if (pre) {
      root.val += pre.val
    }
    pre = root
    // 左
    traversal(root.left)

    return root
  }
  return traversal(root)
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=513 lang=javascript
 *
 * [513] 找树左下角的值
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
var findBottomLeftValue = function (root) {
  let maxDepth = -Infinity
  let res = 0
  const traversal = (node, depth) => {
    if (!node.left && !node.right) {
      // 叶子节点
      if (depth > maxDepth) {
        // 更新最深左孩子节点的值
        maxDepth = depth
        res = node.val
      }
    }

    if (node.left) {
      // 回溯
      depth++
      traversal(node.left, depth)
      depth--
    }

    if (node.right) {
      traversal(node.right, depth + 1)
    }
  }

  traversal(root, 1)
  return res
};
// @lc code=end


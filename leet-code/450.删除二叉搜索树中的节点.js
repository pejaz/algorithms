/*
 * @lc app=leetcode.cn id=450 lang=javascript
 *
 * [450] 删除二叉搜索树中的节点
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
 * @param {number} key
 * @return {TreeNode}
 */
var deleteNode = function (root, key) {
  if (!root) return null
  // 中
  if (root.val === key) {
    // 找到删除节点
    // 1.节点为叶子节点
    if (!root.left && !root.right) {
      return null
    }
    // 2. 左不空右空
    if (root.left && !root.right) {
      return root.left
    }
    // 3. 左空右不空
    if (root.right && !root.left) {
      return root.right
    }
    // 左右都存在，找到右子树中最接近root的(root.right中的左子树中最大的)
    // 替换当前节点
    let parentNode = root
    let node = root.right
    while (node.left) {
      parentNode = node
      node = node.left
    }
    if (node === root.right) {
      node.left = root.left
    } else {
      // node.left为null，但是node.right可能有节点，直接挂在parentNode的left上
      parentNode.left = node.right
      node.left = root.left
      node.right = root.right
    }
    return node
  }

  // 左
  if (root.val > key) {
    root.left = deleteNode(root.left, key)
  }

  // 右
  if (root.val < key) {
    root.right = deleteNode(root.right, key)
  }
  return root
};
// @lc code=end
// @after-stub-for-debug-begin
module.exports = deleteNode;
// @after-stub-for-debug-end
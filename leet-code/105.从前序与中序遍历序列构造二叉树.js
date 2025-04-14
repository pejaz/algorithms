/*
 * @lc app=leetcode.cn id=105 lang=javascript
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
var buildTree = function (preorder, inorder) {
  if (!preorder.length) return null
  const root = preorder.shift()
  const rootIndex = inorder.indexOf(root)
  const rootNode = new TreeNode(root)
  rootNode.left = buildTree(preorder.slice(0, rootIndex), inorder.slice(0, rootIndex))
  rootNode.right = buildTree(preorder.slice(rootIndex), inorder.slice(rootIndex + 1))
  return rootNode
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=107 lang=javascript
 *
 * [107] 二叉树的层序遍历 II
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
 * @return {number[][]}
 */
var levelOrderBottom = function (root) {
  const queue = [root]
  const res = []

  while (queue.length) {
    console.log(queue)
    let len = queue.length
    const level = []
    while (len--) {
      const node = queue.shift();
      level.push(node.val)
      if (node.left) {
        queue.push(node.left)
      }
      if (node.right) {
        queue.push(node.right)
      }
    }

    if (level.length) {
      res.push(level)
    }
  }
  console.log(res)
  return res.reverse()
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = levelOrderBottom;
// @after-stub-for-debug-end
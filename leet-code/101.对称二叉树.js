/*
 * @lc app=leetcode.cn id=101 lang=javascript
 *
 * [101] 对称二叉树
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
var isSymmetric = function (root) {
  if (!root) return true

  // 使用递归遍历左右子树 递归三部曲
  // 1. 确定递归的参数 root.left root.right和返回值true false 
  const compareNode = (left, right) => {
    // 2. 确定终止条件 空的情况
    if ((left === null && right !== null) || (left !== null && right === null)) {
      return false;
    }
    if (left === null && right === null) {
      return true;
    }
    if (left.val !== right.val) {
      return false;
    }
    // 3. 确定单层递归逻辑
    // 外侧: 左节点的左孩子<===>右节点的右孩子
    let outSide = compareNode(left.left, right.right);
    // 内侧: 左节点的右孩子<===>右节点的左孩子
    let inSide = compareNode(left.right, right.left);
    return outSide && inSide;
  };
  return compareNode(root.left, root.right)
}
// @lc code=end


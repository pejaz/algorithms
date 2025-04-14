/*
 * @lc app=leetcode.cn id=235 lang=javascript
 *
 * [235] 二叉搜索树的最近公共祖先
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
  if (!root) return null

  // 左，root比pq都大，往左节点找
  if (root.val > p.val && root.val > q.val) {
    return lowestCommonAncestor(root.left, p, q)
  }

  // 右，root比pq都小，往右节点找
  if (root.val < p.val && root.val < q.val) {
    return lowestCommonAncestor(root.right, p, q)
  }

  // root在pq中间，说明一个在root左边，另一个在root右边
  // 则root为最近公共祖先
  return root
};
// @lc code=end


/*
 * @lc app=leetcode.cn id=337 lang=rust
 *
 * [337] 打家劫舍 III
 */
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  后序遍历，每个节点有偷最大值和不偷最大值提供给父节点参考
     *  考虑偷节点 node : node.val + node.left 不偷+node.right 不偷
     *  考虑不偷节点 node : node.left 偷不偷取最大  + node.right 偷不偷取最大
     *  最终 node = max(偷，不偷)
     */
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn call_rob(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = call_rob(&node.left);
                    let right = call_rob(&node.right);

                    // 偷 node + 不偷 left + 不偷 right
                    let rob = node.val + left.1 + right.1;
                    // 不偷 node + left 偷不偷都行，取最大值 + right 偷不偷都行，取最大值
                    let not_rob = left.0.max(left.1) + right.0.max(right.1);

                    return (rob, not_rob);
                }
                None => (0, 0),
            }
        }

        let root = call_rob(&root);

        return root.0.max(root.1);
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_rob() {
        //assert_eq!(Solution::rob(vec![]),[]);
        assert!(true)
    }
}

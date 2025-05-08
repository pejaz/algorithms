/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                // 叶子节点
                return target_sum == node.val;
            }

            let left = node.left.clone();
            let left_sum = Self::has_path_sum(left, target_sum - node.val);

            let right = node.right.clone();
            let right_sum = Self::has_path_sum(right, target_sum - node.val);

            return left_sum || right_sum;
        }

        return false;
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
    fn test_has_path_sum() {
        //assert_eq!(Solution::has_path_sum(vec![]),[]);
        assert!(true)
    }
}

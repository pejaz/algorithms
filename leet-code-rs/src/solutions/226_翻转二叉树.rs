/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
 */
// #[derive(Debug, PartialEq, Eq)]
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = Self::invert_tree(node.borrow_mut().left.take()); // 翻转左子树
            let right = Self::invert_tree(node.borrow_mut().right.take()); // 翻转右子树
            node.borrow_mut().left = right; // 交换左右儿子
            node.borrow_mut().right = left;
            Some(node)
        } else {
            None
        }
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
    fn test_invert_tree() {
        //assert_eq!(Solution::invert_tree(vec![]),[]);
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=700 lang=rust
 *
 * [700] 二叉搜索树中的搜索
 */
#[derive(Debug, PartialEq, Eq)]
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
use std::cmp::Ordering::{Equal, Greater, Less};
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            // 二叉搜索树，左边比根节点小，右边比根节点大
            Some(node) => {
                let v = node.as_ref().borrow().val;

                return match v.cmp(&val) {
                    Less => Self::search_bst(node.borrow_mut().right.take(), val),
                    Greater => Self::search_bst(node.borrow_mut().left.take(), val),
                    Equal => Some(node),
                };
            }
            None => None,
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
    fn test_search_bst() {
        //assert_eq!(Solution::search_bst(vec![]),[]);
        assert!(true)
    }
}

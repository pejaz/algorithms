/*
 * @lc app=leetcode.cn id=669 lang=rust
 *
 * [669] 修剪二叉搜索树
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
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node_rc) => {
                let val = node_rc.borrow().val;
                if val > high {
                    let left = node_rc.borrow_mut().left.take();
                    Self::trim_bst(left, low, high)
                } else if val < low {
                    let right = node_rc.borrow_mut().right.take();
                    Self::trim_bst(right, low, high)
                } else {
                    let left = Self::trim_bst(node_rc.borrow_mut().left.take(), low, high);
                    let right = Self::trim_bst(node_rc.borrow_mut().right.take(), low, high);

                    node_rc.borrow_mut().left = left;
                    node_rc.borrow_mut().right = right;

                    Some(node_rc)
                }
            }
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
    fn test_trim_bst() {
        //assert_eq!(Solution::trim_bst(vec![]),[]);
        assert!(true)
    }
}

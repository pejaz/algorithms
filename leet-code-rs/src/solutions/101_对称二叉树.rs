/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
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
    pub fn is_match(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                if l.borrow().val != r.borrow().val {
                    return false;
                }
                Self::is_match(&l.borrow().left, &r.borrow().right)
                    && Self::is_match(&l.borrow().right, &r.borrow().left)
            }
            (None, None) => true,
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.unwrap();
        let left = &root.borrow().left;
        let right = &root.borrow().right;

        Self::is_match(left, right)
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
    fn test_is_symmetric() {
        //assert_eq!(Solution::is_symmetric(vec![]),[]);
        assert!(true)
    }
}

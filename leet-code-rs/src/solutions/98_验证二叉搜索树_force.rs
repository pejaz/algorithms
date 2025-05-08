/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn get_val(node: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let mut max_v = i32::MIN;
        let mut min_v = i32::MAX;
        let mut queue = VecDeque::new();

        queue.push_back(node);

        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            max_v = max_v.max(node.val);
            min_v = min_v.min(node.val);

            if let Some(left) = &node.left {
                queue.push_back(Rc::clone(left));
            }

            if let Some(right) = &node.right {
                queue.push_back(Rc::clone(right));
            }
        }

        (min_v, max_v)
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                let left = &node.borrow().left;
                let right = &node.borrow().right;
                match (left, right) {
                    (Some(l), Some(r)) => {
                        // 根节点大于左边最大值且小于右边最小值
                        if val > Self::get_val(l.clone()).1 && val < Self::get_val(r.clone()).0 {
                            return Self::is_valid_bst(Some(l.clone()))
                                && (Self::is_valid_bst(Some(r.clone())));
                        } else {
                            return false;
                        }
                    }
                    (Some(l), None) => {
                        if val > Self::get_val(l.clone()).1 {
                            return Self::is_valid_bst(Some(l.clone()));
                        } else {
                            return false;
                        }
                    }
                    (None, Some(r)) => {
                        if val < Self::get_val(r.clone()).0 {
                            return Self::is_valid_bst(Some(r.clone()));
                        } else {
                            return false;
                        }
                    }
                    _ => true,
                }
            }
            None => false,
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
    fn test_is_valid_bst() {
        //assert_eq!(Solution::is_valid_bst(vec![]),[]);
        assert!(true)
    }
}

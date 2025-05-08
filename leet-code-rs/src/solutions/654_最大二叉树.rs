/*
 * @lc app=leetcode.cn id=654 lang=rust
 *
 * [654] 最大二叉树
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let max_idx = nums
            .iter()
            .enumerate()
            .max_by_key(|&(_, &n)| n)
            .map(|(i, _)| i)?;

        let mut root = TreeNode::new(nums[max_idx]);
        let (l, mut r) = nums.split_at(max_idx);
        r = &r[1..];

        root.left = Self::construct_maximum_binary_tree(l.to_vec());
        root.right = Self::construct_maximum_binary_tree(r.to_vec());

        return Some(Rc::new(RefCell::new(root)));
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
    fn test_construct_maximum_binary_tree() {
        //assert_eq!(Solution::construct_maximum_binary_tree(vec![]),[]);
        assert!(true)
    }
}

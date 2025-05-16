/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let idx = nums.len() / 2;
        let mut root = TreeNode::new(nums[idx]);
        let (left, right) = nums.split_at(idx);
        root.left = Self::sorted_array_to_bst(left.to_vec());
        root.right = Self::sorted_array_to_bst(right[1..].to_vec());

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
    fn test_sorted_array_to_bst() {
        //assert_eq!(Solution::sorted_array_to_bst(vec![]),[]);
        assert!(true)
    }
}

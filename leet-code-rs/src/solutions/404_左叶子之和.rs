/*
 * @lc app=leetcode.cn id=404 lang=rust
 *
 * [404] 左叶子之和
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_left_leaves(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, collected: bool) {
            if let Some(node) = node {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() && collected {
                    // 左叶子
                    *sum += node.val;
                    return;
                }

                let left = &node.left;
                get_left_leaves(left, sum, true);

                let right = &node.right;
                get_left_leaves(right, sum, false);
            }
        }

        let mut sum = 0;
        get_left_leaves(&root, &mut sum, false);

        return sum;
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
    fn test_sum_of_left_leaves() {
        //assert_eq!(Solution::sum_of_left_leaves(vec![]),[]);
        assert!(true)
    }
}

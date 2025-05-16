/*
 * @lc app=leetcode.cn id=538 lang=rust
 *
 * [538] 把二叉搜索树转换为累加树
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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn bst(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = node {
                // 先右
                bst(&node.borrow().right, sum);

                // 中
                *sum += node.borrow().val;
                node.borrow_mut().val = *sum;

                // 左
                bst(&node.borrow().left, sum);
            }
        }

        let mut sum = 0;
        bst(&root, &mut sum);

        return root;
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
    fn test_convert_bst() {
        //assert_eq!(Solution::convert_bst(vec![]),[]);
        assert!(true)
    }
}

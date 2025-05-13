/*
 * @lc app=leetcode.cn id=530 lang=rust
 *
 * [530] 二叉搜索树的最小绝对差
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 中序遍历
        fn inorder_traversal(
            root: &Option<Rc<RefCell<TreeNode>>>,
            pre: &mut Option<i32>,
            min_num: &mut i32,
        ) {
            if let Some(node) = root {
                let node = node.borrow();

                inorder_traversal(&node.left, pre, min_num);

                if let Some(pre) = pre {
                    *min_num = *min_num.min(&mut (node.val - *pre));
                }
                *pre = Some(node.val);

                inorder_traversal(&node.right, pre, min_num);
            }
        }

        let mut pre = None;
        let mut min_num = i32::MAX;
        inorder_traversal(&root, &mut pre, &mut min_num);

        return min_num;
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
    fn test_get_minimum_difference() {
        //assert_eq!(Solution::get_minimum_difference(vec![]),[]);
        assert!(true)
    }
}

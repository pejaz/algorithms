/*
 * @lc app=leetcode.cn id=513 lang=rust
 *
 * [513] 找树左下角的值
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find_value(
            node: &Option<Rc<RefCell<TreeNode>>>,
            v: &mut i32,
            depth: &mut i32,
            cur_depth: i32,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                if &cur_depth > depth {
                    *v = node.val;
                    *depth = cur_depth;
                }

                let left = &node.left;
                find_value(left, v, depth, cur_depth + 1);

                let right = &node.right;
                find_value(right, v, depth, cur_depth + 1);
            }
        }

        let mut val = 0;
        let mut depth = 0;
        find_value(&root, &mut val, &mut depth, 1);

        return val;
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
    fn test_find_bottom_left_value() {
        //assert_eq!(Solution::find_bottom_left_value(vec![]),[]);
        assert!(true)
    }
}

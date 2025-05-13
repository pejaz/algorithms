/*
 * @lc app=leetcode.cn id=501 lang=rust
 *
 * [501] 二叉搜索树中的众数
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder_search(
            node: &Option<Rc<RefCell<TreeNode>>>,
            pre: &mut i32,
            count: &mut i32,
            max_count: &mut i32,
            res: &mut Vec<i32>,
        ) {
            if let Some(node) = node {
                let node = node.borrow();

                inorder_search(&node.left, pre, count, max_count, res);

                if node.val == *pre {
                    *count += 1;
                } else {
                    *count = 1;
                }
                *pre = node.val;

                if count == max_count {
                    res.push(node.val);
                }

                if count > max_count {
                    res.clear();
                    *max_count = *count;
                    res.push(node.val);
                }

                inorder_search(&node.right, pre, count, max_count, res);
            }
        }

        let mut pre = i32::MAX;
        let mut count = 1;
        let mut max_count = 1;
        let mut res = vec![];
        inorder_search(&root, &mut pre, &mut count, &mut max_count, &mut res);

        return res;
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
    fn test_find_mode() {
        //assert_eq!(Solution::find_mode(vec![]),[]);
        assert!(true)
    }
}

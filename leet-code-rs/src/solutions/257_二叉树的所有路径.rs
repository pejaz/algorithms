/*
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = vec![];
        fn get_paths(
            node: &Option<Rc<RefCell<TreeNode>>>,
            mut path: Vec<String>,
            res: &mut Vec<String>,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                path.push(node.val.to_string());

                // 叶子节点搜集
                if node.left.is_none() && node.right.is_none() {
                    res.push(path.join("->"));
                    return;
                }

                let left = &node.left;
                get_paths(left, path.clone(), res);

                let right = &node.right;
                get_paths(right, path.clone(), res);
            }
        }

        get_paths(&root, vec![], &mut res);

        res
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
    fn test_binary_tree_paths() {
        //assert_eq!(Solution::binary_tree_paths(vec![]),[]);
        assert!(true)
    }
}

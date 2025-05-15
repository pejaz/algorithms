/*
 * @lc app=leetcode.cn id=450 lang=rust
 *
 * [450] 删除二叉搜索树中的节点
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
use std::cmp::Ordering::{Equal, Greater, Less};
use std::rc::Rc;
#[allow(unused)]
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            let mut node = root.as_ref()?.borrow_mut();
            return match &node.val.cmp(&key) {
                Equal => match (node.left.take(), node.right.take()) {
                    (Some(left), Some(right)) => {
                        // 找到右子树的最左边节点，将左子树挂在最左边节点的左子节点，然后返回右子树
                        let mut node = right.clone();
                        while node.borrow().left.is_some() {
                            let next = node.borrow().left.clone();
                            node = next.unwrap();
                        }

                        node.borrow_mut().left = Some(left);

                        return Some(right);
                    }
                    (Some(node), None) | (None, Some(node)) => Some(node),
                    (None, None) => None,
                },
                Greater => {
                    node.left = Self::delete_node(node.left.take(), key);
                    return root.clone();
                }
                Less => {
                    node.right = Self::delete_node(node.right.take(), key);
                    return root.clone();
                }
            };
        }
        root
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
    fn test_delete_node() {
        //assert_eq!(Solution::delete_node(vec![]),[]);
        assert!(true)
    }
}

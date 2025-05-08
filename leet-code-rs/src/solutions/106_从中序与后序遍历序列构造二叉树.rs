/*
 * @lc app=leetcode.cn id=106 lang=rust
 *
 * [106] 从中序与后序遍历序列构造二叉树
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
use std::collections::HashSet;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }

        // 后序遍历的最后一个为根节点
        let val = postorder.pop()?;
        let i_idx = inorder.iter().position(|&n| n == val)?;
        // 根节点前面为左子树的 inorder，后面为右子树的 inorder
        let (l_inorder, r_inorder) = inorder.split_at(i_idx);
        let (_, r_inorder) = r_inorder.split_at(1);

        // 中序遍历的最后一个为左子树的最后一个节点。
        let l_order_set = l_inorder.iter().collect::<HashSet<_>>();
        let (l_postorder, r_postorder, _) = postorder.iter().fold(
            (vec![], vec![], l_order_set),
            |(mut l_post, mut r_post, l_set), n| {
                if l_set.contains(n) {
                    l_post.push(*n);
                } else {
                    r_post.push(*n);
                }

                (l_post, r_post, l_set)
            },
        );
        let mut node = TreeNode::new(val);
        node.left = Self::build_tree(l_inorder.to_vec(), l_postorder);
        node.right = Self::build_tree(r_inorder.to_vec(), r_postorder);

        return Some(Rc::new(RefCell::new(node)));
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
    fn test_build_tree() {
        // Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        // Solution::build_tree(vec![1, 2], vec![2, 1]);
        Solution::build_tree(vec![2, 3, 1], vec![3, 2, 1]);
        assert!(true)
    }
}

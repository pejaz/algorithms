/*
 * @lc app=leetcode.cn id=701 lang=rust
 *
 * [701] 二叉搜索树中的插入操作
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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        // 写法一：通过 {} 包裹一层作用域
        // {
        //     let mut root = root.as_ref()?.borrow_mut();
        //     if val < root.val {
        //         root.left = Self::insert_into_bst(root.left.take(), val);
        //     } else {
        //         root.right = Self::insert_into_bst(root.right.take(), val)
        //     }
        // }

        // 写法二：
        let r = root.as_ref()?;
        if val < r.borrow().val {
            let left = Self::insert_into_bst(r.borrow_mut().left.take(), val);
            r.borrow_mut().left = left;
        } else {
            let right = Self::insert_into_bst(r.borrow_mut().right.take(), val);
            r.borrow_mut().right = right;
        }

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
    fn test_insert_into_bst() {
        //assert_eq!(Solution::insert_into_bst(vec![]),[]);
        assert!(true)
    }
}

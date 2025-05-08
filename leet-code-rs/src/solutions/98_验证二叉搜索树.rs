/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 二叉搜索树按照中序遍历-左中右的顺序的值一定是依次递增的。
        // 本题用迭代法会更简单，中序遍历过程判断后一个节点一定大于前一个节点即可。
        fn is_valid(node: Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>) -> bool {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    // 左
                    let left = is_valid(node.left.clone(), pre);

                    // 中,这里 max_v 应该要依次递增
                    // if &node.val <= max_v && *is_init == true {
                    //     return false;
                    // }
                    // *is_init = true;
                    // *max_v = node.val;
                    if let Some(pre) = pre {
                        // 前一个节点一定要比后一个节点小，否则就不是搜索树
                        if &node.val <= pre {
                            return false;
                        }
                    }
                    *pre = Some(node.val);

                    // 右
                    let right = is_valid(node.right.clone(), pre);

                    return left && right;
                }
                None => true,
            }
        }

        // let mut max_v = i32::MIN;
        // let mut is_init = false;
        // 简化为一个 pre
        let mut pre = None;
        return is_valid(root, &mut pre);
    }
    // 解法二：递归过程不断更新上下边界，直接判断上一个节点是否满足
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, lower: i32, upper: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.val <= lower || node.val >= upper {
                    return false;
                }

                // 当前节点一定是左子树的最大值，右子树的最小值
                return Self::helper(node.left.clone(), lower, node.val)
                    && Self::helper(node.right.clone(), node.val, upper);
            }
            None => true,
        }
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
    fn test_is_valid_bst() {
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(
            Solution::is_valid_bst(Some(Rc::new(RefCell::new(tree)))),
            false
        );
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=968 lang=rust
 *
 * [968] 监控二叉树
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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //  存在孩子为 0 的需要加监控
        // （0: 无覆盖，父亲需加摄像头；1: 有覆盖；2: 装摄像头节点）
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, camera: &mut i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();

                let left = traverse(&node.left, camera);
                let right = traverse(&node.right, camera);

                // 存在孩子为0 时，要加监控，否则会漏掉该孩子
                if left == 0 || right == 0 {
                    *camera += 1;
                    return 2; // 往上监控 2 层
                } else {
                    return left.max(right) - 1;
                }
            } else {
                // 代表在监控覆盖范围内，此时叶子节点会在此基础上 -1 变为 0
                return 1;
            }
        }

        let mut camera = 0;
        let root = traverse(&root, &mut camera);

        // 根节点为 0 时 要特殊处理加上监控，否则根节点没有父亲加监控了会被漏掉
        if root == 0 {
            camera += 1;
        }

        return camera;
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
    fn test_min_camera_cover() {
        // assert_eq!(Solution::min_camera_cover(vec![]), []);
        assert!(true)
    }
}

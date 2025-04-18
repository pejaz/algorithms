/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tmp = &mut dummy;

        while let Some(mut n1) = head {
            let mut temp = None;
            // n1 为 1 号节点
            let nxt = n1.next.take(); // take()将n1打断，这样n1只有一个值，返回值是除n1节点外的剩余节点
            if let Some(mut n2) = nxt {
                // n2 为 2 号节点, 1 =>3
                temp = n2.next.take(); // take()将n2打断，n2只有一个值，返回值是除n2节点外的剩余节点

                // n1 和 n2 都存在，则翻转
                // 2 => 1
                n2.next = Some(n1);
                // dummy_head 指向翻转后的头，即 2 号节点,后续也会变成 1 号指向 4号节点
                // 这里替代 n1.next = temp，避免所有权转变
                tmp.next = Some(n2);
                // 走两步, tmp => 2 => 1，翻转 1 后面的 3 和 4，
                tmp = tmp.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                // 恢复 n1 节点指向
                tmp.next = Some(n1);
            }

            head = temp;
        }
        dummy.next
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        //assert_eq!(Solution::swap_pairs(vec![]),[]);
        assert!(true)
    }
}

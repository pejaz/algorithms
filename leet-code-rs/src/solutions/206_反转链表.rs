/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */
#[allow(unused)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
#[allow(unused)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre: Option<Box<ListNode>> = None;
        let mut next = head;
        while let Some(mut node) = next.take() {
            let nxt = node.next.take();
            node.next = pre;
            pre = Some(node);
            next = nxt;
        }

        return pre;
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
    fn test_reverse_list() {
        // assert_eq!(Solution::reverse_list(vec![]),[]);
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummyHead = Box::new(ListNode::new(0));
        dummyHead.next = head;

        let mut cur = dummyHead.as_mut();
        while let Some(next) = cur.next.take() {
            if next.val == val {
                cur.next = next.next;
            } else {
                // cur.next已经被 take() 释放了，这里需要重新指向回来
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap();
            }
        }

        return dummyHead.next;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_elements() {
        //assert_eq!(Solution::remove_elements(vec![]),[]);
        assert!(true)
    }
}

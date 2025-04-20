/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>,  n: i32) -> Option<Box<ListNode>> {
        let dummy = ListNode { val: 0, next: head };
        let mut slow = &dummy;
        let mut fast = &dummy;

        // 右指针先向右走 n 步
        for _ in 0..n {
            fast = fast.next.as_ref()?;
        }

        while let Some(ref node) = fast.next {
            slow = slow.next.as_ref().unwrap();
            fast = node;
        }

        // 这里需要把 slow 从 &ListNode 强转成 &mut ListNode
        #[allow(mutable_transmutes)]
        let slow: &mut ListNode = unsafe { std::mem::transmute(slow) };
        slow.next = slow.next.take()?.next;

        return dummy.next;
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
    fn test_remove_nth_from_end() {
        //assert_eq!(Solution::remove_nth_from_end(vec![]),[]);
        assert!(true)
    }
}

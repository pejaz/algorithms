/*
 * @lc app=leetcode.cn id=19 lang=javascript
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
var removeNthFromEnd = function (head, n) {
  let dummy = new ListNode(0, head)
  let fast = slow = dummy
  // 快指针先前进 n 步,此时 fast 指向删除节点
  while (n) {
    fast = fast.next;
    n--
  }

  // 让慢指针和快指针同步向前，
  // 因为要操作前一个节点，所以 fast 走到最后一个就可以了,不用走到null
  while (fast && fast.next) {
    fast = fast.next;
    slow = slow.next;
  }

  // 此时 slow 为删除的前置节点，slow.next 就是倒数第 n 个节点，删除它
  slow.next = slow.next.next;

  return dummy.next;
};
// @lc code=end


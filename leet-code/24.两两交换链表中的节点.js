/*
 * @lc app=leetcode.cn id=24 lang=javascript
 *
 * [24] 两两交换链表中的节点
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
 * @return {ListNode}
 */
var swapPairs = function(head) {
  const dummyHead = new ListNode(0,head)
  let cur = dummyHead
  while(cur.next && cur.next.next){
    // 1号节点
    const pre = cur.next
    // 2号节点，
    const next = cur.next.next
    // 3 号节点
    const temp = next.next

    // cur => 2
    cur.next = next
    // 翻转 1 和 2 号节点
    // 2 => 1,
    next.next = pre
    // 1 => 3,
    pre.next = temp

    // 新的cur -> 1,cur 的后面两个翻转，即翻转 4 和 3
    cur = pre
  }

  return dummyHead.next
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = swapPairs;
// @after-stub-for-debug-end
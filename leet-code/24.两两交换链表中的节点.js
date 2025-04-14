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
    const temp1 = cur.next
    // 3号节点
    const temp2 = cur.next.next.next

    // cur => 2
    cur.next = cur.next.next
    // 2 => 1,翻转
    cur.next.next = temp1
    // 1 => 3, 下一个翻转
    temp1.next = temp2

    cur = cur.next.next
  }
  return dummyHead.next
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = swapPairs;
// @after-stub-for-debug-end
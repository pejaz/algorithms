/*
 * @lc app=leetcode.cn id=142 lang=javascript
 *
 * [142] 环形链表 II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var detectCycle = function(head) {
  // 代码随想录，x = (n-1)(z+y) + z，令n=1,则x = z 
    let fast = head
    let slow = head
    while(fast && fast.next){
      fast = fast.next.next
      slow = slow.next
      if(fast === slow){
        let node1 = head
        let node2 = slow
        // 分别从起点和快慢指针接触点以相同速度起跑，最终会在入口相遇
        while(node1 !== node2){
          node1 = node1.next
          node2 = node2.next
        }
        return node1
      }
    }
    return null
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = detectCycle;
// @after-stub-for-debug-end
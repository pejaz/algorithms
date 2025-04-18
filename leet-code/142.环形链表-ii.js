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
  // 设入环前距离为 x 个节点，慢指针入环又走了 y 个节点相遇，
  // 相遇节点里入环差 z 个节点相遇，则
  // 慢指针走了：slow = x + y
  // 快指针走了：fast = x + n * (y + z) + y ，
  // n 为相遇前快指针在环内绕的圈数
  // 又因速度 fast = 2 * slow
  // 所以有 x + y + n * (y + z) = 2 * (x + y)
  //    => n * (y + z) = x + y
  //    => x = n * (y + z) - y
  //    => x = (n-1)(z+y) + z，令n=1,则 x = z 
    let fast = head
    let slow = head
    while(fast && fast.next){
      fast = fast.next.next
      slow = slow.next
      if(fast === slow){
        let node1 = head
        let node2 = slow
        // x = z，所以分别从起点和快慢指针接触点以相同速度起跑
        // 最终会在环入口相遇
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
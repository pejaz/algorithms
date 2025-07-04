/*
 * @lc app=leetcode.cn id=25 lang=javascript
 *
 * [25] K 个一组翻转链表
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
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function (head, k) {
  const reverseList = (node) => {
    if (!node) {
      return node
    }

    let pre = null
    while (node) {
      let next = node.next

      node.next = pre
      pre = node
      node = next
    }

    return pre
  }

  let fast = new ListNode(null, head)
  let res = null
  while (fast) {
    let slow = fast

    let step = k
    while (fast && step--) {
      fast = fast.next
    }

    if (fast) {
      let temp = fast.next // 3
      fast.next = null

      let tail = reverseList(slow.next)
      // 此时 fast = tail = 2, slow.next = 1。
      if (!res) {
        res = tail
      }

      // fast => 1
      fast = slow.next
      // 1 => 3, 3 => 5
      fast.next = temp
      // d => 2, 2 => 4
      slow.next = tail
    }
  }

  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = reverseKGroup;
// @after-stub-for-debug-end
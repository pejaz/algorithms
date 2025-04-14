/*
 * @lc app=leetcode.cn id=239 lang=javascript
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var maxSlidingWindow = function (nums, k) {
  // 单调队列
  class DeQueue {
    queue = []
    pop (value) {
      if (this.queue[0] === value) {
        this.queue.shift()
      }
    }
    push (value) {

      while (this.queue.length) {
        let lastItem = this.queue.pop()
        if (lastItem >= value) {
          this.queue.push(lastItem)
          break
        }
      }
      this.queue.push(value)
    }
    front () {
      return this.queue[0] || 0
    }
  }

  const dequeue = new DeQueue()
  const res = []
  for (let i = 0; i < nums.length; i++) {
    const el = nums[i];
    dequeue.push(el)
    if (i > k - 1) {
      dequeue.pop(nums[i - k])
    }
    if (i >= k - 1) {
      res.push(dequeue.front())
    }
  }
  return res
};
// @lc code=end


// @after-stub-for-debug-begin
module.exports = maxSlidingWindow;
// @after-stub-for-debug-end
/*
 * @lc app=leetcode.cn id=707 lang=javascript
 *
 * [707] 设计链表
 */

// @lc code=start

class LinkNode {
  constructor(val, next) {
    this.val = val;
    this.next = next;
  }
}

var MyLinkedList = function () {
  this._size = 0;
  this._tail = null;
  this._head = null;
};

/** 
 * @param {number} index
 * @return {number}
 */
MyLinkedList.prototype.get = function (index) {
  if (index < 0 || index >= this._size) return -1
  let cur = this._head
  while (index) {
    cur = cur.next
    index--
  }
  return cur.val
};

/** 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtHead = function (val) {
  const node = new LinkNode(val, this._head);
  this._size++
  this._head = node
  if (!this._tail) {
    this._tail = node
  }
};

/** 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtTail = function (val) {
  const node = new LinkNode(val, null);
  this._size++
  if (this._tail) {
    this._tail.next = node
  } else {
    // 初始化
    this._head = node
  }
  this._tail = node
};

/** 
 * @param {number} index 
 * @param {number} val
 * @return {void}
 */
MyLinkedList.prototype.addAtIndex = function (index, val) {
  if (index > this._size) return
  if (index <= 0) {
    this.addAtHead(val);
    return;
  }
  if (index === this._size) {
    this.addAtTail(val);
    return;
  }
  let cur = new LinkNode(0, this._head)
  while (index) {
    cur = cur.next
    index--
  }
  const node = new LinkNode(val, cur.next)
  cur.next = node
  this._size++
};

/** 
 * @param {number} index
 * @return {void}
 */
MyLinkedList.prototype.deleteAtIndex = function (index) {
  if (index < 0 || index >= this._size) return;

  let cur = new LinkNode(0, this._head)
  let n = index
  while (n) {
    cur = cur.next
    n--
  }

  cur.next = cur.next.next

  if(index === 0){
    this._head = cur.next
  }
  if (index === this._size - 1) {
    this._tail = cur
  }

  this._size--
  if (!this._size) {
    this._tail = null
  }
};

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * var obj = new MyLinkedList()
 * var param_1 = obj.get(index)
 * obj.addAtHead(val)
 * obj.addAtTail(val)
 * obj.addAtIndex(index,val)
 * obj.deleteAtIndex(index)
 */
// @lc code=end


// @after-stub-for-debug-begin
module.exports = MyLinkedList;
// @after-stub-for-debug-end
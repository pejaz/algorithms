/*
 * @lc app=leetcode.cn id=146 lang=javascript
 *
 * [146] LRU 缓存
 */

// @lc code=start
class ListNode {
  constructor(key, value) {
    this.key = key
    this.value = value
    this.next = null
    this.prev = null
  }
  setNext (listNode) {
    this.next = listNode
  }
  setPrev (listNode) {
    this.prev = listNode
  }
}

/**
 * @param {number} capacity
 */
var LRUCache = function (capacity) {
  this.capacity = capacity
  this.cache = Object.create(null)
  this.count = 0
  this.head = new ListNode()
  this.tail = new ListNode()
  this.head.setNext(this.tail)
  this.tail.setPrev(this.head)
};

/** 
 * @param {number} key
 * @return {number}
 */
LRUCache.prototype.get = function (key) {
  if (this.cache[key]) {
    const node = this.cache[key]

    this.removeNode(node)
    this.addToHead(node)

    return node.value
  }

  return -1
};

/** 
 * @param {number} key 
 * @param {number} value
 * @return {void}
 */
LRUCache.prototype.put = function (key, value) {
  let node = null
  if (this.cache[key]) {
    node = this.cache[key]
    node.value = value

    this.removeNode(node)
  } else {
    node = new ListNode(key, value)
    this.cache[key] = node
  }


  this.addToHead(node)
  if (this.count > this.capacity) {
    const tailNode = this.tail.prev
    this.removeNode(tailNode)
    this.cache[tailNode.key] = null
  }
  return null
};

LRUCache.prototype.removeNode = function (node) {
  node.prev.setNext(node.next)
  node.next.setPrev(node.prev)
  this.count -= 1
}

LRUCache.prototype.addToHead = function (node) {
  node.setNext(this.head.next)
  node.setPrev(this.head)
  this.head.next.setPrev(node)
  this.head.setNext(node)
  this.count += 1
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */
// @lc code=end


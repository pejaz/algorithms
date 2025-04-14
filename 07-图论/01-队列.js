function Queue() {
  //属性
  this.items = [];

  //队列的相关操作
  /*
   * 1. enqueue(element): 向队列尾部添加一个（或多个）新元素
   * 2. dequeue(): 移除队列的第一（即排在队列最前面的）项，同时返回被移出的元素
   * 3. front(): 返回队列第一个的元素，不对队列做任何修改(这个方法不会移除队列前端的元素，仅仅是返回它)
   * 4. isEmpty(): 如果队列没有任何元素就返回true，否则返回false
   * 5. size(): 返回队列的元素个数，这个方法和数组的length属性很类似
   * 6. toString(): 将队列结构的内容以字符形式返回
   */

  // 方法
  // 1. enqueue()
  Queue.prototype.enqueue = function (element) {
    this.items.push(element);
  };
  // 2. dequeue()
  Queue.prototype.dequeue = function () {
    return this.items.shift();
  };
  // 3. front()
  Queue.prototype.front = function () {
    return this.items[0];
  };
  // 4. isEmpty()
  Queue.prototype.isEmpty = function () {
    return this.items.length === 0;
  };
  // 5. size()
  Queue.prototype.size = function () {
    return this.items.length;
  };
  //  6. toString()
  Queue.prototype.toString = function () {
    return this.items.join(" ");
  };
}

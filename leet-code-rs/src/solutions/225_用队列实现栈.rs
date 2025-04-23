/*
 * @lc app=leetcode.cn id=225 lang=rust
 *
 * [225] 用队列实现栈
 */

// @lc code=start
use std::collections::VecDeque;
struct MyStack {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut size = self.queue.len() - 1;

        while size > 0 {
            let x = self.queue.pop_front().unwrap();
            self.queue.push_back(x);
            size -= 1;
        }

        self.queue.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        let x = self.pop();
        self.push(x);
        x
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

#[allow(unused)]
#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_fn() {
        //assert_eq!(Solution::fn(vec![]),[]);
        assert!(true)
    }
}

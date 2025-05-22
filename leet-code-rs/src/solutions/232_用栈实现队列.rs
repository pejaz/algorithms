/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] 用栈实现队列
 */

// @lc code=start
struct MyQueue {
    stack: Vec<i32>,
    out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: vec![],
            out: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x)
    }

    fn pop(&mut self) -> i32 {
        if self.out.is_empty() {
            while !self.stack.is_empty() {
                self.out.push(self.stack.pop().unwrap())
            }
        }
        
        self.out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out.is_empty() {
            while !self.stack.is_empty() {
                self.out.push(self.stack.pop().unwrap())
            }
        }

        *self.out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty() && self.out.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
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

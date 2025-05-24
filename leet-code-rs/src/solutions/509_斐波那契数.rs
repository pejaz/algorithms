/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] 斐波那契数
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        // match n {
        //     0 => 0,
        //     1 => 1,
        //     n => Self::fib(n - 1) + Self::fib(n - 2),
        // }

        if n <= 1 {
            return n;
        }

        // 1. 定义 DP 数组
        let mut dp = vec![0; n as usize + 1];
        // 2. 初始化
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2]
        }

        return dp[n as usize];
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_fib() {
        //assert_eq!(Solution::fib(vec![]),[]);
        assert!(true)
    }
}

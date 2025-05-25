/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }

        // 1. 定义DP数组 dp[n] 代表爬 n 阶楼梯的所有方法
        let mut dp = vec![0; n as usize + 1];
        // 2. 递推公式 dp[n] = dp[n-1] + dp[n-2]，dp[n]可以由 n-1 爬 1 阶或者 n-2 爬 2 阶到达
        // 3. 初始化
        dp[0] = 1;
        dp[1] = 1;
        // dp[2] = dp[1] + dp[0];

        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
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
    fn test_climb_stairs() {
        //assert_eq!(Solution::climb_stairs(vec![]),[]);
        assert!(true)
    }
}

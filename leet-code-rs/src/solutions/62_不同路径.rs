/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // 1. dp[m][n] 代表 走到下标[m][n]的总路径次数
        // 2. dp[m][n] = dp[m][n-1](向右移动一步) + dp[m-1][n](向下移动一步) 到达
        let mut dp = vec![vec![0; n as usize]; m as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 || j == 0 {
                    // 只能向右或者向下
                    dp[i][j] = 1;
                    continue;
                }

                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        return dp[m as usize - 1][n as usize - 1];
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
    fn test_unique_paths() {
        //assert_eq!(Solution::unique_paths(vec![]),[]);
        assert!(true)
    }
}

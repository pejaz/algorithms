/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            // 起始位置有障碍，那么往后没法走
            return 0;
        }

        // 1. 定义dp 数组
        // 2. 递推公式：dp[m][n] = obstacle_grid[m][n] == 1 ? 0 : dp[m-1][n] + dp[m][n-1]
        let row = obstacle_grid.len();
        let col = obstacle_grid[0].len();
        let mut dp = vec![vec![0; col]; row];
        dp[0][0] = 1; // 这里已经排除obstacle_grid[0][0]有障碍

        // 3.初始化第一行和第一列，如果碰到障碍了，那么后续都要变为 0
        for m in 1..row {
            // 初始化第一列。如果当前或者上方有障碍那么往后都不能抵达
            dp[m][0] = if obstacle_grid[m][0] == 1 || dp[m - 1][0] == 0 {
                0
            } else {
                1
            }
        }
        for n in 1..col {
            // 初始化第一行。如果当前或者左边有障碍那么往后都不能抵达
            dp[0][n] = if obstacle_grid[0][n] == 1 || dp[0][n - 1] == 0 {
                0
            } else {
                1
            }
        }

        // 遍历 dp
        for m in 1..row {
            for n in 1..col {
                dp[m][n] = if obstacle_grid[m][n] == 1 {
                    0
                } else {
                    dp[m - 1][n] + dp[m][n - 1]
                }
            }
        }

        return dp[row - 1][col - 1];
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
    fn test_unique_paths_with_obstacles() {
        //assert_eq!(Solution::unique_paths_with_obstacles(vec![]),[]);
        assert!(true)
    }
}

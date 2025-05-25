/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 2 {
            return cost[0].min(cost[1]);
        }

        // dp[n]: 爬到下标 n 的最小花费
        // 转移：dp[n] = min(dp[n-1]+cost[n-1],dp[n-2]+cost[n-2])
        let mut dp = vec![0; cost.len() + 1];
        //0 或 1 开始
        // dp[0] = 0;
        // dp[1] = 0;

        for i in 2..=cost.len() {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }

        return dp[cost.len()];
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
    fn test_min_cost_climbing_stairs() {
        //assert_eq!(Solution::min_cost_climbing_stairs(vec![]),[]);
        assert!(true)
    }
}

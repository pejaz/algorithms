/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 买卖股票的最佳时机含冷冻期
 */

// @lc code=start
/**
 * 动态规划解题思路：
 * 1. dp 定义：
 *  dp[i][0]：第 i 天 持有股票的最高金额
 *  dp[i][1]：第 i 天 不持有股票的最高金额
 * 2. 递归公式：
 *  持有：
 *    冷冻期为 k 天，则前 k 天只能买入一次
 *      dp[k][0] = max(dp[i-1][0], -prices[k])
 *    超过 k 天后，继承前面持有或者第 i - k 天卖出时状态买入（k 为冷冻期）
 *      dp[i][0] = max(dp[i-1][0], dp[i-2][1] - prices[i]);
 *  不持有：继承前面卖出或者第 i 天卖出
 *      dp[i][1] = max(dp[i-1][1], dp[i-1][0] + prices[i]);
 *  第 i 天最大利润：
 *      profit = dp[i][1](不持有股票的现金一定比持有股票的现金多)
 * 3. 初始化：
 *    dp[0][0] = -prices[0];
 *    dp[0][1] = 0;
 * 4. 遍历顺序：
 *  从递归公式可知，dp[i]的状态依赖 i 之前状态，所以下标 1 开始从前往后遍历。
 */
#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        // 1. 定义DP
        let mut dp = vec![[0; 2]; prices.len()];

        // 2. 初始化 DP
        dp[0][0] = -prices[0];

        for i in 1..prices.len() {
            if i == 1 {
                // 第一天持有：前一天持有或者今天第一次买入
                // 不能从前一天不持有状态流转，因为有冷冻期
                dp[i][0] = dp[i - 1][0].max(-prices[i]);
            } else {
                dp[i][0] = dp[i - 1][0].max(dp[i - 2][1] - prices[i]);
            }
            // 不持有
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + prices[i]);
        }

        return dp[prices.len() - 1][1];
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
    fn test_max_profit() {
        //assert_eq!(Solution::max_profit(vec![]),[]);
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 */

// @lc code=start
/**
 * 动态规划解题思路：
 * 1. dp 定义：
 *  dp[i][0]：第 i 天第 1 次 持有股票的最高金额
 *  dp[i][1]：第 i 天第 2 次 持有股票的最高金额
 *  dp[i][2]：第 i 天 第 1 次不持有股票的最高金额
 *  dp[i][3]：第 i 天 第 2 次不持有股票的最高金额
 * 2. 递归公式：
 *  第一次持有：继承前面第一次持有或者第 i 天第一次买入
 *      dp[i][0] = max(dp[i-1][0], -prices[i]);
 *  第二次持有：从前面继承
 *      dp[i][1] = max(dp[i-1][1], dp[i-1][2] - prices[i]);
 *  第一次不持有：继承前面第一次卖出或者第 i 天第一次卖出：
 *      dp[i][2] = max(dp[i-1][2], dp[i-1][0] + prices[i]);
 *  第二次不持有：继承前面第二次卖出或者第 i 天第二次卖出：
 *      dp[i][3] = max(dp[i-1][3], dp[i-1][1] + prices[i]);
 *  第 i 天最大利润：
 *      profit = max(dp[i][2],dp[i][3])(不持有股票的现金一定比持有股票的现金多，这里第二次卖出已经包含第一次卖出了，也可以直接输出dp[i][3])
 * 3. 初始化：
 *    dp[0][0] = -prices[0];
 *    dp[0][1] = 0;
 *    dp[0][2] = 0;
 *    dp[0][3] = 0;
 * 4. 遍历顺序：
 *  从递归公式可知，dp[i]的状态依赖 dp[i-1]，所以下标 1 开始从前往后遍历。
 */
#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        // 定义 DP (第一次持有，第二次持有，第一次不持有，第二次不持有)
        let mut dp = vec![(0, 0, 0, 0); prices.len()];

        // 初始化 dp: 第一次和第二次不持有可以理解为当天买当天卖，所以此时金额还是 0
        dp[0].0 = -prices[0];
        dp[0].1 = -prices[0]; // 注意：因为可以当天买卖，所以这里可以理解为第一天买卖然后又第二次买入

        for i in 1..prices.len() {
            dp[i].0 = dp[i - 1].0.max(-prices[i]);
            dp[i].1 = dp[i - 1].1.max(dp[i - 1].2 - prices[i]);
            dp[i].2 = dp[i - 1].2.max(dp[i - 1].0 + prices[i]);
            dp[i].3 = dp[i - 1].3.max(dp[i - 1].1 + prices[i]);
        }

        return dp[prices.len() - 1].2.max(dp[prices.len() - 1].3);
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
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert!(true)
    }
}

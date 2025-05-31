/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
/**
 * 动态规划解题思路：
 * 1. dp 定义：
 *  dp[i][0]：第 i 天持有股票的最高金额
 *  dp[i][1]：第 i 天不持有股票的最高金额
 * 2. 递归公式：
 *  从前面持有股票继承过来，或者第 i 天买入：
 *    dp[i][0] = max(dp[i-1][0], -prices[i])；
 *  前面就已经卖出，或者第 i 天卖出：
 *    dp[i][1] = max(dp[i-1][1], dp[i-1][0] + prices[i])
 *  第 i 天最大利润：profit = dp[i][1](不持有股票的现金一定比持有股票的现金多)
 * 3. 初始化：
 *    dp[0][0] = -prices[0];
 *    dp[0][1] = 0;
 * 4. 遍历顺序：
 *  从递归公式可知，dp[i]的状态依赖 dp[i-1]，所以下标 1 开始从前往后遍历。
 *
 */
#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = 0;

        for i in 0..prices.len() - 1 {
            let diff = prices[i + 1] - prices[i];

            sum += diff;
            if sum < 0 {
                sum = 0;
            }

            res = res.max(sum);
        }

        return res;
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

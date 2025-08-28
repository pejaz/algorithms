/* 🔖
 * @lc app=leetcode.cn id=188 lang=rust
 *
 * [188] 买卖股票的最佳时机 IV
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
 *    dp[0][1] = -prices[0] ;
 *    dp[0][2] = 0;
 *    dp[0][3] = 0;
 * 4. 遍历顺序：
 *  从递归公式可知，dp[i]的状态依赖 dp[i-1]，所以下标 1 开始从前往后遍历。
 */
#[allow(unused)]
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        // 1. 定义 DP 数组：dp[i][0..k]:第 1..k 次持有股票的金额，dp[i][k..2k]:第 1..k 次不持有股票的最高金额。
        let mut dp = vec![vec![0; 2 * k as usize]; prices.len()];
        // 2. 初始化第一天时持有状态的金额
        dp[0][0..k as usize].fill(-prices[0]);

        for i in 1..prices.len() {
            for j in 0..k as usize {
                // 第 i 天第 j 次持有: max(i-1 天持有, i-1 天第 j-1 次不持有 - prices[i])
                // 第 i 天第 j 次不持有：max(i-1天不持有, i-1 天第 j 次持有 + prices[i])
                let k = k as usize;
                // 第 j 次持有
                if j == 0 {
                    // 前面 i -1 天就第一次持有，或者今天第一次买入
                    dp[i][j] = dp[i - 1][j].max(-prices[i]);
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - 1 + k] - prices[i]);
                }
                // 第 j 次不持有
                dp[i][j + k] = dp[i - 1][j + k].max(dp[i - 1][j] + prices[i]);
            }
        }

        // 最后一次不持有
        return dp[prices.len() - 1][2 * k as usize - 1];
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

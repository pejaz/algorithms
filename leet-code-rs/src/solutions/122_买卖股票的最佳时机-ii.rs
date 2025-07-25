/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */

// @lc code=start
/**
 * 动态规划解题思路：
 * 1. dp 定义：
 *  dp[i][0]：第 i 天持有股票的最高金额
 *  dp[i][1]：第 i 天不持有股票的最高金额
 * 2. 递归公式：
 *  从前面持有股票继承过来，或者第 i 天买入：
 *    dp[i][0] = max(dp[i-1][0], dp[i-1][1]-prices[i])；
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
        // 股票买卖收益
        let mut res = 0;
        let end = prices.len() - 1;

        for i in 0..end {
            let diff = prices[i + 1] - prices[i];

            if diff > 0 {
                res += diff;
            }
        }

        // let mut diff = vec![0; end];
        // for i in 0..end {
        //     diff[i] = prices[i + 1] - prices[i];
        // }

        // for n in diff {
        //     // 收益为正的累加
        //     if n > 0 {
        //         res += n;
        //     }
        // }

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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=518 lang=rust
 *
 * [518] 零钱兑换 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  硬币可以无限次，属于完全背包问题。
     *  因为是装满背包的方式，所以递推公式为：dp[j] += dp[j-weight[i]]
     */
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // 定义dp 数组
        let mut dp = vec![0; amount as usize + 1];
        // 初始化 dp[0] 0 个硬币凑成 0 金额的方式有 1 种
        dp[0] = 1;

        // 遍历物品
        for i in 0..coins.len() {
            let coin = coins[i] as usize;
            // 遍历背包，因为物品不限次放入。所以从小到大
            for j in coin..=amount as usize {
                dp[j] += dp[j - coin];
            }
        }

        return dp[amount as usize];
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
    fn test_change() {
        //assert_eq!(Solution::change(vec![]),[]);
        assert!(true)
    }
}

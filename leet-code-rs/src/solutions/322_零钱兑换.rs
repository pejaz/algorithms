/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  物品数量无限，属于完全背包问题。求最少数量，不是求凑成方法数，属于完全背包基础类型
     *  完全背包基础递推公式：dp[j] = max(dp[j], dp[j-coins[i]] + 1);
     *  本题求最少数量，dp[j] 代表 装满 j 的背包的最小数量。
     *  所以递推公式为： dp[j] = min(dp[j], dp[j-coins[i]] + 1);
     */
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // 本题求最少次数，所以不能用 0 来初始化，否则都会比 0 小，用 i32::MAX 时 dp[j-coins[i]] + 1会溢出。
        // 所以用  amount ，即最坏情况就是用 1 元硬币凑成 amount,此时也就 amount次，不会超过 amount +1。
        let mut dp = vec![amount + 1; amount as usize + 1];
        // 初始化 dp[0]: 0 个银币构成 0 的最少数量也为 0，即dp[0] = 0;
        dp[0] = 0;
        // 先遍历物品
        for i in 0..coins.len() {
            let coin = coins[i] as usize;
            // 遍历背包，因为是完全背包，所以从小到大遍历。
            for j in coin..=amount as usize {
                dp[j] = dp[j].min(dp[j - coin] + 1)
            }
        }

        // 本题不能凑成金额需要返回-1
        // 如果 dp[amount] 还是初始值，说明不能凑满，返回 -1
        return if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        };
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
    fn test_coin_change() {
        //assert_eq!(Solution::coin_change(vec![]),[]);
        assert!(true)
    }
}

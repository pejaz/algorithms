/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 解题思路：和零钱兑换一样。
    //  1. 本题物品是小于 n 的完全平方数，需要自己计算。可以多次相加，所以是完全背包
    //  完全背包基础递推公式： dp[j] = max(dp[j],dp[j-i] + 1)
    //  求最少数量而非方法数，所以是完全背包基础类型
    //  本题递推公式：dp[j] = min(dp[j], dp[j-1] + 1)
    //  2. 求最小需要注意的是dp 数组的初始化和 dp[0]初始化。
    //  因为递推公式是求 min，所以 dp 数组初始化不能是 0，同时使用 i32::MAX时避免 dp[j-1] + 1 数组溢出，所以也不使用。
    //  考虑最后的情况是有 1 组成 n,也就是 n 次，所以我们初始化为 n + 1 即可，这样dp[j]状态自然会覆盖 。
    //  dp[0] 代表由 没有数来组成 0 ，也就是 0 次，即dp[0] = 0
    //  最后因为 n 一定能组成（最差 n 个 1 一定能组成 n），所以不用考虑组不成返回 -1 的情况
    pub fn num_squares(n: i32) -> i32 {
        // 1. dp[j]含义：组成 j 的最少数量
        // 2. 递推公式dp[j] = dp[j].min(dp[j - i] + 1)
        // 3. 初始化 dp 数组 和 dp[0]
        let mut dp = vec![n + 1; n as usize + 1];
        dp[0] = 0;
        // 4. 遍历物品
        let mut i: usize = 1;
        while i.pow(2) <= n as usize {
            let coin = i.pow(2);
            // 遍历背包
            for j in coin..=n as usize {
                dp[j] = dp[j].min(dp[j - coin] + 1);
            }

            i += 1;
        }

        return dp[n as usize];
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
    fn test_num_squares() {
        //assert_eq!(Solution::num_squares(vec![]),[]);
        assert!(true)
    }
}

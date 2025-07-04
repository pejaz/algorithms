/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路： 2，1，1，2
     *  1. DP 数组的含义： dp[i] 代表到 下标 i 的位置所能投的最大金额,(不偷最大值，偷最大值)
     *  2. 递推公式：如果
     *      dp[i] 不偷，则考虑 i-1 最大值就（即 dp[i-1]状态），i-1 不一定偷，可能偷 i-2： dp[i] = max(dp[i-1](不偷) ,dp[i-1] (偷));
     *      dp[i] 偷，dp[i] = dp[i-1](不偷) + money[i]
     *      最终 dp[i] 取值这两种的最大值。
     *  3. 初始化，依赖i-1 状态 dp[0] 和 dp[1]
     *  4. 遍历顺序：从小到达
     */
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 1. 定义 DP，（没偷，偷了）
        let mut dp = vec![(0, 0); nums.len()];
        // 3. 初始化 DP，（没偷，偷了）
        dp[0] = (0, nums[0]);

        for i in 1..nums.len() {
            // 避免 i-2 下标为负数
            if i == 1 {
                // 没偷 1，则偷 0；偷 1
                dp[i] = (dp[0].1, nums[i]);
                continue;
            }

            // 没偷 i,则i-1 不一定偷; 偷 i，则不偷 n-1
            dp[i] = (dp[i - 1].1.max(dp[i-1].0), dp[i - 1].0 + nums[i]);
        }

        return dp[nums.len() - 1].0.max(dp[nums.len() - 1].1);
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
    fn test_rob() {
        
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
        assert_eq!(Solution::rob(vec![2, 3, 2]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 1, 2, 5]), 7);
        assert!(true)
    }
}

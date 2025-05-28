/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  1. DP 数组的含义： dp[i] 代表到 下标 i 的位置所能偷的最大金额
     *  2. 递推公式：如果
     *      i-1 偷了，那么 dp[i]不能偷，所以 dp[i] = dp[i-1];
     *      i-1 没偷，那么 dp[i]可以偷，所以 dp[i] = dp[i-2] + money[i]
     *      最终 dp[i] 取值这两种的最大值。
     *  3. 初始化，依赖i-1 状态 dp[0] 和 dp[1]
     *  4. 遍历顺序：从小到达
     */
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 1. 定义 DP
        let mut dp = vec![0; nums.len()];
        // 3. 初始化 DP
        dp[0] = nums[0];

        for i in 1..nums.len() {
            // 避免 i-2 下标为负数
            if i == 1 {
                // 0 偷了，则 1 不能偷，0 没偷，则 1 偷
                dp[i] = dp[0].max(nums[i]);
                continue;
            }

            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }

        return dp[nums.len() - 1];
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
        //assert_eq!(Solution::rob(vec![]),[]);
        assert!(true)
    }
}

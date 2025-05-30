/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 解题思路：
    //  要么偷第一家不偷最后一家（只遍历到 len -2） 。
    //  要么不偷第一家（从1 开始遍历）偷最后一家。对比最大值
    //  dp[i] 代表考虑到 i 所能偷的最大金额
    //  递推公式： dp[i] = max(dp[i-1], dp[i-2]+nums[i])
    //
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        fn call_rob(nums: &[i32]) -> i32 {
            // 不成环的情况
            let mut dp = vec![0; nums.len()];
            dp[0] = nums[0];
            for i in 1..nums.len() {
                if i == 1 {
                    dp[i] = nums[0].max(nums[i]);
                    continue;
                } else {
                    dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
                }
            }

            return dp[nums.len() - 1];
        }

        // 成环：排除第一家
        let omit_first = call_rob(&nums[1..]);
        // 成环：排除最后一家
        let omit_last = call_rob(&nums[0..nums.len() - 1]);

        return omit_first.max(omit_last);
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
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 1, 1]), 3);
        assert!(true)
    }
}

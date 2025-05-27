/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  将 nums 分为加法一个集合left，减法一个集合right，
     *  假设加法总和为left，则 right = sum - left ，又left - right = target
     *  所以消去 right 得 left = (sum + target) / 2
     *  此时转化为 求 nums 中 和为 left 有多少种方法。转为 01 背包求排列组合问题
     *  背包重量为 left，物品i重量为nums[i]
     */
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        // 所有数加起来也比目标小，无解
        if sum < target.abs() {
            return 0;
        }

        if (sum + target) % 2 == 1 {
            // sum + target 为奇数， 此时除以 2 有小数
            // 整数相加不可能有小数，无解
            return 0;
        }

        let bagsize = (sum + target) as usize / 2;
        // 递推公式： dp[j] = dp[j] + dp[j-weight[i]]
        let mut dp = vec![0; bagsize + 1];
        // 初始化 dp[0]，没有任何物品刚好重量为 0，为一种1情况
        dp[0] = 1;
        // 遍历物品
        for i in 0..nums.len() {
            // 倒序遍历背包
            let num = nums[i] as usize;
            for j in (num..=bagsize).rev() {
                dp[j] += dp[j - num];
            }
        }

        return dp[bagsize];
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
    fn test_find_target_sum_ways() {
        //assert_eq!(Solution::find_target_sum_ways(vec![]),[]);
        assert!(true)
    }
}

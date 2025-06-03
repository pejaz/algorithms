/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 动态规划解题思路：
     *  本题子数组和是枚举哪一个子元素的问题。
     *  1. dp[i]: 以元素 i结尾的连续子数组最大和
     *  2. 递推公式：是否累加 i 前面的元素
     *      dp[i] = dp[i-1] < 0 ? nums[i] : dp[i-1]+nums[i]
     *  3. 初始化 dp[0] = nums[0]
     */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut dp = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            // 判断是否要累加前面的最大值
            dp[i + 1] = nums[i].max(dp[i] + nums[i]);
            res = res.max(dp[i + 1])
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
    fn test_max_sub_array() {
        //assert_eq!(Solution::max_sub_array(vec![]),[]);
        assert!(true)
    }
}

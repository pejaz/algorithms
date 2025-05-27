/*
 * @lc app=leetcode.cn id=377 lang=rust
 *
 * [377] 组合总和 Ⅳ
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  完全背包求解最大排列数（非组合）
     */
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        // 遍历顺序，因为是求排列，所以要先遍历背包，确保物品顺序被打乱，同时因为是完全背包，所以要从小到大遍历。
        for j in 0..=target as usize {
            // 后遍历物品
            for i in 0..nums.len() {
                if j >= nums[i] as usize {
                    dp[j] += dp[j - nums[i] as usize];
                }
            }
        }

        return dp[target as usize];
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
    fn test_combination_sum4() {
        //assert_eq!(Solution::combination_sum4(vec![]),[]);
        assert!(true)
    }
}

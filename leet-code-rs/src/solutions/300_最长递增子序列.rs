/* 🔖
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  1. dp[i]含义：代表nums[i]为结尾的最长递增子序列长度
     *  2. 递推公式：j 为 [0,i-1] 区间的所有数字
     *         dp[i] = nums[i] > nums[j] ? dp[j] + 1
     *  3. 初始化，dp[i] = 1，任何nums[i]至少自身长度为 1
     *  4. 遍历顺序：正序遍历
     */
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 初始化 dp[0]: 自身也是子序列。dp[i]= 1
        let mut dp = vec![1; nums.len()];
        // 结尾最长子序列并不是在 dp 最后一个元素,而是任何一个 nums[i]为结尾都可能是最长
        // 例如: 4,10,2,11,3,5 以 11 为结尾或者5 为结尾都可以构成最长子序列
        let mut res = 1;

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }

            res = res.max(dp[i]);
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
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![4, 10, 2, 3, 4]), 3);
        assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
        assert!(true)
    }
}

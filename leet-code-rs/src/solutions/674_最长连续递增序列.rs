/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  1. dp[i]含义：代表nums[i]为结尾的最长连续递增子序列长度
     *  2. 递推公式：
     *     - 因为本题要求连续，所以不需要考虑 [0,i-1]区间的问题，只需要考虑 i-1 的状态即可
     *     - dp[i] = nums[i] > nums[i-1] ? dp[i-1] + 1 : dp[i]
     *  3. 初始化，dp[i] = 1，任何nums[i]至少自身长度为 1
     *  4. 遍历顺序：正序遍历
     */
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut res = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                dp[i] = dp[i - 1] + 1;
                res = res.max(dp[i]);
            }
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
    fn test_find_length_of_lcis() {
        //assert_eq!(Solution::find_length_of_lcis(vec![]),[]);
        assert!(true)
    }
}

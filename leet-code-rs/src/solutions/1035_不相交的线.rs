/*
 * @lc app=leetcode.cn id=1035 lang=rust
 *
 * [1035] 不相交的线
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  LCS：选或不选的问题。
     *  1. dp[i][j]含义：代表前 i 个元素和前 j个元素的最长公共子序列长度
     *  2. 递推公式：
     *      选：nums1[i] == nums2[j]: dp[i+1][j+1] = dp[i][j] + 1;
     *      不选：nums1[i] != nums2[j]: dp[i+1][j+1] = max(dp[i][j+1],dp[i+1][j]);
     *  3. 下标偏移 1，可以不同特殊处理 dp[0] 的情况
     *  4. 区间含义，最优解在最后一个元素
     */
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                dp[i + 1][j + 1] = if nums1[i] == nums2[j] {
                    dp[i][j] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }

        return dp[nums1.len()][nums2.len()];
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
    fn test_max_uncrossed_lines() {
        //assert_eq!(Solution::max_uncrossed_lines(vec![]),[]);
        assert!(true)
    }
}

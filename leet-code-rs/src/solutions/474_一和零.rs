/*
 * @lc app=leetcode.cn id=474 lang=rust
 *
 * [474] 一和零
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：
     *  strs 代表物品，物品重量为 i 个 0，j 个 1，物品价值都为 1，
     *  背包重量为 m 个 0，n 个 1，装满背包的最大价值即为最大子集个数。因为物品价值都为 1.
     *  dp[j] = max(dp[j],dp[j-weight[i]]+value[i])
     */
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // dp[m][n] = max(dp[m][n], dp[m - i][n - j] + 1)
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        // 初始化dp[0][0]， 即背包 m 为 0，n为 0，那么放不下任何物品，即dp[0][0] = 0

        // 遍历物品
        for i in 0..strs.len() {
            // 计算物体重量。
            let str = &strs[i];
            let zeros = str.chars().filter(|&c| c == '0').count();
            let ones = str.chars().filter(|&c| c == '1').count();
            for j in (zeros..=m as usize).rev() {
                for k in (ones..=n as usize).rev() {
                    dp[j][k] = dp[j][k].max(dp[j - zeros][k - ones] + 1);
                }
            }
        }

        return dp[m as usize][n as usize];
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
    fn test_find_max_form() {
        //assert_eq!(Solution::find_max_form(vec![]),[]);
        assert!(true)
    }
}

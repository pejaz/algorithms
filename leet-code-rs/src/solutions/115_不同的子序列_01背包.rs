/*
 * @lc app=leetcode.cn id=115 lang=rust
 *
 * [115] 不同的子序列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路：本题是选和不选问题
     *  1. dp[i][j]: s 前 i 个元素和出现 t 前 j 个元素的个数
     *  2. 递推公式：
     *     - s[i] == t[i]: dp[i+1][j+1] = dp[i][j+1] + dp[i][j];
     *     - s[i] != t[i]: dp[i+1][j+1] = dp[i][j+1]（t 不能删）
     *  3. 初始化：根据 dp 含义：dp[i][0] = 1
     *
     * 思路2：现在有7个物品“babgbag", 依次放进size为3的背包"bag"中，
     *  但放进背包的条件是只有字符相等才能放，可以选择放与不放，求放满背包的方法数
     */
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        //  dp[j] 含义：装满背包 bagweight 为 j 的方法数
        let mut dp = vec![0; t.len() + 1];
        dp[0] = 1; // 装满背包为 0 的方法为一种
        // 遍历物品
        for i in 0..s.len() {
            // 倒入背包，确保物品只被放入一次
            for j in (1..=t.len()).rev() {
                // 可以放入背包
                if s[i] == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        return dp[t.len()];
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
    fn test_num_distinct() {
        //assert_eq!(Solution::num_distinct(vec![]),[]);
        assert!(true)
    }
}

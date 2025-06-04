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
     *     - s[i] == t[i]: dp[i+1][j+1] = dp[i][j+1]（只删除 i ） + dp[i][j](同时删除 i 和 j);
     *     - s[i] != t[i]: dp[i+1][j+1] = dp[i][j+1]（t 不能删）
     *  3. 初始化：根据 dp 含义：dp[i][0] = 1
     *
     * 思路2：现在有7个物品“babgbag", 依次放进size为3的背包"bag"中，
     *  但放进背包的条件是只有字符相等才能放，可以选择放与不放，求放满背包的方法数
     */
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
        // 初始化 dp[i][0]：s i-1下标出现 t -1 空串次数为 1；
        for i in 0..=s.len() {
            dp[i][0] = 1;
        }

        for i in 0..s.len() {
            for j in 0..t.len() {
                dp[i + 1][j + 1] = if s[i] == t[j] {
                    dp[i][j + 1] + dp[i][j]
                } else {
                    dp[i][j + 1]
                }
            }
        }

        return dp[s.len()][t.len()];
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;
    //  [  r  a  b  i
    // r [ 1, 0, 0, 0],
    // a [ 1, 1, 0, 0],
    // b [ 1, 1, 1, 0],
    // b [ 1, 1, 2, 0],
    // b [ 1, 1, 3, 0],
    // i [ 1, 1, 3, 3],
    // t [ 1, 1, 3, 3]
    // ]
    #[test]
    fn test_num_distinct() {
        //assert_eq!(Solution::num_distinct(vec![]),[]);
        assert!(true)
    }
}

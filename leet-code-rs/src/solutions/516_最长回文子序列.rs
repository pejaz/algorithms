/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 * [516] 最长回文子序列
 */

// @lc code=start
#[allow(unused)]
impl Solution {
    /**
     * 解题思路：选与不选问题 
     *  1. dp[i][j] ：以 i 为起点 j 为终点的最长回文子序列
     *  2. 递推公式：
     *     - s[i] == s[j]:
     *      - i == j: dp[i][j] = 1
     *      - i != j: dp[i][j] = dp[i+1][j-1] + 2
     *    - s[i] != s[j]:
     *      dp[i][j] = dp[i+1][j].max(dp[i][j-1])
     *  3. 初始化：i 从大到小，j 从小到大
     */
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; s.len()]; s.len()];

        // 终点 j
        for j in 0..s.len() {
            // 起点 i
            for i in (0..=j).rev() {
                if s[i] == s[j] {
                    if j - i <= 1 {
                        dp[i][j] = j - i + 1;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1] + 2;
                    }
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1])
                }
            }
        }

        return dp[0][s.len() - 1] as i32;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    // dbasabc
    // bbbab
    //  j
    // [1, 1, 1, 1], dp[0]
    // [0, 1, 2, 2],
    // [0, 0, 1, 1],
    // [0, 0, 0, 1]
    #[test]
    fn test_longest_palindrome_subseq() {
        assert_eq!(
            Solution::longest_palindrome_subseq(String::from("bbbab")),
            4
        );
        assert_eq!(Solution::longest_palindrome_subseq(String::from("cbbd")), 2);
        assert_eq!(
            Solution::longest_palindrome_subseq(String::from("dbasabc")),
            5
        );
        assert!(true)
    }
}

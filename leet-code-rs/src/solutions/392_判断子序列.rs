/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    // 动态规划思路（非最佳）：如果 s 为 t的子序列，那么 s 和 t 的最长公共子序列一定为 s。
    //    可以用求最长公共子序列(不考虑 s 删除元素，即 s[i] != t[i] 时，dp[i][j] = dp[i][j-1])
    //    然后判断最后结果 是否等于 s 长度即可。
    // 贪心思路：和爬楼梯类似
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        if s.is_empty() {
            return true;
        }

        let mut idx = 0;
        let s_len = s.len();

        for c in t {
            if c == s[idx] {
                idx += 1;
                if idx == s_len {
                    return true;
                }
            }
        }

        return false;
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
    fn test_is_subsequence() {
        //assert_eq!(Solution::is_subsequence(vec![]),[]);
        assert!(true)
    }
}

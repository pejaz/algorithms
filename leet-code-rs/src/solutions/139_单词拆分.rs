/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    /**
     * 解题思路:
     *  单词可以重复使用，属于完全背包问题。
     *  因为本题对物品顺序有要求['leet','code']能组成,['code','leet']则不行，所以是完全背包排列型
     *  可以转为爬楼梯思维：我们要爬 s.len() 层楼梯，每次只能走 wordDict 步，判断能否走到楼顶。
     *  此时递推公式为： dp[j] = dp[j-i] 也就是dp[j-i] 要能抵达的前提下，再走 i 步到达楼顶，i 要在 wordDict中
     *  即 j-i 区间字串在 wordDict 中
     */
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        // 1. 楼梯
        for j in 0..=s.len() {
            // 2. 步数
            for i in 0..word_dict.len() {
                let str = &word_dict[i];
                // 在dp[j-str]为 true（即能抵达的前提），再走 str 步能到达
                if j >= str.len() && s[j - str.len()..j] == *str && dp[j - str.len()] == true {
                    dp[j] = true;
                }
            }
        }

        return dp[s.len()] == true;
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
    fn test_word_break() {
        assert_eq!(
            Solution::word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            ),
            false
        );
        assert!(true)
    }
}

/*
 * @lc app=leetcode.cn id=1047 lang=rust
 *
 * [1047] 删除字符串中的所有相邻重复项
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut st = vec![];
        for c in s.chars() {
            if !st.is_empty() && *st.last().unwrap() == c {
                st.pop();
            } else {
                st.push(c);
            }
        }

        st.iter().collect()
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
    fn test_remove_duplicates() {
        //assert_eq!(Solution::remove_duplicates(vec![]),[]);
        assert!(true)
    }
}

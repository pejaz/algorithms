/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn backtracking(
            s: &String,
            idx: usize,
            path: &mut Vec<String>,
            res: &mut Vec<Vec<String>>,
        ) {
            if idx == s.len() {
                res.push(path.clone());
                return;
            }

            for i in idx..s.len() {
                let str = &s[idx..=i];
                if str.chars().eq(str.chars().rev()) {
                    path.push(str.to_string());
                    backtracking(s, i + 1, path, res);
                    path.pop();
                }
            }
        }

        let mut path = vec![];
        let mut res = vec![];

        backtracking(&s, 0, &mut path, &mut res);

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
    fn test_partition() {
        //assert_eq!(Solution::partition(vec![]),[]);
        assert!(true)
    }
}

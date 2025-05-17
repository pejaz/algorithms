/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
use std::collections::HashMap;
#[allow(unused)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        // 树的深度为 digits 的数字个数，树的宽度为每一个数字对应的字母数
        fn backtracking(
            mp: &HashMap<char, &str>,
            digits: &Vec<char>,
            i: usize, // idx 代表当前遍历到的 digits 下标
            path: &mut String,
            res: &mut Vec<String>,
        ) {
            // 递归深度，为输入 digits 的 len()，也可以用 i == digits.len()判断
            if path.len() == digits.len() {
                res.push(path.clone());
                return;
            }

            let str = *mp.get(&digits[i]).unwrap();
            // for 循环代表树的宽度，为数字对应的 str 数
            for s in str.chars() {
                &path.push(s);
                backtracking(mp, digits, i + 1, path, res);
                &path.pop();
            }
        }
        let mp = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);
        let mut path = String::new();
        let mut res = vec![];
        let digits = digits.chars().collect();
        backtracking(&mp, &digits, 0, &mut path, &mut res);

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
    fn test_letter_combinations() {
        println!("{:?}", Solution::letter_combinations(String::from("23")));
        println!("{:?}", Solution::letter_combinations(String::from("2")));
        println!("{:?}", Solution::letter_combinations(String::from("234")));
        // assert_eq!(Solution::letter_combinations(vec![]),[]);
        assert!(true)
    }
}

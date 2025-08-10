/*
 * @lc app=leetcode.cn id=738 lang=rust
 *
 * [738] 单调递增的数字
 */

// @lc code=start

#[allow(unused)]
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut str = n.to_string().chars().collect::<Vec<char>>();

        // 从右到左匹配，如果发现左边比右边大，则左边数字-1，并把后面位开始位置 变为 9。否则保持原来数字

        let mut idx = None;
        for i in (0..str.len() - 1).rev() {
            if str[i] > str[i + 1] {
                // 不是递增
                str[i] = (str[i] as u8 - 1) as char;
                // idx 往后填充 9
                idx = Some(i + 1)
            }
        }

        if let Some(idx) = idx {
            str[idx..].fill('9');
        }

        return str.into_iter().collect::<String>().parse().unwrap();
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
    fn test_monotone_increasing_digits() {
        //assert_eq!(Solution::monotone_increasing_digits(vec![]),[]);
        assert!(true)
    }
}

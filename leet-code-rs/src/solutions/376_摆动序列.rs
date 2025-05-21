/*
 * @lc app=leetcode.cn id=376 lang=rust
 *
 * [376] 摆动序列
 */

// @lc code=start

use std::cmp::Ordering::{Equal, Greater, Less};

#[allow(unused)]
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut res = 1;
        // large 为 true 代表下一个数应该大于前一个数
        let mut large = None;
        let mut last = nums[0];

        for i in 1..nums.len() {
            match nums[i].cmp(&last) {
                Less => {
                    if large != Some(true) {
                        res += 1;
                        large = Some(true);
                    }
                }
                Greater => {
                    if large != Some(false) {
                        res += 1;
                        large = Some(false)
                    }
                }
                Equal => {}
            }

            last = nums[i]
        }

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
    fn test_wiggle_max_length() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
        assert!(true)
    }
}

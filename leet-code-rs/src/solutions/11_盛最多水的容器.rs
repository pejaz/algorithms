/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */
// @lc code=start
use std::cmp::{max, min};
#[allow(unused)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            let area = (r - l) * min(height[l] as usize, height[r] as usize);
            res = max(area, res);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        return res as i32;
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
    fn test_max_area() {
        //assert_eq!(Solution::max_area(vec![]),[]);
        assert!(true)
    }
}

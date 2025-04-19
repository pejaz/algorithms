/*
 * @lc app=leetcode.cn id=454 lang=rust
 *
 * [454] 四数相加 II
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut cnt = HashMap::new();
        for n1 in nums1 {
            for n2 in &nums2 {
                *cnt.entry(n1 + n2).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for n3 in nums3 {
            for n4 in &nums4 {
                let sum = -(n3 + n4);
                ans += cnt.get(&sum).unwrap_or(&0);
            }
        }

        return ans;
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_four_sum_count() {
        assert_eq!(
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
        assert_eq!(
            Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
            1
        );
        assert!(true)
    }
}

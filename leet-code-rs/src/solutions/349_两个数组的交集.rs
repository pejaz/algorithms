/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // nums1
        //     .into_iter()
        //     .collect::<HashSet<i32>>()
        //     .intersection(&nums2.into_iter().collect::<HashSet<i32>>())
        //     .map(|&x| x)
        //     .collect()
        let mut st = nums1.into_iter().collect::<HashSet<_>>();
        let mut ans = vec![];

        for x in nums2 {
            if st.remove(&x) {
                // x 在 st 中
                ans.push(x);
            }
        }
        ans
    }
}
// @lc code=end

#[allow(unused)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection() {
        //assert_eq!(Solution::intersection(vec![]),[]);
        assert!(true)
    }
}
